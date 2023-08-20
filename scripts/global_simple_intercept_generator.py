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
    decayed_type_to_rust_type,
    UnhandledCommand,
    generate_unhandled_command_comments,
    VkXmlLenKind,
    VkXmlParam,
    opaque_type_map,
    VulkanAliases,
    write_license,
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

    def __find_param_by_name(self, name: str) -> Optional[tuple[VkXmlParam, RustParam]]:
        all_params = zip(self.vk_xml_command.parameters, self.rust_fn.parameters)
        return next((param for param in all_params if param[0].name == name), None)

    def __generate_ptr_param_expr(self, param_index: int) -> tuple[list[str], str]:
        assert param_index < len(self.vk_xml_command.parameters), "Parameter index out of range."
        xml_param = self.vk_xml_command.parameters[param_index]
        rust_param = self.rust_fn.parameters[param_index]
        assert xml_param.type.points_to is not None, "Can only handle pointer type parameter."
        if xml_param.type.len == VkXmlLenKind.VARIABLE:
            ptr_to_slice_fn = "std::slice::from_raw_parts"
            if not xml_param.type.points_to.is_const:
                ptr_to_slice_fn = "std::slice::from_raw_parts_mut"
            assert xml_param.len_var is not None, "Must has a len variable"
            len_exp: str = None
            if (
                xml_param.len_var == "(samples + 31) / 32"
                and self.vk_xml_command.name == "vkCmdSetSampleMaskEXT"
            ):
                len_exp = "((samples.as_raw() + 31) / 32)"
            elif "->" in xml_param.len_var:
                # handle cases similar to pAllocateInfo->descriptorSetCount in
                # vkAllocateDescriptorSets
                len_var_components = xml_param.len_var.split("->")
                assert len(len_var_components) == 2, f"Unknown len: {xml_param.len_var}"
                len_param = self.__find_param_by_name(len_var_components[0])
                assert len_param is not None, "Can't find the length parameter."
                len_xml_param, len_rust_param = len_param
                assert not len_xml_param.type.is_optional, (
                    f"When handling length parameter, {len_var_components[0]} in "
                    f"{xml_param.len_var} is optional. This is currently not supported."
                )
                len_exp = f"unsafe {{ {len_rust_param.name}.as_ref() }}"
                len_exp = f"{len_exp}.unwrap().{camel_case_to_snake_case(len_var_components[1])}"
            else:
                len_param = self.__find_param_by_name(xml_param.len_var)
                assert len_param is not None, (
                    f"Can't find len var {xml_param.len_var} for "
                    f"{xml_param.name} in {self.vk_xml_command.name}"
                )
                len_xml_param, len_rust_param = len_param
                len_param_rust_type = len_rust_param.type
                if len_param_rust_type.name in ["u32", "usize", "vk::DeviceSize"]:
                    assert not len_param_rust_type.is_optional, (
                        f"Primitive length {len_rust_param.name} in {self.vk_xml_command.name} "
                        "can't be optional."
                    )
                    len_exp = len_rust_param.name
                elif (
                    len_param_rust_type.points_to is not None
                    and len_param_rust_type.points_to.name in ["u32", "usize", "vk::DeviceSize"]
                ):
                    len_param_xml_type = len_xml_param.type
                    assert not len_param_xml_type.is_optional, (
                        f"Pointer length {len_xml_param.name} in {self.vk_xml_command.name} can't "
                        "be optional."
                    )
                    len_exp = f"*unsafe {{ {len_rust_param.name}.as_ref() }}.unwrap()"
                else:
                    assert (
                        False
                    ), f"Unknown type for length variable: {len_param_rust_type.to_string()}"
            data_ptr_expr = rust_param.name
            if xml_param.type.points_to.name == "void":
                mut_mod = "const" if xml_param.type.points_to.is_const else "mut"
                data_ptr_expr = f"{rust_param.name} as *{mut_mod} u8"
            res = f"unsafe {{ {ptr_to_slice_fn}({data_ptr_expr}, {len_exp} as usize) }}"
            # ABI change array.
            if rust_param.type.points_to.name == "vk::Bool32":
                res = f"{res}.iter().map(|v| *v == vk::TRUE)"
            if xml_param.type.is_optional:
                res = f"if {rust_param.name}.is_null() {{ None }} else {{ Some({res}) }}"
            return (["#[allow(clippy::unnecessary_cast)]"], res)
        elif xml_param.type.len == VkXmlLenKind.NULL_TERMINATED:
            param_is_c_string = (
                xml_param.type.points_to is not None
                and xml_param.type.points_to.name == "char"
                and xml_param.type.points_to.is_const
            )
            assert param_is_c_string, "Null terminated param must be a C string."
            assert not xml_param.type.is_optional, (
                f"{xml_param.name} in {self.vk_xml_command.name} is an optional C string. "
                "Unsupported."
            )
            return ([], f"unsafe {{ CStr::from_ptr({rust_param.name}) }}.to_str().unwrap()")
        elif xml_param.type.len is None:
            to_ref_fn = "as_ref" if xml_param.type.points_to.is_const else "as_mut"
            res = f"unsafe {{ {rust_param.name}.{to_ref_fn}() }}"

            if not xml_param.type.is_optional:
                res = f"{res}.unwrap()"

            return ([], res)
        else:
            assert False, f"Unknown len type: {xml_param.type.len}"

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
        ), f"No dipatch info found for command {self.vk_xml_command.name}"
        lines += [
            (
                f"let dispatch_table = &{dispatch_chain_var}.dispatch_table."
                f"{dispatch_info.get_dispatch_table_field_name()};"
            ),
        ]
        len_params = set(
            (param.len_var for param in self.vk_xml_command.parameters if param.len_var is not None)
        )
        all_params: list[tuple[VkXmlParam, RustParam]] = list(
            zip(self.vk_xml_command.parameters, self.rust_fn.parameters)
        )
        params: list[tuple[int, VkXmlParam, RustParam]] = []

        for i, param in enumerate(all_params):
            xml_param, _ = param
            if xml_param.name in len_params:
                continue
            if (
                i == 0
                and xml_param.type.name is not None
                and xml_param.type.name in ["VkInstance", "VkDevice"]
            ):
                continue
            params.append((i, *param))

        ret_param: Optional[tuple[int, VkXmlParam, RustParam]] = None
        if self.vk_xml_command.return_type in ["VkResult", "void"]:
            # Find the last not length parameter:
            reversed_params = params.copy()
            reversed_params.reverse()
            last_not_len_param = next(
                (param for _, param, _ in reversed_params if param.name not in len_params), None
            )
            is_returned = False
            if (
                last_not_len_param is not None
                and not last_not_len_param.type.decayed_type().is_struct()
            ):
                param_type = last_not_len_param.type
                is_returned = param_type.points_to is not None and not param_type.points_to.is_const
                # pointers to opaque type won't be part of the return value while array types are
                # always returned.
                is_returned = is_returned and (
                    param_type.points_to.name not in opaque_type_map
                    or param_type.len == VkXmlLenKind.VARIABLE
                )
            if is_returned:
                # The last parameter is a return value.
                ret_param = params.pop()

        intercept_params: list[str] = []
        for i, xml_param, rust_param in params:
            arg_exp: str = None
            if len(xml_param.type.dimensions) > 0:
                lines.append(
                    f"let {rust_param.name} = unsafe {{ {rust_param.name}.as_ref() }}.unwrap();"
                )
            if xml_param.type.name is not None:
                arg_exp = rust_param.name
                if xml_param.type.name == "VkBool32":
                    arg_exp = f"{arg_exp} == vk::TRUE"
            elif xml_param.type.points_to is not None:
                leading_lines, arg_exp = self.__generate_ptr_param_expr(i)
                lines += leading_lines
            else:
                assert False, "Should have exausted all parameter types."
            intercept_params.append(arg_exp)

        def generate_ret_expr(ret_var: str) -> list[str]:
            return [ret_var]

        if self.vk_xml_command.return_type == "VkResult":
            if ret_param is not None:

                def generate_ret_expr(ret_var):  # noqa: F811
                    assign_to_ret_param_lines = self.__generate_assign_to_ret_param_lines(
                        "res", ret_param[0], last_expr_type="vk::Result"
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

            else:

                def generate_ret_expr(ret_var) -> list[str]:
                    return [
                        f"match {ret_var} {{",
                        "    Ok(()) => vk::Result::SUCCESS,",
                        "    Err(e) => e,",
                        "}",
                    ]

        elif self.vk_xml_command.return_type == "void":
            if ret_param is not None:

                def generate_ret_expr(ret_var) -> list[str]:
                    return self.__generate_assign_to_ret_param_lines(ret_var, ret_param[0])

        elif self.vk_xml_command.return_type == "VkBool32":

            def generate_ret_expr(ret_var):
                return [f"if {ret_var} {{ vk::TRUE }} else {{ vk::FALSE }}"]

        param_names = [param.name for param in self.rust_fn.parameters]
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
                    f"{', '.join(param_names)}) }},"
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

        write_license(self.outFile)
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
                        "use crate::{DeviceInfo, fill_vk_out_array, Global, InstanceInfo, Layer,"
                        " LayerResult, LayerVulkanCommand, InstanceHooks, DeviceHooks};"
                    ),
                    (
                        "use super::{get_instance_proc_addr_loader, get_device_proc_addr_loader, "
                        "VulkanCommand, TryFromExtensionError, ApiVersion, Feature};"
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
                # Use the actual name, because ash doesn't generate type names for alised types.
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
                    f"    proc: unsafe {{ std::mem::transmute({rust_fn_name} as {fp_type_name})}},",
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
        vulkan_command = VulkanCommand(vk_xml_command=vk_xml_cmd, rust_fn=rust_ffi_function)

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
