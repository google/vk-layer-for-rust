# Copyright 2023 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from __future__ import annotations
from enum import Enum, IntEnum
from functools import cache
import logging
import os
from pathlib import Path
import re
import reg
from typing import Callable, ClassVar, NamedTuple, Optional, Generator
from xml.etree import ElementTree
from xml.etree.ElementTree import Element
from dataclasses import dataclass, field
import dataclasses
import datetime
import unittest


class VulkanAliases:
    """
    This class is essentially a string of union-find data structure, but the representative element
    is picked in a the following priority from the highest to the lowest: core name, KHR suffixed
    name, EXT suffixed name, other suffixed name.
    """

    @dataclass
    class __Node:
        name: str
        parent: Optional[VulkanAliases.__Node] = None

        def root(self) -> VulkanAliases.__Node:
            if self.parent is None:
                return self
            while self.parent.parent is not None:
                self.parent = self.parent.parent
            return self.parent

    def __init__(self) -> None:
        self.__all_nodes: dict[str, VulkanAliases.__Node] = {}

    def add_alias(self, name: str, alias: Optional[str]):
        name_node = self.__all_nodes.setdefault(name, VulkanAliases.__Node(name))

        if alias is None:
            return

        alias_node = self.__all_nodes.setdefault(alias, VulkanAliases.__Node(alias))

        name_root_node = name_node.root()
        alias_root_node = alias_node.root()

        stem = os.path.commonprefix([name, alias])
        name_suffix = name.removeprefix(stem)
        alias_suffix = alias.removeprefix(stem)

        def calculate_rank(suffix: str) -> int:
            suffix_order = [
                "",
                "KHR",
                "EXT",
            ]
            try:
                return suffix_order.index(suffix)
            except ValueError:
                return len(suffix_order)

        name_rank = calculate_rank(name_suffix)
        alias_rank = calculate_rank(alias_suffix)
        (parent_node, child_node) = (name_root_node, alias_root_node)
        if name_rank > alias_rank or (name_rank == alias_rank and name > alias):
            (parent_node, child_node) = (child_node, parent_node)

        assert child_node.parent is None, "Both nodes should be the root node now"
        child_node.parent = parent_node

    def get_represent_name(self, name: str) -> Optional[str]:
        """
        Return the represent name of the input name, e.g. for input `vkWaitSemaphoresKHR`,
        `vkWaitSemaphores` will be returned. If the name is never added, None will be returned.
        """
        node = self.__all_nodes.get(name)
        if node is None:
            return None
        return node.root().name


@dataclass
class UnhandledCommand:
    name: str
    reason: str

    map: ClassVar[dict[str, UnhandledCommand]] = None

    @classmethod
    def find(cls, name: str) -> Optional[UnhandledCommand]:
        if cls.map is None:
            commands_unable_to_handle = {
                "The ash Rust binding doesn't have proper bindings yet.": [
                    "vkMapMemory2KHR",
                    "vkUnmapMemory2KHR",
                    "vkCreateShadersEXT",
                    "vkDestroyShaderEXT",
                    "vkGetShaderBinaryDataEXT",
                    "vkCmdBindShadersEXT",
                    "vkCmdDrawClusterHUAWEI",
                    "vkCmdDrawClusterIndirectHUAWEI",
                    "vkCmdSetAttachmentFeedbackLoopEnableEXT",
                    "vkCmdSetDiscardRectangleEnableEXT",
                    "vkCmdSetExclusiveScissorEnableNV",
                    "vkCmdSetDiscardRectangleModeEXT",
                ],
                # TODO: add a way to customize type generation for the ppBuildRangeInfos parameter
                # of vkCmdBuildAccelerationStructuresKHR. It should be
                # &[&[vk::AccelerationStructureBuildRangeInfoKHR]]. Now it is generated as
                # &[&vk::AccelerationStructureBuildRangeInfoKHR]. Other similar commands include but
                # not limit to vkCmdBuildAccelerationStructuresIndirectKHR.
                "Dynamic multi-dimensional array bindings are not supported yet.": [
                    "vkCmdBuildAccelerationStructuresKHR",
                    "vkBuildAccelerationStructuresKHR",
                    "vkCmdBuildAccelerationStructuresIndirectKHR",
                ],
                # TODO: for these cases, we may need to manually generate them.
                "The length info and the data pointer are nested in structs.": [
                    "vkGetDeviceFaultInfoEXT",
                ],
            }
            cls.map = {}
            for reason, cmd_names in commands_unable_to_handle.items():
                for cmd_name in cmd_names:
                    cls.map[cmd_name] = UnhandledCommand(name=cmd_name, reason=reason)

        return cls.map.get(name)


def camel_case_to_snake_case(input: str) -> str:
    if len(input) <= 1:
        return input.lower()
    # Handling continuous captilized letters like EXT, KHR in the suffix, or IOS, FB, in the middle.
    res = input[0].lower()
    for prev, cur, next in zip(input, input[1:], input[2:]):
        if cur.isupper() and (next.islower() or not prev.isupper()):
            res += f"_{cur.lower()}"
        else:
            res += cur.lower()
    return res + input[-1].lower()


def snake_case_to_upper_camel_case(input: str) -> str:
    if len(input) <= 1:
        return input.upper()
    output = input[0].upper()
    for prev, cur, next in zip(input, input[1:], input[2:]):
        if cur == "_" and next != "_":
            continue
        if prev == "_":
            output += cur.upper()
            continue
        output += cur
    return output + input[-1]


def escape_rust_keywords(id: str) -> str:
    keywords = set(["type"])
    if id not in keywords:
        return id
    return escape_rust_keywords("_" + id)


def write_preamble(file):
    today = datetime.date.today()
    file.write(
        "".join(
            [
                f"// Copyright {today.year} Google LLC\n",
                "//\n",
                '// Licensed under the Apache License, Version 2.0 (the "License");\n',
                "// you may not use this file except in compliance with the License.\n",
                "// You may obtain a copy of the License at\n",
                "//\n",
                "//     http://www.apache.org/licenses/LICENSE-2.0\n",
                "//\n",
                "// Unless required by applicable law or agreed to in writing, software\n",
                '// distributed under the License is distributed on an "AS IS" BASIS,\n',
                "// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n",
                "// See the License for the specific language governing permissions and\n",
                "// limitations under the License.\n",
                "\n",
                "#![allow(missing_docs)]\n",
            ]
        )
    )


opaque_type_map = {
    "void": "c_void",
    # from "xcb/xcb.h"
    "xcb_connection_t": "vk::xcb_connection_t",
    # from "wayland-client.h"
    "wl_display": "vk::wl_display",
    "wl_surface": "vk::wl_surface",
    # from "screen/screen.h"
    "_screen_context": "vk::_screen_context",
    "_screen_window": "vk::_screen_window",
    # from "windows.h"
    "SECURITY_ATTRIBUTES": "vk::SECURITY_ATTRIBUTES",
    "ANativeWindow": "vk::ANativeWindow",
    "AHardwareBuffer": "vk::AHardwareBuffer",
    "CAMetalLayer": "vk::CAMetalLayer",
}

primitive_type_map = {
    "uint64_t": "u64",
    "uint32_t": "u32",
    "uint16_t": "u16",
    "int32_t": "i32",
    "char": "c_char",
    "size_t": "usize",
    "float": "f32",
    "int": "c_int",
}


def decayed_type_to_rust_type(decayed_type: str) -> str:
    manual_type_map = {
        # from "X11/Xlib.h"
        "Display": "vk::Display",
        "VisualID": "vk::VisualID",
        "Window": "vk::Window",
        # from "X11/extensions/Xrandr.h"
        "RROutput": "vk::RROutput",
        # from "windows.h"
        "HINSTANCE": "vk::HINSTANCE",
        "HWND": "vk::HWND",
        "HMONITOR": "vk::HMONITOR",
        "HANDLE": "vk::HANDLE",
        "DWORD": "vk::DWORD",
        "LPCWSTR": "vk::LPCWSTR",
        # from "xcb/xcb.h"
        "xcb_visualid_t": "vk::xcb_visualid_t",
        "xcb_window_t": "vk::xcb_window_t",
        # from "directfb.h"
        "IDirectFB": "vk::IDirectFB",
        "IDirectFBSurface": "vk::IDirectFBSurface",
        # from "zircon/types.h"
        "zx_handle_t": "vk::zx_handle_t",
        # from "ggp_c/vulkan_types.h"
        "GgpStreamDescriptor": "vk::GgpStreamDescriptor",
        "GgpFrameToken": "vk::GgpFrameToken",
    }
    manual_type_map |= opaque_type_map | primitive_type_map
    try_manual_type = manual_type_map.get(decayed_type)
    if try_manual_type is not None:
        return try_manual_type
    if decayed_type.startswith("Vk"):
        possible_suffixes = ["KHR", "EXT"]
        suffix = ""
        for possible_suffix in possible_suffixes:
            if decayed_type.endswith(possible_suffix):
                suffix = possible_suffix
                break
        decayed_type_without_suffix = decayed_type.removesuffix(suffix)
        # ash binding doesn't have the flag bits types.
        if decayed_type_without_suffix.endswith("FlagBits"):
            return f'vk::{decayed_type_without_suffix[2:].removesuffix("FlagBits")}Flags{suffix}'
        return f"vk::{decayed_type[2:]}"
    if decayed_type.startswith("PFN_vk"):
        return f"vk::{decayed_type}"
    logging.fatal(f"Unknown type: {decayed_type}")
    raise RuntimeError(f"Unknown type: {decayed_type}")


class DispatchChainType(IntEnum):
    DEVICE = 1
    INSTANCE = 2
    GLOBAL = 3


class VkXmlLenKind(Enum):
    # TODO: add support for "1" kind
    # TODO: add support for expression type which starts with "latexmath:"
    VARIABLE = 1
    NULL_TERMINATED = 2


@dataclass
class VkXmlType:
    is_const: bool = True
    is_optional: bool = False

    # A pointer type only has the points_to field. Otherwise, the type only has the name field.
    points_to: Optional[VkXmlType] = None
    name: Optional[str] = None
    reg_type_info: Optional[reg.TypeInfo | reg.GroupInfo] = None

    len: Optional[VkXmlLenKind] = None
    dimensions: list[int] = field(default_factory=list)

    def decayed_type(self) -> VkXmlType:
        if self.name is not None:
            return self
        return self.points_to.decayed_type()

    def is_struct(self) -> bool:
        if self.reg_type_info is None:
            return False
        if not isinstance(self.reg_type_info, reg.TypeInfo):
            return False
        return self.reg_type_info.elem.get("category") == "struct"


def get_param_decayed_type(param: Element) -> str:
    type_element = param.find("type")
    return "".join(type_element.itertext()).strip()


class VkXmlParam(NamedTuple):
    type: VkXmlType
    name: str
    len_var: Optional[str] = None

    @staticmethod
    def from_param_element(
        param: Element,
        decayed_type_info: reg.TypeInfo | reg.GroupInfo,
    ) -> VkXmlParam:
        type_element = param.find("type")

        name_element = param.find("name")

        param_name = "".join(name_element.itertext()).strip()
        decayed_type = get_param_decayed_type(param)

        dimensions: list[int] = []
        if name_element.tail is not None and name_element.tail.strip() != "":
            tail = name_element.tail.strip()
            assert re.match(r"^\[\d+?\]$", tail) is not None, f"Unexpected name tail: {tail}"
            dimensions = [int(d) for d in tail.removeprefix("[").removesuffix("]").split("][")]

        optional_attr = param.get("optional")
        is_optionals = []
        if optional_attr is not None:
            for is_optional in [is_optional.strip() for is_optional in optional_attr.split(",")]:
                if is_optional == "true":
                    is_optionals.append(True)
                elif is_optional == "false":
                    is_optionals.append(False)
                else:
                    assert False, f"Unexpected optional attribute: {optional_attr}"
        is_consts: list[bool] = []
        const_or_stars = type_element.tail.strip()
        while len(const_or_stars) > 0:
            if const_or_stars.endswith("const"):
                is_consts.append(True)
            else:
                is_consts.append(False)
            const_or_stars = const_or_stars.removesuffix("const")
            const_or_stars = const_or_stars.strip()
            assert const_or_stars.endswith("*"), f"Unknown type: {''.join(param.itertext())}"
            const_or_stars = const_or_stars.removesuffix("*")
            const_or_stars = const_or_stars.strip()

        last_is_const = False
        if param.text is not None:
            type_prefix = param.text.strip()
            type_prefix = type_prefix.removesuffix("struct")
            type_prefix = type_prefix.strip()
            if type_prefix != "":
                assert type_prefix == "const", f"Unknown type: {''.join(param.itertext())}"
                last_is_const = True
        is_consts.append(last_is_const)

        assert len(is_optionals) <= len(is_consts), (
            "Length of optional attributes is greater than the levels of pointers unexpectedly in "
            f"param: {ElementTree.tostring(param, encoding='unicode')}."
        )
        is_optionals += [False] * (len(is_consts) - len(is_optionals))
        vk_xml_type = VkXmlType(
            is_const=is_consts.pop(),
            is_optional=is_optionals.pop(),
            name=decayed_type,
            reg_type_info=decayed_type_info,
            dimensions=dimensions,
        )
        for _ in range(len(is_consts)):
            vk_xml_type = VkXmlType(
                is_const=is_consts.pop(), is_optional=is_optionals.pop(), points_to=vk_xml_type
            )

        len_attr = param.get("len")
        len_var = None
        if len_attr is not None:
            len_attr = len_attr.strip()
            if len_attr.startswith("latexmath:"):
                altlen_attr = param.get("altlen")
                assert altlen_attr is not None, "Expect latexmath len comes with altlen"
                len_var = altlen_attr
                vk_xml_type.len = VkXmlLenKind.VARIABLE
            else:
                assert "," not in len_attr, "Multi-dimensional array unimplemented."
                assert len_attr != "1", "Unsupported length attribute: 1"
                if len_attr == "null-terminated":
                    vk_xml_type.len = VkXmlLenKind.NULL_TERMINATED
                else:
                    vk_xml_type.len = VkXmlLenKind.VARIABLE
                    len_var = len_attr

        return VkXmlParam(type=vk_xml_type, name=param_name, len_var=len_var)


class VkXmlCommand(NamedTuple):
    return_type: str
    name: str
    parameters: list[VkXmlParam]

    @staticmethod
    def from_cmd_info(
        cmdinfo: reg.CmdInfo, typeinfos: dict[str, reg.TypeInfo | reg.GroupInfo]
    ) -> VkXmlCommand:
        params: list[VkXmlParam] = []
        for param in cmdinfo.getParams():
            typename = get_param_decayed_type(param)
            assert typename in typeinfos, f"Unknown type: {typename}"
            typeinfo = typeinfos[typename]
            params.append(VkXmlParam.from_param_element(param, typeinfo))
        return_type = cmdinfo.elem.find("proto/type").text.strip()
        xmlElem = cmdinfo.elem
        name = xmlElem.get("name")
        assert name is not None, f"CmdInfo doesn't have a name: {ElementTree.tostring(xmlElem)}"
        return VkXmlCommand(return_type=return_type, name=name, parameters=params)

    def get_dispatch_chain_type(self) -> Optional[DispatchChainType]:
        global_commands = [
            "vkCreateInstance",
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkEnumerateInstanceVersion",
        ]
        if self.name in global_commands:
            return DispatchChainType.GLOBAL
        elif len(self.parameters) > 0 and self.parameters[0].type.name is not None:
            type_to_dispatch_chain = {
                "VkInstance": DispatchChainType.INSTANCE,
                "VkPhysicalDevice": DispatchChainType.INSTANCE,
                "VkDevice": DispatchChainType.DEVICE,
                "VkCommandBuffer": DispatchChainType.DEVICE,
                "VkQueue": DispatchChainType.DEVICE,
            }
            return type_to_dispatch_chain.get(self.parameters[0].type.name)
        return None


class RustType(NamedTuple):
    class ArrayInfo(NamedTuple):
        element_type: RustType
        length: int

    class TemplateInfo(NamedTuple):
        template_name: str
        template_params: list[RustType]

    is_const: bool = True
    is_optional: bool = False
    is_root: bool = True
    # A reference type only has the refers_to field. A pointer type only has the points_to field.
    # A slice type only has the slice_of field. An array type only has the array_info field.
    # Otherwise, the type only has the name field.
    refers_to: Optional[RustType] = None
    points_to: Optional[RustType] = None
    slice_of: Optional[RustType] = None
    array_info: Optional[ArrayInfo] = None
    template_info: Optional[TemplateInfo] = None
    name: Optional[str] = None

    @staticmethod
    def __handle_array_type(
        vk_xml_type: VkXmlType, from_vk_xml_type: Callable[[VkXmlType], RustType]
    ) -> VkXmlType:
        assert len(vk_xml_type.dimensions) > 0, "vk_xml_type must be an array type"
        dimensions = vk_xml_type.dimensions.copy()
        array_length = dimensions.pop()
        assert vk_xml_type.is_optional is False, "Optional array is not supported yet"
        element_vk_xml_type = dataclasses.replace(vk_xml_type, dimensions=dimensions)
        element_type = None
        if len(dimensions) == 0:
            element_type = from_vk_xml_type(element_vk_xml_type)
        else:
            element_type = RustType.__handle_array_type(element_vk_xml_type, from_vk_xml_type)
        element_type = element_type._replace(is_root=False)
        return RustType(
            is_const=vk_xml_type.is_const,
            is_optional=vk_xml_type.is_optional,
            array_info=RustType.ArrayInfo(
                length=array_length,
                element_type=element_type,
            ),
        )

    @staticmethod
    def from_vk_xml_type(vk_xml_type: VkXmlType) -> RustType:
        if len(vk_xml_type.dimensions) > 0:
            return RustType(
                refers_to=RustType.__handle_array_type(
                    vk_xml_type, RustType.from_vk_xml_type
                )._replace(is_root=False)
            )

        if vk_xml_type.name is not None:
            name = (
                "bool"
                if vk_xml_type.name == "VkBool32"
                else decayed_type_to_rust_type(vk_xml_type.name)
            )
            return RustType(
                is_const=vk_xml_type.is_const, is_optional=vk_xml_type.is_optional, name=name
            )

        assert (
            vk_xml_type.points_to is not None
        ), "A VkXmlType is either a basic type or a pointer type."

        if vk_xml_type.len == VkXmlLenKind.NULL_TERMINATED:
            assert (
                vk_xml_type.points_to is not None
                and vk_xml_type.points_to.name == "char"
                and vk_xml_type.points_to.is_const is True
            ), "null-terminated only supports const char* type."
            return RustType(
                is_optional=vk_xml_type.is_optional, refers_to=RustType(name="str", is_root=False)
            )
        elif vk_xml_type.len == VkXmlLenKind.VARIABLE:
            element_vk_xml_type = vk_xml_type.points_to
            element_rust_type: RustType = None
            if element_vk_xml_type.name == "void":
                # void* is actually an array of zero-able bytes
                element_rust_type = RustType(
                    name="u8", is_optional=True, is_const=element_vk_xml_type.is_const
                )
            else:
                element_rust_type = RustType.from_vk_xml_type(element_vk_xml_type)
            element_rust_type = element_rust_type._replace(is_root=False)
            # For mutable pointer parameters in vk.xml, they can be uninitialized (e.g.
            # pPhysicalDevices in vkEnumeratePhysicalDevices), which makes it invalid to use the
            # slice type (&mut [T]) to be the Rust interface type. The Rust slice type requires the
            # underlying data be initialized. We use &mut [MaybeUninit<T>] here instead.
            if not element_vk_xml_type.is_const:
                element_rust_type = RustType(
                    template_info=RustType.TemplateInfo(
                        template_name="MaybeUninit", template_params=[element_rust_type]
                    ),
                    is_const=False,
                    is_root=False,
                )
            return RustType(
                is_const=vk_xml_type.is_const,
                is_optional=vk_xml_type.is_optional,
                slice_of=element_rust_type,
            )
        else:
            assert vk_xml_type.len is None, "Should have exhausted all len link."

        if vk_xml_type.points_to.name in opaque_type_map:
            # A pointer to an opaque type will translate to a pointer
            return RustType(
                is_const=vk_xml_type.is_const,
                is_optional=vk_xml_type.is_optional,
                points_to=RustType.from_vk_xml_type(vk_xml_type.points_to)._replace(is_root=False),
            )
        else:
            refers_to = RustType.from_vk_xml_type(vk_xml_type.points_to)._replace(is_root=False)
            if not vk_xml_type.points_to.is_const:
                # For mutable pointer parameters in vk.xml, they can be uninitialized (e.g. pDevice
                # in vkCreateDevice), which makes it invalid to use the reference type (&mut T) to
                # be the Rust interface type. The Rust reference type requires the underlying data
                # be initialized. We use &mut MaybeUninit<T> here.
                refers_to = RustType(
                    is_const=vk_xml_type.points_to.is_const,
                    is_optional=False,
                    template_info=RustType.TemplateInfo(
                        template_name="MaybeUninit", template_params=[refers_to]
                    ),
                    is_root=False,
                )
            return RustType(
                is_const=vk_xml_type.is_const,
                is_optional=vk_xml_type.is_optional,
                refers_to=refers_to,
            )

    @staticmethod
    def from_vk_xml_type_for_ffi(vk_xml_type: VkXmlType) -> RustType:
        if len(vk_xml_type.dimensions) > 0:
            return RustType(
                is_const=True,
                is_optional=vk_xml_type.is_optional,
                points_to=RustType.__handle_array_type(
                    vk_xml_type, RustType.from_vk_xml_type_for_ffi
                )._replace(is_root=False),
            )

        if vk_xml_type.name is not None:
            return RustType(
                is_const=vk_xml_type.is_const,
                # Suppress the generation of Option<T> at the FFI boundary.
                is_optional=False,
                name=decayed_type_to_rust_type(vk_xml_type.name),
            )

        assert (
            vk_xml_type.points_to is not None
        ), "A VkXmlType is either a basic type or a pointer type."

        # Suppress the generation of Option<T>
        return RustType(
            is_const=vk_xml_type.is_const,
            is_optional=False,
            points_to=RustType.from_vk_xml_type_for_ffi(vk_xml_type.points_to)._replace(
                is_root=False
            ),
        )

    def to_string(self) -> str:
        if self.name is not None:
            # We do nothing on is_optional, optional=false on scalar type doesn't necessarily mean
            # the value must be non-zero.
            return self.name

        def wrap_option(inner: str) -> str:
            if self.is_optional:
                return f"Option<{inner}>"
            else:
                return inner

        if self.template_info is not None:
            template_info = self.template_info
            template_params: list[str] = [
                param.to_string() for param in template_info.template_params
            ]
            template_params: str = ", ".join(template_params)
            return wrap_option(f"{template_info.template_name}<{template_params}>")

        inner_type = self.points_to or self.slice_of or self.refers_to
        if self.array_info is not None:
            inner_type = self.array_info.element_type
        assert inner_type.name != "bool", "This interface breaks the ABI, and won't be efficient."

        if self.points_to is not None:

            def add_pointer(inner: str) -> str:
                if self.points_to.is_const:
                    return f"*const {inner}"
                else:
                    return f"*mut {inner}"

            return wrap_option(add_pointer(self.points_to.to_string()))
        elif self.slice_of is not None:
            mut_mod = "" if self.slice_of.is_const else "mut "
            return wrap_option(f"&{mut_mod}[{self.slice_of.to_string()}]")
        elif self.refers_to is not None:
            mut_mod = "" if self.refers_to.is_const else "mut "
            return wrap_option(f"&{mut_mod}{self.refers_to.to_string()}")
        elif self.array_info is not None:
            array_info = self.array_info
            return wrap_option(f"[{array_info.element_type.to_string()}; {array_info.length}]")
        else:
            assert False, "Unreachable"


class RustParam(NamedTuple):
    name: str
    type: RustType

    @staticmethod
    def from_vk_xml_param(vk_xml_param: VkXmlParam) -> RustParam:
        return RustParam(
            name="_" + camel_case_to_snake_case(vk_xml_param.name),
            type=RustType.from_vk_xml_type(vk_xml_param.type),
        )

    @staticmethod
    def param_name_from_vk_xml_param_for_ffi(vk_xml_param_name: str) -> str:
        return escape_rust_keywords(camel_case_to_snake_case(vk_xml_param_name))

    @staticmethod
    def from_vk_xml_param_for_ffi(vk_xml_param: VkXmlParam) -> RustParam:
        return RustParam(
            name=RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name),
            type=RustType.from_vk_xml_type_for_ffi(vk_xml_param.type),
        )

    def to_string(self) -> str:
        param_type = self.type.to_string()
        return f"{self.name}: {param_type}"


class VkXmlToRustMethodInfo(NamedTuple):
    return_info: Return
    rust_method_name: str
    parameters: list[Param]
    vk_xml_cmd: VkXmlCommand

    class Return(NamedTuple):
        main_source_vk_xml_params: list[VkXmlParam]
        rust_method_return_type: str

    class Param(NamedTuple):
        main_source_vk_xml_param: VkXmlParam
        rust_method_param: RustParam

    @staticmethod
    def from_vk_xml_command(vk_xml_cmd: VkXmlCommand) -> VkXmlToRustMethodInfo:
        vk_xml_params = vk_xml_cmd.parameters.copy()

        def handle_return_info() -> VkXmlToRustMethodInfo.Return:
            """
            Create the return info and remove the return parameters from the rust_params list
            """
            return_c_type = vk_xml_cmd.return_type
            if return_c_type == "VkBool32":
                # We use bool instead of vk::Bool32 for VkBool32 in our layer trait interfaces.
                return VkXmlToRustMethodInfo.Return(
                    main_source_vk_xml_params=[], rust_method_return_type="LayerResult<bool>"
                )
            if return_c_type not in ["void", "VkResult"]:
                # Handle return types that can't be combined with extra out parameters, e.g.
                # vkGetBufferOpaqueCaptureAddress which returns uint64_t.
                rust_method_return_type = f"LayerResult<{decayed_type_to_rust_type(return_c_type)}>"
                return VkXmlToRustMethodInfo.Return(
                    main_source_vk_xml_params=[],
                    rust_method_return_type=rust_method_return_type,
                )

            assert return_c_type in ["VkResult", "void"]
            assert len(vk_xml_params) > 0, "Unexpected command with 0 parameters."
            # TODO: add support to multiple out parameters.
            last_param_type = vk_xml_cmd.parameters[-1].type

            def get_rust_method_return_type(inner_type: str) -> str:
                if return_c_type == "VkResult":
                    return f"LayerResult<VkResult<{inner_type}>>"
                assert return_c_type == "void"
                return f"LayerResult<{inner_type}>"

            return_info = VkXmlToRustMethodInfo.Return(
                main_source_vk_xml_params=[],
                rust_method_return_type=get_rust_method_return_type("()"),
            )
            if last_param_type.decayed_type().is_struct():
                # For struct type, we don't treat it as a return type given the complication of
                # pNext chain of the return parameter, and possible input fields.
                return return_info

            if vk_xml_cmd.name == "vkCreateDevice":
                # We don't treat VkDevice just as a return value, because the layer is supposed
                # to pass through the passed in pDevice parameter.
                return return_info

            if last_param_type.points_to is None:
                # If the last parameter is not a pointer, it's not an out parameter, and won't be
                # included in the return type
                return return_info

            # The last parameter is a C pointer.

            if last_param_type.points_to.is_const:
                # The last parameter is not a mutable pointer, so it's not an out parameter.
                return return_info

            last_rust_method_type = RustType.from_vk_xml_type(last_param_type)

            if last_param_type.len is None:
                # If the last parameter is a mutable pointer of a simple type. It's part of the
                # return type. If the last parameter is void*, it's possible that the spec is
                # missing the len attribute(e.g. vkGetSamplerOpaqueCaptureDescriptorDataEXT), but we
                # are sure, so we don't transform the parameter type, and it won't be part of the
                # return type.
                if last_param_type.points_to.name == "void":
                    return return_info
                else:
                    assert last_param_type.points_to is not None
                    assert last_rust_method_type.refers_to is not None
                    assert last_rust_method_type.refers_to.template_info is not None
                    assert (
                        last_rust_method_type.refers_to.template_info.template_name == "MaybeUninit"
                    )
                    rust_return_type = (
                        last_rust_method_type.refers_to.template_info.template_params[0]
                    )
                    return VkXmlToRustMethodInfo.Return(
                        main_source_vk_xml_params=[vk_xml_params.pop(-1)],
                        rust_method_return_type=get_rust_method_return_type(
                            rust_return_type.to_string()
                        ),
                    )

            # The last parameter is a C array.

            if last_param_type.points_to.name == "void":
                # For an array of void, we don't return a Vec, as the return value can be large, and
                # we should use the passed-in memory as the storage.
                return return_info

            # TODO: If the len_var is a const parameter, we should still pass the slice as the
            # interface. In this case, if we return a Vec and remove this parameter, we don't know
            # the length to use to call into the Vulkan API.
            # TODO: returning Vec<_> could cause extra copy here compared to directly passing a
            # mutable slice to the layer trait. Even worse, the layer won't be able to access the
            # length of the passed in write buffer. Restrict the use of Vec<_> to only return
            # handles
            parameter_names = [param.name for param in vk_xml_params]
            if vk_xml_params[-1].len_var in parameter_names:
                len_param_index = parameter_names.index(vk_xml_params[-1].len_var)
                vk_xml_params.pop(len_param_index)
            assert last_rust_method_type.slice_of is not None
            assert (
                last_rust_method_type.slice_of.template_info is not None
            ), "Mutable slice must be MaybeUninit as a parameter"
            assert last_rust_method_type.slice_of.template_info.template_name == "MaybeUninit"
            element_type = last_rust_method_type.slice_of.template_info.template_params[0]
            return VkXmlToRustMethodInfo.Return(
                main_source_vk_xml_params=[vk_xml_params.pop(-1)],
                rust_method_return_type=get_rust_method_return_type(
                    f"Vec<{element_type.to_string()}>"
                ),
            )

        def handle_parameters() -> list[VkXmlToRustMethodInfo.Param]:
            """
            Create the VkXmlToRustMethodInfo.Param list after the return parameters are removed.

            Some parameters may not appear in the final Rust method interface, because they are
            included in another parameter type (e.g. in get_ray_tracing_shader_group_handles_khr
            p_data is a slice. data_size is the length of p_data, which is included in the length of
            the slice).
            """
            len_params = [param.len_var for param in vk_xml_params if param.len_var is not None]
            res: list[VkXmlToRustMethodInfo.Param] = []
            for i, param in enumerate(vk_xml_params):
                # Remove the leading VkInstance or VkDevice parameter, since it should be included in
                # &self.
                if i == 0 and param.type.name is not None:
                    if param.type.name in ["VkInstance", "VkDevice"]:
                        continue
                # Any non-pointer length parameters are removed, as they are always included in the
                # correspondent slice parameter
                if param.name in len_params and param.type.name is not None:
                    continue

                res.append(
                    VkXmlToRustMethodInfo.Param(
                        main_source_vk_xml_param=param,
                        rust_method_param=RustParam.from_vk_xml_param(param),
                    )
                )
            return res

        return_info = handle_return_info()
        rust_params = handle_parameters()

        if vk_xml_cmd.name == "vkCreateDevice":
            # Also pass the current VkLayerDeviceLink and the pDevice to the layer.
            rust_params.insert(
                2,
                VkXmlToRustMethodInfo.Param(
                    main_source_vk_xml_param=vk_xml_params[1],
                    rust_method_param=RustParam(
                        name="_layer_device_link",
                        type=RustType(refers_to=RustType(name="VkLayerDeviceLink", is_root=False)),
                    ),
                ),
            )
        return VkXmlToRustMethodInfo(
            return_info=return_info,
            rust_method_name=camel_case_to_snake_case(vk_xml_cmd.name.removeprefix("vk")),
            parameters=rust_params,
            vk_xml_cmd=vk_xml_cmd,
        )


class RustMethod(NamedTuple):
    return_type: str
    name: str
    parameters: list[RustParam]
    vk_xml_cmd: VkXmlCommand

    @staticmethod
    def from_vk_xml_command(vk_xml_cmd: VkXmlCommand) -> RustMethod:
        vk_xml_to_rust_method_info = VkXmlToRustMethodInfo.from_vk_xml_command(vk_xml_cmd)
        return RustMethod(
            name=vk_xml_to_rust_method_info.rust_method_name,
            return_type=vk_xml_to_rust_method_info.return_info.rust_method_return_type,
            parameters=[param.rust_method_param for param in vk_xml_to_rust_method_info.parameters],
            vk_xml_cmd=vk_xml_cmd,
        )

    def to_string(self) -> str:
        class TypeParam(NamedTuple):
            name: str
            restriction: str

        def generate_type_params(used_params: list[TypeParam]) -> Generator[str, str, None]:
            type_arg_names = ["T", "U", "V", "W"]
            for type_arg_name in type_arg_names:
                restriction = yield type_arg_name
                used_params.append(TypeParam(name=type_arg_name, restriction=restriction))
            assert False, "Too many generic type parameters."

        used_type_params: list[TypeParam] = []
        type_params = generate_type_params(used_type_params)

        # TODO: move this logic to RustMethod generation process so that the Rust type
        # representation is more natural
        def param_to_string(param: RustParam) -> str:
            if param.type.slice_of is not None and param.type.slice_of.name == "bool":
                type_param = next(type_params)
                type_params.send("Iterator<Item = bool> + 'static")
                # Generic type for bool for efficient ABI breaking API.
                return f"{param.name}: {type_param}"
            return param.to_string()

        params = ", ".join([param_to_string(param) for param in self.parameters])
        # Prevent the Rust linter from complaining explicit -> ()
        assert (
            self.return_type != "()"
        ), "Unexpected rust return type: (). All return type should be LayerResult<T>"
        generic_params: str = ""
        if len(used_type_params) > 0:
            generic_params = ", ".join(
                [f"{type_param.name}: {type_param.restriction}" for type_param in used_type_params]
            )
            generic_params = f"<{generic_params}>"
        return f"fn {self.name}{generic_params}(&self, {params}) -> {self.return_type}"


def generate_unhandled_command_comments(
    unhandled_commands: list[UnhandledCommand], indent: int = 0
) -> str:
    if len(unhandled_commands) == 0:
        return ""
    lines = ["// Unhandled commands:"]
    lines += [f"// * {cmd.name}: {cmd.reason}" for cmd in unhandled_commands]
    return "".join([" " * indent + line + "\n" for line in lines])


class TestUtils:

    @dataclass
    class __VkXml:
        cmd_dict: dict[str, reg.CmdInfo] = field(default_factory=dict)
        type_infos: dict[str, reg.TypeInfo | reg.GroupInfo] = field(default_factory=dict)

    @staticmethod
    @cache
    def __get_vk_xml() -> TestUtils.__VkXml:
        vk_xml_path = (
            Path(__file__).absolute().parents[2]
            / "third_party"
            / "Vulkan-Headers"
            / "registry"
            / "vk.xml"
        )
        tree = ElementTree.parse(str(vk_xml_path))
        registry = reg.Registry()
        registry.loadElementTree(tree)
        return TestUtils.__VkXml(cmd_dict=registry.cmddict, type_infos=registry.typedict)

    @staticmethod
    def get_vk_xml_command(command_name: str) -> VkXmlCommand:
        vk_xml = TestUtils.__get_vk_xml()
        return VkXmlCommand.from_cmd_info(vk_xml.cmd_dict[command_name], vk_xml.type_infos)


class TestRustMethod(unittest.TestCase):
    def test_get_image_sparse_memory_requirements2(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetImageSparseMemoryRequirements2")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(rust_method.return_type, "LayerResult<()>")
        self.assertEqual(
            rust_method.parameters[0].type.to_string(), "&vk::ImageSparseMemoryRequirementsInfo2"
        )
        self.assertEqual(rust_method.parameters[1].type.to_string(), "&mut MaybeUninit<u32>")
        self.assertEqual(
            rust_method.parameters[2].type.to_string(),
            "Option<&mut [MaybeUninit<vk::SparseImageMemoryRequirements2>]>",
        )

    def test_allocate_descriptor_sets(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkAllocateDescriptorSets")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(
            rust_method.parameters[0].type.to_string(), "&vk::DescriptorSetAllocateInfo"
        )
        self.assertEqual(rust_method.return_type, "LayerResult<VkResult<Vec<vk::DescriptorSet>>>")

    def test_get_device_queue(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetDeviceQueue")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(len(rust_method.parameters), 2)
        self.assertEqual(rust_method.return_type, "LayerResult<vk::Queue>")

    def test_queue_submit(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkQueueSubmit")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(rust_method.parameters[0].type.to_string(), "vk::Queue")
        self.assertEqual(rust_method.parameters[1].type.to_string(), "&[vk::SubmitInfo]")
        self.assertEqual(rust_method.parameters[2].type.to_string(), "vk::Fence")

    def test_get_ray_tracing_shader_group_handles_khr(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetRayTracingShaderGroupHandlesKHR")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(rust_method.parameters[0].type.to_string(), "vk::Pipeline")
        self.assertEqual(rust_method.parameters[1].type.to_string(), "u32")
        self.assertEqual(rust_method.parameters[2].type.to_string(), "u32")
        self.assertEqual(rust_method.parameters[3].type.to_string(), "&mut [MaybeUninit<u8>]")
        self.assertEqual(rust_method.return_type, "LayerResult<VkResult<()>>")

    def test_get_sampler_opaque_capture_descriptor_data_ext(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetSamplerOpaqueCaptureDescriptorDataEXT")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(
            rust_method.parameters[0].type.to_string(), "&vk::SamplerCaptureDescriptorDataInfoEXT"
        )
        self.assertEqual(rust_method.parameters[1].type.to_string(), "*mut c_void")

    def test_ffi_cmd_set_blend_constants(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdSetBlendConstants")
        blend_constants_ffi_param = RustParam.from_vk_xml_param_for_ffi(
            vk_xml_command.parameters[1]
        )
        self.assertEqual(blend_constants_ffi_param.to_string(), "blend_constants: *const [f32; 4]")

    def test_enumerate_physical_device_queue_family_performance_query_counters_khr(self):
        vk_xml_command = TestUtils.get_vk_xml_command(
            "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"
        )
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(rust_method.parameters[2].type.to_string(), "&mut MaybeUninit<u32>")
        self.assertEqual(
            rust_method.parameters[3].type.to_string(),
            "Option<&mut [MaybeUninit<vk::PerformanceCounterKHR>]>",
        )
        self.assertEqual(
            rust_method.parameters[4].type.to_string(),
            "Option<&mut [MaybeUninit<vk::PerformanceCounterDescriptionKHR>]>",
        )

    def test_map_memory(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkMapMemory")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        self.assertEqual(len(rust_method.parameters), 4)
        self.assertEqual(rust_method.return_type, "LayerResult<VkResult<Option<*mut c_void>>>")

    def test_get_physical_device_surface_support_khr(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetPhysicalDeviceSurfaceSupportKHR")
        vk_xml_to_rust_method_info = VkXmlToRustMethodInfo.from_vk_xml_command(vk_xml_command)
        return_info = vk_xml_to_rust_method_info.return_info
        self.assertEqual(len(return_info.main_source_vk_xml_params), 1)
        self.assertEqual(return_info.main_source_vk_xml_params[0].name, "pSupported")
