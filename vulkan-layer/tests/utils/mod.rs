// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ash::vk::{self, Handle};
use std::{
    collections::HashSet,
    ffi::{c_char, c_void, CStr},
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan_layer::{
    test_utils::{
        TestLayerWrapper, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink,
    },
    Global, Layer,
};

pub type MockLayer<T> = Arc<TestLayerWrapper<T>>;

pub fn create_entry<T: Layer>() -> ash::Entry {
    unsafe {
        ash::Entry::from_static_fn(vk::StaticFn {
            get_instance_proc_addr: Global::<T>::get_instance_proc_addr,
        })
    }
}

pub struct Context {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub next_entry: ash::Entry,
}

pub trait InstanceCreateInfoExt {
    fn with_instance<T: Layer>(self, f: impl FnOnce(&Context));
}

impl InstanceCreateInfoExt for vk::InstanceCreateInfoBuilder<'_> {
    fn with_instance<T: Layer>(self, f: impl FnOnce(&Context)) {
        extern "system" fn get_instance_proc_addr(
            instance: vk::Instance,
            p_name: *const i8,
        ) -> vk::PFN_vkVoidFunction {
            #[repr(C)]
            struct InstanceData {
                _dispatch_table: *const c_void,
                version: u32,
                enabled_extensions: HashSet<String>,
            }
            extern "system" fn create_instance(
                create_info: *const vk::InstanceCreateInfo,
                _: *const vk::AllocationCallbacks,
                instance: *mut vk::Instance,
            ) -> vk::Result {
                let create_info = unsafe { create_info.as_ref() }.unwrap();
                let application_info = unsafe { create_info.p_application_info.as_ref() };
                let enabled_extensions = unsafe {
                    std::slice::from_raw_parts(
                        create_info.pp_enabled_extension_names,
                        create_info.enabled_extension_count.try_into().unwrap(),
                    )
                }
                .iter()
                .map(|enabled_extension| {
                    unsafe { CStr::from_ptr(*enabled_extension) }
                        .to_owned()
                        .into_string()
                        .unwrap()
                })
                .collect::<_>();
                let inner: Box<InstanceData> = Box::new(InstanceData {
                    _dispatch_table: 1234 as *const c_void,
                    version: application_info
                        .map(|application_info| application_info.api_version)
                        .unwrap_or(vk::API_VERSION_1_0),
                    enabled_extensions,
                });
                *unsafe { instance.as_mut() }.unwrap() =
                    vk::Instance::from_raw(Box::leak(inner) as *mut _ as _);
                vk::Result::SUCCESS
            }
            extern "system" fn destroy_instance(
                instance: vk::Instance,
                _: *const vk::AllocationCallbacks,
            ) {
                let inner = unsafe { Box::from_raw(instance.as_raw() as *mut InstanceData) };
                drop(inner);
            }
            extern "system" fn get_physical_device_sparse_image_format_properties2(
                _: vk::PhysicalDevice,
                _: *const vk::PhysicalDeviceSparseImageFormatInfo2,
                _: *mut u32,
                _: *mut vk::SparseImageFormatProperties2,
            ) {
                unimplemented!()
            }
            extern "system" fn destroy_surface_khr(
                _: vk::Instance,
                surface: vk::SurfaceKHR,
                _: *const vk::AllocationCallbacks,
            ) {
                if surface == vk::SurfaceKHR::null() {
                    return;
                }
                unimplemented!()
            }
            extern "system" fn enumerate_device_layer_properties(
                _: vk::PhysicalDevice,
                _property_count: *mut u32,
                _properties: *mut vk::LayerProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn enumerate_device_extension_properties(
                _: vk::PhysicalDevice,
                _layer_name: *const c_char,
                _property_count: *mut u32,
                _properties: *mut vk::ExtensionProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn get_device_proc_addr(
                _: vk::Device,
                _name: *const c_char,
            ) -> vk::PFN_vkVoidFunction {
                unimplemented!()
            }
            extern "system" fn enumerate_physical_devices(
                _: vk::Instance,
                _physical_device_count: *mut u32,
                _physical_devices: *mut vk::PhysicalDevice,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn enumerate_physical_device_groups(
                _: vk::Instance,
                _physical_device_group_count: *mut u32,
                _phyiscal_device_group_properties: *mut vk::PhysicalDeviceGroupProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn create_device(
                _: vk::PhysicalDevice,
                _: *const vk::DeviceCreateInfo,
                _: *const vk::AllocationCallbacks,
                _: *mut vk::Device,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn destroy_device(_: vk::Device, _: *const vk::AllocationCallbacks) {
                unimplemented!()
            }
            let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
            if instance == vk::Instance::null() {
                match name {
                    "vkCreateInstance" => Some(unsafe {
                        std::mem::transmute(create_instance as vk::PFN_vkCreateInstance)
                    }),
                    _ => None,
                }
            } else {
                let instance_data =
                    unsafe { (instance.as_raw() as *const InstanceData).as_ref() }.unwrap();
                match name {
                    "vkDestroyDevice" => Some(unsafe {
                        std::mem::transmute(destroy_device as vk::PFN_vkDestroyDevice)
                    }),
                    "vkCreateDevice" => Some(unsafe {
                        std::mem::transmute(create_device as vk::PFN_vkCreateDevice)
                    }),
                    "vkEnumeratePhysicalDeviceGroups" => {
                        if instance_data.version >= vk::API_VERSION_1_1 {
                            Some(unsafe {
                                std::mem::transmute(
                                    enumerate_physical_device_groups
                                        as vk::PFN_vkEnumeratePhysicalDeviceGroups,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    "vkEnumeratePhysicalDevices" => Some(unsafe {
                        std::mem::transmute(
                            enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices,
                        )
                    }),
                    "vkGetDeviceProcAddr" => Some(unsafe {
                        std::mem::transmute(get_device_proc_addr as vk::PFN_vkGetDeviceProcAddr)
                    }),
                    "vkGetInstanceProcAddr" => Some(unsafe {
                        std::mem::transmute(get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr)
                    }),
                    "vkEnumerateDeviceLayerProperties" => Some(unsafe {
                        std::mem::transmute(
                            enumerate_device_layer_properties
                                as vk::PFN_vkEnumerateDeviceLayerProperties,
                        )
                    }),
                    "vkEnumerateDeviceExtensionProperties" => Some(unsafe {
                        std::mem::transmute(
                            enumerate_device_extension_properties
                                as vk::PFN_vkEnumerateDeviceExtensionProperties,
                        )
                    }),
                    "vkDestroyInstance" => Some(unsafe {
                        std::mem::transmute(destroy_instance as vk::PFN_vkDestroyInstance)
                    }),
                    "vkGetPhysicalDeviceSparseImageFormatProperties2" => {
                        if instance_data.version >= vk::API_VERSION_1_1 {
                            Some(unsafe {
                                std::mem::transmute(
                                    get_physical_device_sparse_image_format_properties2
                                        as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    "vkDestroySurfaceKHR" => {
                        if instance_data
                            .enabled_extensions
                            .contains(vk::KhrSurfaceFn::name().to_str().unwrap())
                        {
                            Some(unsafe {
                                std::mem::transmute(
                                    destroy_surface_khr as vk::PFN_vkDestroySurfaceKHR,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
        let entry = create_entry::<T>();
        let mut layer_link = VkLayerInstanceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: get_instance_proc_addr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        };
        let mut layer_create_info = VkLayerInstanceCreateInfo {
            sType: vk::StructureType::LOADER_INSTANCE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        *unsafe { layer_create_info.u.pLayerInfo.as_mut() } = &mut layer_link as *mut _;

        let create_instance_info = self.push_next(&mut layer_create_info);
        let instance = unsafe { entry.create_instance(&create_instance_info, None) }.unwrap();
        assert_ne!(instance.handle(), vk::Instance::null());
        let context = Context {
            entry,
            instance,
            next_entry: unsafe {
                ash::Entry::from_static_fn(vk::StaticFn {
                    get_instance_proc_addr,
                })
            },
        };
        f(&context);
        unsafe { context.instance.destroy_instance(None) };
    }
}
