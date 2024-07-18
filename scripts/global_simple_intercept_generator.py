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
from abc import ABC, abstractmethod
import unittest
from generator import OutputGenerator
import reg
import sys
from typing import NamedTuple, Optional, ClassVar, Callable
from vk_xml_util import (
    DispatchChainType,
    VkXmlCommand,
    camel_case_to_snake_case,
    snake_case_to_upper_camel_case,
    RustParam,
    RustType,
    decayed_type_to_rust_type,
    UnhandledCommand,
    generate_unhandled_command_comments,
    VkXmlType,
    VkXmlLenKind,
    VkXmlParam,
    VulkanAliases,
    write_preamble,
    TestUtils,
    RustMethod,
    VkXmlToRustMethodInfo,
)
from dataclasses import dataclass, field


@dataclass
class CommandDispatchInfo:
    class CoreInfo(NamedTuple):
        version_major: int
        version_minor: int

    class ExtensionInfo(NamedTuple):
        vendor: str
        name: str

    # Core commands only have core_info field. Extension commands only have extension_info field.
    core_info: Optional[CoreInfo]
    extension_info: Optional[ExtensionInfo]
    commands: set[str] = field(default_factory=set)

    deprecated_extensions: ClassVar[set[str]] = set(
        [
            "VK_EXT_debug_report",
        ]
    )

    @staticmethod
    def from_feature_info(
        feature: reg.FeatureInfo,
        should_skip_command: Callable[[str], bool],
    ) -> CommandDispatchInfo:
        core_info: Optional[CommandDispatchInfo.CoreInfo] = None
        extension_info: Optional[CommandDispatchInfo.ExtensionInfo] = None
        if feature.category == "VERSION":
            version_number = feature.versionNumber.split(".")
            core_info = CommandDispatchInfo.CoreInfo(
                version_major=int(version_number[0]), version_minor=int(version_number[1])
            )
        else:
            extension_info = CommandDispatchInfo.ExtensionInfo(
                vendor=feature.category, name=feature.name
            )
        commands: set[str] = set()
        for command_elem in feature.elem.iterfind("require/command"):
            name = command_elem.get("name")
            assert name is not None, f"Failed to find the name of a command for {feature.category}"
            if should_skip_command(name):
                continue
            commands.add(name)

        return CommandDispatchInfo(
            core_info=core_info, extension_info=extension_info, commands=commands
        )

    def get_dispatch_table_field_name(self) -> str:
        if self.core_info is not None:
            return f"core.fp_v{self.core_info.version_major}_{self.core_info.version_minor}()"
        elif self.extension_info is not None:
            return self.extension_info.name[3:].lower()
        else:
            assert False, "CommandDispatchInfo should either be core or extension."

    def _get_enum_variant_name(self) -> str:
        if self.extension_info is not None:
            extension_info = self.extension_info
            return snake_case_to_upper_camel_case(extension_info.name.removeprefix("VK_"))
        else:
            assert False, "Only extension is defined as enum variant"

    def get_enum_name(self) -> str:
        """Return the enum with the path e.g. Feature::Core(ApiVersion{major: 1, minor: 1})"""
        if self.core_info is not None:
            core_info = self.core_info
            return (
                f"Feature::Core(ApiVersion {{ major: {core_info.version_major}, minor: "
                f"{core_info.version_minor}}})"
            )
        elif self.extension_info is not None:
            return f"Feature::Extension(Extension::{self._get_enum_variant_name()})"
        else:
            assert False, "A dispatch info should be either from an extension or from the core"

    @staticmethod
    def get_feature_enum_lines(dispatch_infos: list[CommandDispatchInfo]) -> list[str]:
        extension_enum_lines = [
            "#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]",
            "pub enum Extension {",
        ]
        extension_enum_try_from_lines = [
            "impl TryFrom<&str> for Extension {",
            "    type Error = TryFromExtensionError;",
            "",
            "    fn try_from(value: &str) -> Result<Self, Self::Error> {",
            "        match value {",
        ]
        str_from_extension_enum_lines = [
            "impl From<Extension> for &'static str {",
            "    fn from(value: Extension) -> &'static str {",
            "         match value {",
        ]
        for dispatch_info in dispatch_infos:
            if dispatch_info.core_info is not None:
                pass
            elif dispatch_info.extension_info is not None:
                variant_name = dispatch_info._get_enum_variant_name()
                extension_enum_lines.append(f"    {variant_name},")
                extension_info = dispatch_info.extension_info
                extension_enum_try_from_lines.append(
                    f'            "{extension_info.name}" => Ok(Extension::{variant_name}),'
                )
                str_from_extension_enum_lines.append(
                    f'            Extension::{variant_name} => "{extension_info.name}",'
                )
            else:
                assert False, "A dispatch info should be either from an extension or from the core"

        extension_enum_lines.append("}")
        extension_enum_try_from_lines += [
            "            _ => Err(TryFromExtensionError::UnknownExtension(value.to_owned()))",
            "        }",
            "    }",
            "}",
        ]
        str_from_extension_enum_lines += [
            "        }",
            "    }",
            "}",
        ]
        return (
            extension_enum_lines
            + ["\n"]
            + extension_enum_try_from_lines
            + ["\n"]
            + str_from_extension_enum_lines
            + ["\n"]
        )

    @staticmethod
    def get_disapatch_tables_impl_lines(
        dispatch_infos: list[CommandDispatchInfo],
        instance_commands: dict[str, VulkanCommand],
        device_commands: dict[str, VulkanCommand],
        skip_commands: set[str],
    ) -> list[str]:
        dispatch_infos = [info for info in dispatch_infos if len(info.commands - skip_commands) > 0]
        device_dispatch_def_lines: list[str] = [
            "pub(crate) struct DeviceDispatchTable {",
            "    pub core: Arc<ash::Device>,",
        ]
        device_dispatch_impl_lines: list[str] = [
            "impl DeviceDispatchTable {",
            (
                "    pub(crate) fn load(get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr, device:"
                " Arc<ash::Device>) -> Self {"
            ),
            (
                "        let proc_addr_loader = get_device_proc_addr_loader(get_device_proc_addr, "
                "&device);"
            ),
            "        Self {",
            "            core: Arc::clone(&device),",
        ]
        instance_dispatch_def_lines: list[str] = [
            "pub(crate) struct InstanceDispatchTable {",
            "    pub core: Arc<ash::Instance>,",
        ]
        instance_dispatch_impl_lines: list[str] = [
            "impl InstanceDispatchTable {",
            (
                "    pub(crate) fn load(get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr, "
                "instance: Arc<ash::Instance>) -> Self {"
            ),
            (
                "        let proc_addr_loader ="
                " get_instance_proc_addr_loader(get_instance_proc_addr, &instance);"
            ),
            "        Self {",
            "            core: Arc::clone(&instance),",
        ]

        for dispatch_info in dispatch_infos:
            if dispatch_info.core_info is not None:
                continue
            if len(dispatch_info.commands - skip_commands) == 0:
                continue
            assert (
                dispatch_info.extension_info is not None
            ), "A CommandDispatchInfo should be either a core dispatch or an extension dispatch."
            extension_info = dispatch_info.extension_info
            def_lines: list[str] = []
            impl_lines: list[str] = []

            # Remove the VK_ prefix
            field_name = dispatch_info.get_dispatch_table_field_name()
            field_type = f"ash::vk::{snake_case_to_upper_camel_case(field_name)}Fn"

            if extension_info.name in CommandDispatchInfo.deprecated_extensions:
                def_lines.append("    #[allow(deprecated)]")
            def_lines.append(f"    {field_name}: Arc<{field_type}>,")
            impl_lines.append(
                f"            {field_name}: Arc::new({field_type}::load(&proc_addr_loader)),"
            )

            if any([command in instance_commands for command in dispatch_info.commands]):
                instance_dispatch_def_lines += def_lines
                instance_dispatch_impl_lines += impl_lines
            if any([command in device_commands for command in dispatch_info.commands]):
                device_dispatch_def_lines += def_lines
                device_dispatch_impl_lines += impl_lines

        device_dispatch_def_lines.append("}")
        device_dispatch_impl_lines += [
            "        }",
            "    }",
            "}",
        ]
        instance_dispatch_def_lines.append("}")
        instance_dispatch_impl_lines += [
            "        }",
            "    }",
            "}",
        ]

        return (
            device_dispatch_def_lines
            + device_dispatch_impl_lines
            + instance_dispatch_def_lines
            + instance_dispatch_impl_lines
        )


class RustFfiFunction(NamedTuple):
    name: str
    return_type: str
    parameters: list[RustParam]

    @staticmethod
    def from_vk_xml_command(vk_xml_cmd: VkXmlCommand) -> RustFfiFunction:
        c_return_type = vk_xml_cmd.return_type
        return_type = "()" if c_return_type == "void" else decayed_type_to_rust_type(c_return_type)
        return RustFfiFunction(
            name=camel_case_to_snake_case(vk_xml_cmd.name.removeprefix("vk")),
            return_type=return_type,
            parameters=[
                RustParam.from_vk_xml_param_for_ffi(vk_xml_param)
                for vk_xml_param in vk_xml_cmd.parameters
            ],
        )

    def get_def_str(self) -> str:
        params = ", ".join([param.to_string() for param in self.parameters])
        fn_return_type = ""
        if self.return_type != "()":
            fn_return_type = f" -> {self.return_type}"
        return f'extern "system" fn {self.name}({params}){fn_return_type}'


class VulkanCommand(NamedTuple):
    vk_xml_command: VkXmlCommand
    rust_fn: RustFfiFunction
    rust_method: RustMethod

    def __find_param_by_name(self, name: str) -> Optional[tuple[VkXmlParam, RustParam]]:
        all_params = zip(self.vk_xml_command.parameters, self.rust_fn.parameters)
        return next((param for param in all_params if param[0].name == name), None)

    # Return lines of code to assign the variable of ret_var to the FFI return parameter correctly,
    # e.g. assign the result of create_image to the p_image pointer of the FFI parameter.
    def __generate_assign_to_ret_param_lines(
        self, ret_var: str, ret_param_index: int, last_expr_type: str = "()"
    ) -> list[str]:
        assert last_expr_type in ["()", "vk::Result"]
        assert ret_param_index < len(
            self.vk_xml_command.parameters
        ), "Parameter index out of range."
        ret_xml_param = self.vk_xml_command.parameters[ret_param_index]
        ret_rust_param = self.rust_fn.parameters[ret_param_index]
        ret_param_xml_type = ret_xml_param.type
        need_to_assign_len_param = False
        if ret_param_xml_type.len == VkXmlLenKind.VARIABLE:
            len_expr: str = None
            if "->" in ret_xml_param.len_var:
                len_components = ret_xml_param.len_var.split("->")
                assert (
                    len(len_components) == 2
                ), f"Unsupported length parameter: {ret_xml_param.len_var}"
                len_struct_name, len_field_name = len_components
                len_struct_param = self.__find_param_by_name(len_struct_name)
                assert (
                    len_struct_param is not None
                ), f"Failed to find the length parameter: {len_struct_name}"
                len_struct_xml_param, len_struct_rust_param = len_struct_param
                assert not len_struct_xml_param.type.is_optional, (
                    f"The length of {ret_xml_param.name}, {len_struct_name} in "
                    f"{ret_xml_param.len_var}, is optional. This is currently not supported."
                )
                len_expr = (
                    f"unsafe {{ {len_struct_rust_param.name}.as_ref().unwrap() }}."
                    f"{camel_case_to_snake_case(len_field_name)}.try_into().unwrap()"
                )
            else:
                len_param = self.__find_param_by_name(ret_xml_param.len_var)
                assert (
                    len_param is not None
                ), f"Failed to find the length parameter: {ret_xml_param.len_var}."
                len_xml_param, len_rust_param = len_param
                len_xml_type = len_xml_param.type
                assert not len_xml_type.is_optional, "Can't handle optional length for now."
                if len_xml_type.name in ["uint32_t", "size_t"]:
                    len_expr = len_rust_param.name
                    if len_xml_type.name != "size_t":
                        len_expr = f"{len_rust_param.name}.try_into().unwrap()"
                elif len_xml_type.points_to is not None and len_xml_type.points_to.name in [
                    "uint32_t",
                    "size_t",
                ]:
                    len_expr = len_rust_param.name
                    need_to_assign_len_param = True
                else:
                    assert False, f"Unhandled length parameter type: {len_xml_type}"

            lines = []
            if ret_param_xml_type.points_to.name == "void":
                lines.append(f"let {ret_rust_param.name} = {ret_rust_param.name} as *mut u8;")
            if not need_to_assign_len_param:
                lines.append(
                    f"unsafe {{ std::slice::from_raw_parts_mut({ret_rust_param.name}, {len_expr}) "
                    f"}}.copy_from_slice(&{ret_var});"
                )
                if last_expr_type == "vk::Result":
                    lines.append("vk::Result::SUCCESS")
                return lines
            else:
                fill_array_lines = [
                    "    fill_vk_out_array(",
                    f"        &{ret_var},",
                    f"        NonNull::new({len_expr}).unwrap(),",
                    f"        {ret_rust_param.name},",
                    "    )",
                ]
                if last_expr_type == "vk::Result":
                    return lines + ["unsafe {"] + fill_array_lines + ["}"]
                else:
                    fill_array_lines[-1] += ";"
                    return (
                        lines
                        + [
                            f"// We can't return INCOMPLETE from {self.vk_xml_command.name}",
                            "#[allow(unused_must_use)]",
                            "unsafe {",
                        ]
                        + fill_array_lines
                        + ["}"]
                    )
        elif ret_param_xml_type.len is None:
            assert (
                not ret_param_xml_type.is_optional
            ), f"Optional return type in {self.vk_xml_command.name} is not supported."
            ret_expr = ret_var
            if ret_param_xml_type.points_to.name == "VkBool32":
                ret_expr = f"if {ret_var} {{ vk::TRUE }} else {{ vk::FALSE }}"
            elif (
                ret_param_xml_type.points_to.points_to is not None
                and ret_param_xml_type.points_to.is_optional
            ):
                assert (
                    not ret_param_xml_type.points_to.points_to.is_const
                ), "Optional const pointer return value is not supported."
                ret_expr = f"{ret_var}.unwrap_or(std::ptr::null_mut())"
            lines = [f"*unsafe {{ {ret_rust_param.name}.as_mut() }}.unwrap() = {ret_expr};"]
            if last_expr_type == "vk::Result":
                lines.append("vk::Result::SUCCESS")
            return lines
        else:
            assert False, f"Unsupported return type with length type: {ret_param_xml_type.len}"

    def get_rust_fn_impl_lines(self, dispatch_infos: list[CommandDispatchInfo]) -> list[str]:
        lines = ["let global = Self::instance();"]
        lines.append(f"// {self.vk_xml_command.name}")
        assert (
            len(self.vk_xml_command.parameters) > 0
        ), "Vulkan commands should have at least one parameter."
        dispatch_chain_type = self.vk_xml_command.get_dispatch_chain_type()
        dispatch_chain_var = None
        if dispatch_chain_type == DispatchChainType.DEVICE:
            dispatch_chain_var = "device_info"
            lines.append(
                f"let {dispatch_chain_var} = global.get_device_info"
                f"({self.rust_fn.parameters[0].name}).unwrap();"
            )
        elif dispatch_chain_type == DispatchChainType.INSTANCE:
            dispatch_chain_var = "instance_info"
            lines.append(
                f"let {dispatch_chain_var} = global.get_instance_info"
                f"({self.rust_fn.parameters[0].name}).unwrap();"
            )
        else:
            assert False, f"Unhandled dispatch chain type: {dispatch_chain_type}"
        dispatch_info = next(
            (info for info in dispatch_infos if self.vk_xml_command.name in info.commands), None
        )
        assert (
            dispatch_info is not None
        ), f"No dispatch info found for command {self.vk_xml_command.name}"
        lines += [
            (
                f"let dispatch_table = &{dispatch_chain_var}.dispatch_table."
                f"{dispatch_info.get_dispatch_table_field_name()};"
            ),
        ]

        vk_xml_to_rust_method_info = VkXmlToRustMethodInfo.from_vk_xml_command(self.vk_xml_command)

        intercept_params: list[str] = []
        param_transformer = ParamTransformer.create()
        for param in vk_xml_to_rust_method_info.parameters:
            rust_param = param.rust_method_param
            xml_param = param.main_source_vk_xml_param
            assert param_transformer.type_matches(
                rust_param.type, xml_param.type, self.vk_xml_command
            ), (
                f"Failed to transform parameter {xml_param.name}, {rust_param.name} in "
                f"{self.vk_xml_command.name}."
            )
            arg_exp = param_transformer.transform(rust_param, xml_param, self.vk_xml_command)
            intercept_params.append(arg_exp)

        param_names = [param.name for param in self.vk_xml_command.parameters]

        def generate_ret_expr(ret_var: str) -> list[str]:
            return_info = vk_xml_to_rust_method_info.return_info
            if self.vk_xml_command.return_type == "VkResult":
                if len(return_info.main_source_vk_xml_params) == 0:
                    return [
                        f"match {ret_var} {{",
                        "    Ok(()) => vk::Result::SUCCESS,",
                        "    Err(e) => e,",
                        "}",
                    ]
                assert len(return_info.main_source_vk_xml_params) == 1
                param_index = param_names.index(return_info.main_source_vk_xml_params[0].name)
                assign_to_ret_param_lines = self.__generate_assign_to_ret_param_lines(
                    "res", param_index, last_expr_type="vk::Result"
                )
                return (
                    [
                        f"match {ret_var} {{",
                        "    Ok(res) => {",
                    ]
                    + [8 * " " + line for line in assign_to_ret_param_lines]
                    + [
                        "    }",
                        "    Err(e) => e,",
                        "}",
                    ]
                )
            elif self.vk_xml_command.return_type == "void":
                if len(return_info.main_source_vk_xml_params) != 0:
                    assert len(return_info.main_source_vk_xml_params) == 1
                    param_index = param_names.index(return_info.main_source_vk_xml_params[0].name)
                    return self.__generate_assign_to_ret_param_lines(ret_var, param_index)
            elif self.vk_xml_command.return_type == "VkBool32":
                return [f"if {ret_var} {{ vk::TRUE }} else {{ vk::FALSE }}"]

            return [ret_var]

        rust_ffi_param_names = [param.name for param in self.rust_fn.parameters]
        lines += (
            [
                (
                    "let layer_result ="
                    f" {dispatch_chain_var}.customized_info.borrow().hooks().{self.rust_fn.name}("
                    f"{', '.join(intercept_params)});"
                ),
                "match layer_result {",
                "    LayerResult::Handled(res) => {",
            ]
            + [8 * " " + line for line in generate_ret_expr("res")]
            + [
                "    }",
                (
                    f"    LayerResult::Unhandled => unsafe {{ (dispatch_table.{self.rust_fn.name})("
                    f"{', '.join(rust_ffi_param_names)}) }},"
                ),
                "}",
            ]
        )
        return lines


class GlobalSimpleInterceptGenerator(OutputGenerator):
    @staticmethod
    def should_skip_cmd(command: str) -> Optional[UnhandledCommand]:
        return UnhandledCommand.find(command)

    def __init__(self, err_file=sys.stderr, warn_file=sys.stderr, diag_file=sys.stdout):
        super().__init__(err_file, warn_file, diag_file)
        self.device_commands: dict[str, VulkanCommand] = {}
        self.instance_commands: dict[str, VulkanCommand] = {}
        self.unhandled_commands: dict[str, UnhandledCommand] = {}
        self.command_aliases: list[set[str]] = []
        self.dispatch_infos: dict[str, CommandDispatchInfo] = {}
        self.types: dict[str, reg.TypeInfo] = {}
        self.manually_implemented_cmd: set[str] = set(
            [
                "vkDestroyInstance",
                "vkEnumeratePhysicalDevices",
                "vkEnumeratePhysicalDeviceGroups",
                "vkGetInstanceProcAddr",
                "vkGetDeviceProcAddr",
                "vkCreateDevice",
                "vkDestroyDevice",
                "vkEnumerateDeviceExtensionProperties",
                "vkEnumerateDeviceLayerProperties",
            ]
        )
        self.command_aliases: VulkanAliases = VulkanAliases()

    def beginFile(self, gen_opts):
        super().beginFile(gen_opts)

        write_preamble(self.outFile)
        self.newline()

        self.outFile.write("// This file is generated from the Vulkan XML API registry.\n")
        self.outFile.write(
            "\n".join(
                [
                    "#![allow(unused_unsafe)]",
                    (
                        "use std::{borrow:: Borrow, ffi::{c_int, c_void, c_char, CStr},"
                        " ptr::NonNull, sync::Arc, collections::HashSet};"
                    ),
                    "use ash::vk;",
                    "use smallvec::smallvec;",
                    "",
                    (
                        "use crate::{DeviceInfo, fill_vk_out_array, Global, InstanceInfo, Layer, "
                        "LayerResult, LayerVulkanCommand, InstanceHooks, DeviceHooks, "
                        "vk_utils::{slice_from_raw_parts, ptr_as_uninit_mut}};"
                    ),
                    (
                        "use super::{get_instance_proc_addr_loader, get_device_proc_addr_loader, "
                        "VulkanCommand, TryFromExtensionError, ApiVersion, Feature, "
                        "bool_iterator_from_raw_parts, maybe_slice_from_raw_parts, "
                        "maybe_uninit_slice_from_raw_parts_mut, uninit_slice_from_raw_parts_mut};"
                    ),
                    "",
                ]
            )
        )
        self.newline()

    def endFile(self):
        all_commands = self.instance_commands | self.device_commands
        # Handle aliases
        for command in all_commands:
            represent_name = self.command_aliases.get_represent_name(command)
            assert represent_name is not None, f"{command} is never added to the command aliases."
            for commands in [self.instance_commands, self.device_commands]:
                if command in commands:
                    commands[command] = all_commands[represent_name]

        # merge the 2 dictionaries again, because the aliased commands are directed to point to the
        # same version
        all_commands = self.instance_commands | self.device_commands
        aliased_commands = set(
            (
                name
                for name, command_info in all_commands.items()
                if command_info.vk_xml_command.name != name
            )
        )
        not_aliased_commands = [
            cmd for name, cmd in all_commands.items() if name not in aliased_commands
        ]
        # Not all dispatch will be used. For some extension, e.g. VK_EXT_shader_object, all commands
        # are in other extensions like VK_EXT_extended_dynamic_state, so we don't need to create a
        # dispatch table for it.
        used_dispatch_info: list[CommandDispatchInfo] = []
        for command in not_aliased_commands:
            command_name = command.vk_xml_command.name
            dispatch_info = next(
                (info for info in self.dispatch_infos.values() if command_name in info.commands),
                None,
            )
            assert dispatch_info is not None, f"Can't find dispatch info for {command_name}"
            if dispatch_info not in used_dispatch_info:
                used_dispatch_info.append(dispatch_info)

        self.outFile.write(
            "\n".join(
                CommandDispatchInfo.get_feature_enum_lines(list(self.dispatch_infos.values()))
            )
        )
        self.newline()
        self.newline()

        self.outFile.write(
            "\n".join(
                CommandDispatchInfo.get_disapatch_tables_impl_lines(
                    used_dispatch_info,
                    self.instance_commands,
                    self.device_commands,
                    aliased_commands,
                )
            )
        )
        self.newline()
        self.newline()

        self.outFile.write(generate_unhandled_command_comments(self.unhandled_commands.values()))

        def generate_vulkan_command_entries(
            indent: int,
            commands: dict[str, VulkanCommand],
            dispatch_infos: dict[str, CommandDispatchInfo],
            hooked_commands_var: str,
        ) -> str:
            command_to_dispatch_infos: dict[str, list[CommandDispatchInfo]] = {}
            for dispatch_info in dispatch_infos.values():
                for command in dispatch_info.commands:
                    command_dispatch_infos = command_to_dispatch_infos.setdefault(command, [])
                    command_dispatch_infos.append(dispatch_info)
            lines: list[str] = []
            command_items = sorted(commands.items(), key=lambda command: command[0])
            always_hooked_commands = [
                "vkEnumerateInstanceLayerProperties",
                "vkEnumerateInstanceExtensionProperties",
                "vkEnumerateDeviceLayerProperties",
                "vkEnumerateDeviceExtensionProperties",
                "vkGetInstanceProcAddr",
                "vkGetDeviceProcAddr",
                "vkCreateInstance",
                "vkDestroyInstance",
                "vkCreateDevice",
                "vkDestroyDevice",
                "vkEnumeratePhysicalDeviceGroups",
                "vkEnumeratePhysicalDeviceGroupsKHR",
                "vkEnumeratePhysicalDevices",
            ]
            for proc_name, command in command_items:
                # Use the actual name, because ash doesn't generate type names for aliased types.
                fp_type_name = f"vk::PFN_{command.vk_xml_command.name}"
                rust_fn_name = f"Self::{command.rust_fn.name}"
                features = [
                    dispatch_info.get_enum_name()
                    for dispatch_info in command_to_dispatch_infos[proc_name]
                ]
                assert (
                    len(features) > 0
                ), "Every command must have at least one dispatch_info associated."
                hook_command_variant_value = snake_case_to_upper_camel_case(command.rust_fn.name)
                hooked_expr: str = ""
                if proc_name in always_hooked_commands:
                    hooked_expr = "true"
                else:
                    hooked_expr = (
                        f"{hooked_commands_var}.contains("
                        f"&LayerVulkanCommand::{hook_command_variant_value})"
                    )
                lines += [
                    "VulkanCommand {",
                    f'    name: "{proc_name}",',
                    f'    features: smallvec![{", ".join(features)}],',
                    f"    hooked: {hooked_expr},",
                    (
                        f"    proc: unsafe {{ std::mem::transmute::<{fp_type_name},"
                        f" vk::PFN_vkVoidFunction>({rust_fn_name})}},"
                    ),
                    "},",
                ]
            return "".join([" " * indent + line + "\n" for line in lines])

        self.newline()

        self.outFile.write(
            "\n".join(
                [
                    "impl<T: Layer> Global<T> {",
                    (
                        "    pub(crate) fn create_device_commands(&self, instance_info:"
                        " &T::InstanceInfo, device_info: Option<&T::DeviceInfo>) ->"
                        " Box<[VulkanCommand]> {"
                    ),
                    "        let hooked_commands = self.layer_info",
                    "            .hooked_device_commands(instance_info, device_info)",
                    "            .collect::<HashSet<_>>();",
                    "        Box::new([",
                ]
                + [""]
            )
        )
        self.outFile.write(
            generate_vulkan_command_entries(
                12, self.device_commands, self.dispatch_infos, "hooked_commands"
            )
        )
        self.outFile.write("        ])\n")
        self.outFile.write("    }\n")

        self.outFile.write(
            "\n".join(
                [
                    (
                        "    pub(crate) fn create_instance_commands(&self, instance_info:"
                        " &T::InstanceInfo) -> Box<[VulkanCommand]> {"
                    ),
                    "        let hooked_commands = self.layer_info",
                    "            .hooked_instance_commands(instance_info)",
                    "            .collect::<HashSet<_>>();",
                    "        Box::new([",
                ]
                + [""]
            )
        )
        self.outFile.write(
            generate_vulkan_command_entries(
                12, self.instance_commands, self.dispatch_infos, "hooked_commands"
            )
        )
        self.outFile.write("        ])\n")
        self.outFile.write("    }\n")

        for vulkan_command in not_aliased_commands:
            if vulkan_command.vk_xml_command.name in self.manually_implemented_cmd:
                continue
            self.outFile.write(f"    {vulkan_command.rust_fn.get_def_str()} {{\n")
            impl = "".join(
                [
                    " " * 8 + line + "\n"
                    for line in vulkan_command.get_rust_fn_impl_lines(
                        list(self.dispatch_infos.values())
                    )
                ]
            )
            self.outFile.write(impl)
            self.outFile.write("    }\n")
        self.outFile.write("}\n")

        super().endFile()

    def genCmd(self, cmdinfo: reg.CmdInfo, name: str, alias: Optional[str]):
        super().genCmd(cmdinfo, name, alias)

        # Note that self.featureName is not accurate in genCmd: when handling
        # `vkGetRayTracingShaderGroupHandlesKHR` the feature name is `VK_NV_ray_tracing` instead of
        # `VK_KHR_ray_tracing_pipeline` so we choose to connect the dispatch info with the command
        # in endFile

        unhandled_command = GlobalSimpleInterceptGenerator.should_skip_cmd(name)
        if unhandled_command is not None:
            self.unhandled_commands[name] = unhandled_command
            return

        vk_xml_cmd = VkXmlCommand.from_cmd_info(cmdinfo, self.types)
        rust_ffi_function = RustFfiFunction.from_vk_xml_command(vk_xml_cmd)
        vulkan_command = VulkanCommand(
            vk_xml_command=vk_xml_cmd,
            rust_fn=rust_ffi_function,
            rust_method=RustMethod.from_vk_xml_command(vk_xml_cmd),
        )

        self.command_aliases.add_alias(name, alias)

        dispatch_chain_type = vk_xml_cmd.get_dispatch_chain_type()
        if dispatch_chain_type == DispatchChainType.INSTANCE:
            self.instance_commands[name] = vulkan_command
        elif dispatch_chain_type == DispatchChainType.DEVICE:
            self.device_commands[name] = vulkan_command
        elif dispatch_chain_type != DispatchChainType.GLOBAL:
            self.unhandled_commands[name] = UnhandledCommand(
                name=name, reason="Unknown dispatch chain type"
            )

    def beginFeature(self, interface, emit):
        super().beginFeature(interface, emit)

        assert (
            self.dispatch_infos.get(self.featureName) is None
        ), f"Duplicate entries for {self.featureName}"

        feature = None
        if self.featureName in self.registry.apidict:
            feature = self.registry.apidict[self.featureName]
        elif self.featureName in self.registry.extdict:
            feature = self.registry.extdict[self.featureName]
        else:
            assert False, f"Unknown feature name: {self.featureName}"

        def should_skip_command(cmd: str) -> bool:
            return GlobalSimpleInterceptGenerator.should_skip_cmd(cmd) is not None

        self.dispatch_infos[self.featureName] = CommandDispatchInfo.from_feature_info(
            feature, should_skip_command
        )

    def genType(self, typeinfo: reg.TypeInfo, name: str, alias):
        super().genType(typeinfo, name, alias)
        self.types[name] = typeinfo

    def genGroup(self, groupinfo: reg.GroupInfo, groupName: str, alias):
        super().genGroup(groupinfo, groupName, alias)
        self.types[groupName] = groupinfo


class ParamTransformer(ABC):
    @abstractmethod
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        raise NotImplementedError

    @abstractmethod
    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        raise NotImplementedError

    @staticmethod
    def create() -> ParamTransformer:
        return ComposedParamTransformer(
            [
                ArrayParamTransformer(),
                BoolParamTransformer(),
                PrimitiveParamTransformer(),
                NoLenPtrParamTransformer(),
                NullTerminatedLenPtrParamTransformer(),
                VoidDataSliceParamTransformer(),
                BoolSliceParamTransformer(),
                GeneralSliceParamTransformer(),
            ]
        )

    @staticmethod
    def get_ptr_to_slice_fn(vk_xml_type: VkXmlType) -> str:
        element_xml_type = vk_xml_type.points_to
        assert element_xml_type is not None
        if vk_xml_type.is_optional:
            if element_xml_type.is_const:
                return "maybe_slice_from_raw_parts"
            else:
                return "maybe_uninit_slice_from_raw_parts_mut"
        else:
            if element_xml_type.is_const:
                return "slice_from_raw_parts"
            else:
                return "uninit_slice_from_raw_parts_mut"


class BoolParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return rust_type.name == "bool" and vk_xml_type.name == "VkBool32"

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        input_var = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        return f"{input_var} == vk::TRUE"


class ArrayParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        if len(vk_xml_type.dimensions) == 0:
            return False
        if rust_type.refers_to is None:
            return False
        if rust_type.refers_to.array_info is None:
            return False
        if rust_type.refers_to.array_info.length != vk_xml_type.dimensions[0]:
            return False
        return True

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        assert not vk_xml_param.type.is_optional
        assert vk_xml_param.type.is_const
        assert vk_xml_param.name is not None
        rust_ffi_param_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        return f"unsafe {{ {rust_ffi_param_name}.as_ref() }}.unwrap()"


class PrimitiveParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return rust_type.name is not None

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        return RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)


class NoLenPtrParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return vk_xml_type.points_to is not None and vk_xml_type.len is None

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        assert vk_xml_param.type.points_to is not None
        ffi_param_var = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        if rust_param.type.points_to is not None:
            # In Rust, this parameter is still a pointer type, no transformation will be applied
            return ffi_param_var

        def get_ptr_to_opt_ref_expr() -> str:
            if vk_xml_param.type.points_to.is_const:
                return f"unsafe {{ {ffi_param_var}.as_ref() }}"
            return f"unsafe {{ ptr_as_uninit_mut({ffi_param_var}) }}"

        ptr_to_opt_ref_expr = get_ptr_to_opt_ref_expr()
        if vk_xml_param.type.is_optional:
            return ptr_to_opt_ref_expr
        else:
            return f"{ptr_to_opt_ref_expr}.unwrap()"


class NullTerminatedLenPtrParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return (
            vk_xml_type.len == VkXmlLenKind.NULL_TERMINATED
            and vk_xml_type.points_to is not None
            and vk_xml_type.points_to.name == "char"
            and vk_xml_type.points_to.is_const
            and not vk_xml_type.is_optional
            and rust_type.refers_to is not None
            and rust_type.refers_to.name == "str"
            and rust_type.refers_to.is_const
        )

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        rust_ffi_param_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        return f"unsafe {{ CStr::from_ptr({rust_ffi_param_name}) }}.to_str().unwrap()"


class BoolSliceParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return (
            vk_xml_type.len == VkXmlLenKind.VARIABLE
            and vk_xml_type.points_to is not None
            and vk_xml_type.points_to.name == "VkBool32"
            and vk_xml_type.points_to.is_const
            and not vk_xml_type.is_optional
        )

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        rust_ffi_param_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        vk_xml_params_by_name = {param.name: param for param in vk_xml_command.parameters}
        len_xml_param = vk_xml_params_by_name.get(vk_xml_param.len_var, None)
        assert len_xml_param is not None
        assert "->" not in vk_xml_param.len_var
        assert len_xml_param.type.name in ["uint32_t", "size_t", "VkDeviceSize"]
        rust_ffi_len_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.len_var)
        return (
            f"unsafe {{ bool_iterator_from_raw_parts({rust_ffi_param_name}, {rust_ffi_len_name}) }}"
        )


class VoidDataSliceParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        if vk_xml_type.len != VkXmlLenKind.VARIABLE:
            return False
        if vk_xml_type.points_to is None:
            return False
        if vk_xml_type.points_to.name != "void":
            return False
        return True

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        params_by_name = {param.name: param for param in vk_xml_command.parameters}
        len_var_xml_param = params_by_name.get(vk_xml_param.len_var)
        assert len_var_xml_param is not None
        assert not len_var_xml_param.type.is_optional
        ptr_to_slice_fn = ParamTransformer.get_ptr_to_slice_fn(vk_xml_param.type)
        rust_ffi_len_var_name = RustParam.param_name_from_vk_xml_param_for_ffi(
            len_var_xml_param.name
        )
        len_expr: str = rust_ffi_len_var_name
        if vk_xml_param.type.is_optional and not vk_xml_param.type.points_to.is_const:
            assert len_var_xml_param.type.points_to is not None
            assert len_var_xml_param.type.points_to.name in [
                "uint32_t",
                "size_t",
            ], f"Unsuppored length parameter {len_var_xml_param.name} in {vk_xml_command.name}"
        else:
            assert len_var_xml_param.type.name in ["uint32_t", "size_t", "VkDeviceSize"]
            len_expr = f"{rust_ffi_len_var_name}"

        rust_ffi_param_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        data_ptr_type = "*const u8" if vk_xml_param.type.points_to.is_const else "*mut u8"
        return (
            f"unsafe {{ {ptr_to_slice_fn}({rust_ffi_param_name} as {data_ptr_type}, {len_expr}) }}"
        )


class GeneralSliceParamTransformer(ParamTransformer):
    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        if vk_xml_type.len != VkXmlLenKind.VARIABLE:
            return False
        if vk_xml_type.points_to is None:
            return False
        return True

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        rust_ffi_param_name = RustParam.param_name_from_vk_xml_param_for_ffi(vk_xml_param.name)
        assert vk_xml_param.type.points_to.name is not None

        params_by_name = {param.name: param for param in vk_xml_command.parameters}

        def generate_len_expr():
            if (
                vk_xml_param.len_var == "(samples + 31) / 32"
                and vk_xml_command.name == "vkCmdSetSampleMaskEXT"
            ):
                return "(samples.as_raw() + 31) / 32"

            if vk_xml_param.type.is_optional and not vk_xml_param.type.points_to.is_const:
                # handle mutable optional slice
                # Only supported with a mutable pointer type length variable.
                assert vk_xml_param.len_var in params_by_name
                len_var_xml_param = params_by_name[vk_xml_param.len_var]
                assert len_var_xml_param.type.points_to is not None
                assert len_var_xml_param.type.points_to.name in [
                    "size_t",
                    "uint32_t",
                ], f"Unsupported length type: {len_var_xml_param.name} in {vk_xml_command.name}"
                return RustParam.param_name_from_vk_xml_param_for_ffi(len_var_xml_param.name)

            if "->" in vk_xml_param.len_var:
                # handle pBuildInfo->geometryCount in vkGetAccelerationStructureBuildSizesKHR.
                len_var_components = vk_xml_param.len_var.split("->")
                assert len(len_var_components) == 2, f"Unknown len: {vk_xml_param.len_var}"
                len_xml_param = params_by_name.get(len_var_components[0])
                assert len_xml_param is not None, "Can't find the length parameter."
                assert not len_xml_param.type.is_optional, (
                    f"When handling length parameter, {len_var_components[0]} in "
                    f"{vk_xml_param.len_var} is optional. This is currently not supported."
                )
                assert len_xml_param.type.points_to is not None
                assert len_xml_param.type.points_to.is_const
                len_var = RustParam.param_name_from_vk_xml_param_for_ffi(len_var_components[0])
                # Safe, because len_var is a const pointer and it's not optional, so it must point
                # to an initialized object.
                len_expr = f"unsafe {{ {len_var}.as_ref() }}"
                field_name = RustParam.param_name_from_vk_xml_param_for_ffi(len_var_components[1])
                return f"{len_expr}.unwrap().{field_name}"

            assert vk_xml_param.len_var in params_by_name
            len_var_xml_param = params_by_name[vk_xml_param.len_var]
            rust_ffi_len_var_name = RustParam.param_name_from_vk_xml_param_for_ffi(
                len_var_xml_param.name
            )

            if len_var_xml_param.type.name in ["uint32_t"]:
                return rust_ffi_len_var_name

            if len_var_xml_param.type.points_to.name in ["uint32_t"]:
                assert not len_var_xml_param.type.is_optional
                return f"*unsafe {{ {rust_ffi_len_var_name}.as_ref() }}.unwrap()"

            assert False, f"Unsupported length variable type: {len_var_xml_param.type}"

        len_expr = generate_len_expr()
        ptr_to_slice_fn = ParamTransformer.get_ptr_to_slice_fn(vk_xml_param.type)
        return f"unsafe {{ {ptr_to_slice_fn}({rust_ffi_param_name}, {len_expr}) }}"


class ComposedParamTransformer(ParamTransformer):
    def __init__(self, transformers: list[ParamTransformer]):
        super().__init__()
        self.transformers = transformers

    def type_matches(
        self, rust_type: RustType, vk_xml_type: VkXmlType, vk_xml_command: VkXmlCommand
    ) -> bool:
        return any(
            transformer.type_matches(rust_type, vk_xml_type, vk_xml_command)
            for transformer in self.transformers
        )

    def transform(
        self, rust_param: RustParam, vk_xml_param: VkXmlParam, vk_xml_command: VkXmlCommand
    ) -> str:
        transformer = next(
            (
                transformer
                for transformer in self.transformers
                if transformer.type_matches(rust_param.type, vk_xml_param.type, vk_xml_command)
            ),
            None,
        )
        assert transformer is not None
        return transformer.transform(rust_param, vk_xml_param, vk_xml_command)


class TestSimpleInterceptGenerator(unittest.TestCase):
    def test_bool_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdSetRepresentativeFragmentTestEnableNV")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            BoolParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[1].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[1], vk_xml_command
                ),
                "representative_fragment_test_enable == vk::TRUE",
            )

    def test_array_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdSetBlendConstants")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            ArrayParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[1].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[1], vk_xml_command
                ),
                "unsafe { blend_constants.as_ref() }.unwrap()",
            )

    def test_primitive_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdDraw")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            PrimitiveParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[0].type,
                    vk_xml_command.parameters[0].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[0], vk_xml_command.parameters[0], vk_xml_command
                ),
                "command_buffer",
            )
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[1].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[1], vk_xml_command
                ),
                "vertex_count",
            )

    def test_primitive_param_transformer_keyword_argument(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetPhysicalDeviceImageFormatProperties")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            PrimitiveParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[2], vk_xml_command
                ),
                "_type",
            )

    def test_c_expr_length_var_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetAccelerationStructureBuildSizesKHR")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            GeneralSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[3].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[3], vk_xml_command
                ),
                (
                    "unsafe { maybe_slice_from_raw_parts(p_max_primitive_counts, unsafe { "
                    "p_build_info.as_ref() }.unwrap().geometry_count) }"
                ),
            )

    def test_no_length_not_optional_const_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCreateImage")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            NoLenPtrParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[0].type,
                    vk_xml_command.parameters[1].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[0], vk_xml_command.parameters[1], vk_xml_command
                ),
                "unsafe { p_create_info.as_ref() }.unwrap()",
            )

    def test_no_length_optional_const_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCreateImage")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            NoLenPtrParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[2], vk_xml_command
                ),
                "unsafe { p_allocator.as_ref() }",
            )

    def test_null_terminated_length_const_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkDebugReportMessageEXT")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            NullTerminatedLenPtrParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[5].type,
                    vk_xml_command.parameters[6].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[5], vk_xml_command.parameters[6], vk_xml_command
                ),
                "unsafe { CStr::from_ptr(p_layer_prefix) }.to_str().unwrap()",
            )

    def test_boolean_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdSetColorWriteEnableEXT")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            BoolSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[2], vk_xml_command
                ),
                "unsafe { bool_iterator_from_raw_parts(p_color_write_enables, attachment_count) }",
            )

    def test_const_void_data_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdUpdateBuffer")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            VoidDataSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[3].type,
                    vk_xml_command.parameters[4].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[3], vk_xml_command.parameters[4], vk_xml_command
                ),
                "unsafe { slice_from_raw_parts(p_data as *const u8, data_size) }",
            )

    def test_mutable_void_data_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetQueryPoolResults")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            VoidDataSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[3].type,
                    vk_xml_command.parameters[5].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[3], vk_xml_command.parameters[5], vk_xml_command
                ),
                "unsafe { uninit_slice_from_raw_parts_mut(p_data as *mut u8, data_size) }",
            )

    def test_required_const_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkQueueSubmit")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            GeneralSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[2], vk_xml_command
                ),
                "unsafe { slice_from_raw_parts(p_submits, submit_count) }",
            )

    def test_required_mutable_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetCalibratedTimestampsEXT")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            GeneralSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[3].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[3], vk_xml_command
                ),
                "unsafe { uninit_slice_from_raw_parts_mut(p_timestamps, timestamp_count) }",
            )

    def test_optional_mutable_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command(
            "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"
        )
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            GeneralSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[3].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[3], vk_xml_command
                ),
                "unsafe { maybe_uninit_slice_from_raw_parts_mut(p_counters, p_counter_count) }",
            )

    def test_cmd_set_sample_mask_ext_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkCmdSetSampleMaskEXT")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            GeneralSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[2], vk_xml_command
                ),
                "unsafe { slice_from_raw_parts(p_sample_mask, (samples.as_raw() + 31) / 32) }",
            )

    def test_no_length_not_optional_mutable_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetPhysicalDeviceFeatures2")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            NoLenPtrParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[1].type,
                    vk_xml_command.parameters[1].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[1], vk_xml_command.parameters[1], vk_xml_command
                ),
                "unsafe { ptr_as_uninit_mut(p_features) }.unwrap()",
            )

    def test_no_length_not_optional_mutable_opaque_type_ptr_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command(
            "vkGetPhysicalDeviceXcbPresentationSupportKHR"
        )
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            NoLenPtrParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[2].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[2], vk_xml_command
                ),
                "connection",
            )

    def test_optional_mutable_void_slice_param_transformer(self):
        vk_xml_command = TestUtils.get_vk_xml_command("vkGetPipelineCacheData")
        rust_method = RustMethod.from_vk_xml_command(vk_xml_command)
        param_transformers: list[ParamTransformer] = [
            VoidDataSliceParamTransformer(),
            ParamTransformer.create(),
        ]
        for param_transformer in param_transformers:
            self.assertTrue(
                param_transformer.type_matches(
                    rust_method.parameters[2].type,
                    vk_xml_command.parameters[3].type,
                    vk_xml_command,
                )
            )
            self.assertEqual(
                param_transformer.transform(
                    rust_method.parameters[2], vk_xml_command.parameters[3], vk_xml_command
                ),
                "unsafe { maybe_uninit_slice_from_raw_parts_mut(p_data as *mut u8, p_data_size) }",
            )
