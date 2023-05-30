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

from generator import OutputGenerator
import reg
import sys
from typing import Optional
from vk_xml_util import (VkXmlCommand, RustMethod, DispatchChainType,
                         UnhandledCommand, generate_unhandled_command_comments, VulkanAliases,
                         write_license)


class LayerTraitGenerator(OutputGenerator):

    def __init__(self, err_file=sys.stderr, warn_file=sys.stderr, diag_file=sys.stdout):
        super().__init__(err_file, warn_file, diag_file)
        self.all_commands: dict[DispatchChainType, dict[str, RustMethod]] = {}
        self.command_aliases: VulkanAliases = VulkanAliases()
        self.unhandled_commands: dict[str, UnhandledCommand] = {}
        self.types: dict[str, reg.TypeInfo | reg.GroupInfo] = {}

    def beginFile(self, gen_opts):
        super().beginFile(gen_opts)
        write_license(self.outFile)
        self.newline()
        self.outFile.write("// This file is generated from the Vulkan XML API registry.\n")
        self.outFile.write("#![allow(clippy::too_many_arguments)]\n")
        self.outFile.write('\n'.join([
            "use std::ffi::{c_int, c_void};",
            "use std::sync::Arc;",
            "",
            "use ash::{vk, prelude::VkResult};",
            "",
            "use super::LayerResult;"
        ]))
        self.newline()
        self.newline()

    def endFile(self):
        flat_all_commands: dict[str, RustMethod] = {}
        not_aliased_commands: set[str] = set()
        for commands in self.all_commands.values():
            flat_all_commands |= commands
        for command in flat_all_commands:
            represent_name = self.command_aliases.get_represent_name(command)
            assert represent_name is not None, f"{command} is never added to the command aliases"
            if represent_name == command:
                not_aliased_commands.add(command)

        self.outFile.write(generate_unhandled_command_comments(self.unhandled_commands.values()))

        dispatch_chain_type_to_lines: dict[DispatchChainType, list[str]] = {
            DispatchChainType.GLOBAL: [
                "pub trait Layer: 'static + Sync + Default {",
                "    const LAYER_NAME: &'static str;",
                "    const SPEC_VERSION: u32;",
                "    const IMPLEMENTATION_VERSION: u32;",
                "    const LAYER_DESCRIPTION: &'static str;",
                "",
                "    type InstanceInfo: InstanceInfo;",
                "    type DeviceInfo: DeviceInfo;",
                "",
                "    fn create_instance_info(",
                "        &self,",
                "        create_info: &vk::InstanceCreateInfo,",
                "        allocator: Option<&vk::AllocationCallbacks>,",
                "        instance: Arc<ash::Instance>,",
                "    ) -> Self::InstanceInfo;",
                "",
                "    fn create_device_info(",
                "        &self,",
                "        physical_device: vk::PhysicalDevice,",
                "        create_info: &vk::DeviceCreateInfo,",
                "        allocator: Option<&vk::AllocationCallbacks>,",
                "        device: Arc<ash::Device>,",
                "    ) -> Self::DeviceInfo;",
                "",
            ],
            DispatchChainType.INSTANCE: ["pub trait InstanceInfo: Send + Sync {"],
            DispatchChainType.DEVICE: ["pub trait DeviceInfo: Send + Sync {"],
        }

        for dispatch_type, commands in self.all_commands.items():
            for name, command in commands.items():
                if name not in not_aliased_commands:
                    continue
                dispatch_chain_type_to_lines[dispatch_type] += [
                    f'    {command.to_string()} {{',
                    f'        LayerResult::Unhandled',
                    f'    }}',
                ]

        for dispatch_chain_type in sorted(list(dispatch_chain_type_to_lines.keys())):
            lines = dispatch_chain_type_to_lines[dispatch_chain_type]
            self.outFile.write('\n'.join(lines))
            self.newline()
            self.outFile.write("}\n")
            self.newline()

        super().endFile()

    def genCmd(self, cmdinfo: reg.CmdInfo, name: str, alias: Optional[str]):
        super().genCmd(cmdinfo, name, alias)

        unhandled_command = UnhandledCommand.find(name)
        if unhandled_command is not None:
            self.unhandled_commands[unhandled_command.name] = unhandled_command
            return

        should_skip = [
            # vkGet*ProcAddr should only be handled in the layer framework level, so they should not
            # be able to be implemented in the layer trait
            "vkGetInstanceProcAddr",
            "vkGetDeviceProcAddr",
            # Should be intercepted through Drop trait of InstanceInfo and DeviceInfo
            "vkDestroyInstance",
            "vkDestroyDevice",
            # Should be intercepted when the PhysicalDeviceInfo is created
            "vkEnumeratePhysicalDevices",
            "vkEnumeratePhysicalDeviceGroups",
            # Layer and extension properties should be provided via trait consts
            "vkEnumerateDeviceLayerProperties",
            "vkEnumerateDeviceExtensionProperties",
            # TODO: pre-instance functions are not supported now, but such support should be added.
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkEnumerateInstanceVersion",
        ]
        if name in should_skip:
            return

        vk_xml_cmd = VkXmlCommand.from_cmd_info(cmdinfo, self.types)
        dispatch_chain_type = vk_xml_cmd.get_dispatch_chain_type()

        if dispatch_chain_type is None:
            self.unhandled_commands[name] = UnhandledCommand(
                name=name, reason="Unknown dispatch chain type")
            return

        self.command_aliases.add_alias(name, alias)
        commands = self.all_commands.setdefault(dispatch_chain_type, {})
        rust_method = RustMethod.from_vk_xml_command(vk_xml_cmd)
        commands[name] = rust_method

    def genType(self, typeinfo: reg.TypeInfo, name: str, alias):
        super().genType(typeinfo, name, alias)
        self.types[name] = typeinfo

    def genGroup(self, groupinfo: reg.GroupInfo, groupName: str, alias):
        super().genGroup(groupinfo, groupName, alias)
        self.types[groupName] = groupinfo
