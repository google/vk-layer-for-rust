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
    ffi::{c_char, c_void, CStr, CString},
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan_layer::{
    test_utils::{
        TestLayer, TestLayerWrapper, VkLayerFunction, VkLayerInstanceCreateInfo,
        VkLayerInstanceLink,
    },
    Global, Layer, LayerResult, LayerVulkanCommand,
};

type MockLayer<T> = Arc<TestLayerWrapper<T>>;

fn create_entry<T: Layer>() -> ash::Entry {
    unsafe {
        ash::Entry::from_static_fn(vk::StaticFn {
            get_instance_proc_addr: Global::<T>::get_instance_proc_addr,
        })
    }
}

struct Context {
    entry: ash::Entry,
    instance: ash::Instance,
    next_entry: ash::Entry,
}

trait InstanceCreateInfoExt {
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

mod get_instance_proc_addr {
    use super::*;
    mod null_instance {
        use vulkan_layer::test_utils::TestLayer;

        use super::*;
        #[test]
        fn test_should_return_fp_when_called_with_get_instance_proc_addr_name() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let entry = create_entry::<MockLayer<Tag>>();
            let get_instance_proc_addr_name = b"vkGetInstanceProcAddr\0".as_ptr() as *const i8;
            let get_instance_proc_addr = unsafe {
                entry.get_instance_proc_addr(vk::Instance::null(), get_instance_proc_addr_name)
            }
            .expect("fp should be returned");
            // Verify that this is a function pointer that can be called.
            let get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr =
                unsafe { std::mem::transmute(get_instance_proc_addr) };
            unsafe { get_instance_proc_addr(vk::Instance::null(), get_instance_proc_addr_name) };
        }

        #[test]
        fn test_should_return_fp_when_called_with_global_command_name() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            type Layer = MockLayer<Tag>;
            let entry = create_entry::<Layer>();

            let layer_name = CString::new(Layer::LAYER_NAME).unwrap();
            entry
                .enumerate_instance_extension_properties(Some(&layer_name))
                .unwrap();
            entry.enumerate_instance_layer_properties().unwrap();
            // Verify if the returned function pointer can be called in other tests.
            unsafe {
                entry.get_instance_proc_addr(
                    vk::Instance::null(),
                    b"vkCreateInstance\0".as_ptr() as *const i8,
                )
            }
            .expect("vkCreateInstance should be a valid function pointer.");
        }

        #[test]
        fn test_should_return_null_when_called_with_core_instance_dispatchable_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let entry = create_entry::<MockLayer<Tag>>();
            assert!(unsafe {
                entry.get_instance_proc_addr(
                    vk::Instance::null(),
                    b"vkDestroyInstance\0".as_ptr() as *const i8,
                )
            }
            .is_none());

            // TODO: when we allow to intercept vkDestroyInstance also add a layer that intercepts
            // vkDestroyInstance in this test
        }

        #[test]
        #[ignore]
        fn test_should_return_null_when_called_with_available_instance_extension_command() {
            todo!(concat!(
                "This test is meaningless until we implement the mechanism for the layer to add ",
                "extensions"
            ))
        }

        #[test]
        #[ignore]
        fn test_should_return_null_when_called_with_available_device_extension_commands() {
            todo!(concat!(
                "This test is meaningless until we implement the mechanism for the layer to add ",
                "extensions"
            ))
        }
    }

    mod valid_instance {

        use ash::vk::ApplicationInfo;

        use super::*;

        #[test]
        fn test_should_return_null_for_global_commands() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            vk::InstanceCreateInfo::builder().with_instance::<MockLayer<Tag>>(
                |Context {
                     entry, instance, ..
                 }| {
                    let global_commands = [
                        "vkEnumerateInstanceVersion",
                        "vkEnumerateInstanceExtensionProperties",
                        "vkEnumerateInstanceLayerProperties",
                        "vkCreateInstance",
                    ];
                    for global_command in &global_commands {
                        let global_command = CString::new(global_command.to_owned()).unwrap();
                        let proc_addr = unsafe {
                            entry.get_instance_proc_addr(instance.handle(), global_command.as_ptr())
                        };
                        assert!(
                            proc_addr.is_none(),
                            "{} should be null",
                            global_command.to_string_lossy()
                        );
                    }
                },
            );
            // TODO: when we allow to intercept the global commands also add a layer that intercepts
            // the global commands and test.
        }

        #[test]
        fn test_should_return_fp_for_get_instance_proc_addr() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            vk::InstanceCreateInfo::builder().with_instance::<MockLayer<Tag>>(
                |Context {
                     entry, instance, ..
                 }| {
                    let get_instance_proc_addr_name =
                        b"vkGetInstanceProcAddr\0".as_ptr() as *const i8;
                    let get_instance_proc_addr = unsafe {
                        entry.get_instance_proc_addr(instance.handle(), get_instance_proc_addr_name)
                    }
                    .unwrap();
                    let get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr =
                        unsafe { std::mem::transmute(get_instance_proc_addr) };
                    assert_eq!(
                        get_instance_proc_addr as usize,
                        entry.static_fn().get_instance_proc_addr as usize
                    );
                },
            );
        }

        #[test]
        fn test_should_return_fp_for_core_dispatchable_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            vk::InstanceCreateInfo::builder().with_instance::<MockLayer<Tag>>(
                |Context {
                     entry, instance, ..
                 }| {
                    let destroy_instance = unsafe {
                        entry.get_instance_proc_addr(
                            instance.handle(),
                            b"vkDestroyInstance\0".as_ptr() as *const i8,
                        )
                    };
                    assert!(destroy_instance.is_some());
                    // vkDestroyInstance will be called in with_instance after this function is
                    // returned.
                },
            );
        }

        #[test]
        fn test_should_return_next_proc_addr_for_not_intercepted_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let enabled_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
            vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&enabled_extensions)
                .with_instance::<MockLayer<Tag>>(
                    |Context {
                         entry,
                         instance,
                         next_entry,
                         ..
                     }| {
                        let destroy_surface_name = b"vkDestroySurfaceKHR\0".as_ptr() as *const i8;
                        let destroy_surface = unsafe {
                            entry.get_instance_proc_addr(instance.handle(), destroy_surface_name)
                        }
                        .map(|fp| fp as usize);
                        // We don't wrap the object, so the VkInstance should be the same.
                        let next_destroy_surface = unsafe {
                            next_entry
                                .get_instance_proc_addr(instance.handle(), destroy_surface_name)
                        }
                        .map(|fp| fp as usize);
                        assert_eq!(destroy_surface, next_destroy_surface);
                    },
                );
        }

        #[test]
        fn test_should_return_fp_for_enabled_instance_extension_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}

            let application_info = ApplicationInfo::builder().api_version(vk::API_VERSION_1_1);
            vk::InstanceCreateInfo::builder()
                .application_info(&application_info)
                .with_instance::<MockLayer<Tag>>(
                    |Context {
                         entry, instance, ..
                     }| {
                        let fp = unsafe {
                            entry.get_instance_proc_addr(
                                instance.handle(),
                                b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr()
                                    as *const i8,
                            )
                        };
                        assert!(fp.is_some());
                    },
                );

            let enabled_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
            #[derive(Default)]
            struct Tag2;
            impl TestLayer for Tag2 {
                fn hooked_commands(&self) -> Box<dyn Iterator<Item = LayerVulkanCommand> + '_> {
                    Box::new(vec![LayerVulkanCommand::DestroySurfaceKhr].into_iter())
                }
            }
            vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&enabled_extensions)
                .with_instance::<MockLayer<Tag2>>(
                    |Context {
                         entry, instance, ..
                     }| {
                        let destroy_surface = unsafe {
                            entry.get_instance_proc_addr(
                                instance.handle(),
                                b"vkDestroySurfaceKHR\0".as_ptr() as *const i8,
                            )
                        };
                        let destroy_surface: vk::PFN_vkDestroySurfaceKHR =
                            unsafe { std::mem::transmute(destroy_surface.unwrap()) };
                        let instance_hooks_mock = Global::<MockLayer<Tag2>>::instance()
                            .layer_info
                            .get_instance_hooks_mock(instance.handle())
                            .unwrap();
                        instance_hooks_mock
                            .lock()
                            .unwrap()
                            .expect_destroy_surface_khr()
                            .times(1)
                            .return_const(LayerResult::Unhandled);
                        unsafe {
                            destroy_surface(instance.handle(), vk::SurfaceKHR::null(), null())
                        };
                    },
                )
            // TODO: when we allow customize layer extensions, also test with the extension provided
            // by the layer
        }

        #[test]
        fn test_if_extension_is_not_enabled_null_should_be_returned_for_not_hooked_proc() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            vk::InstanceCreateInfo::builder().with_instance::<MockLayer<Tag>>(
                |Context {
                     entry, instance, ..
                 }| {
                    let fp = unsafe {
                        entry.get_instance_proc_addr(
                            instance.handle(),
                            b"vkDestroySurfaceKHR\0".as_ptr() as *const i8,
                        )
                    };
                    assert!(fp.is_none());
                    let fp = unsafe {
                        entry.get_instance_proc_addr(
                            instance.handle(),
                            b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr()
                                as *const i8,
                        )
                    };
                    assert!(fp.is_none());
                },
            );
        }

        #[test]
        fn test_if_extension_is_not_enabled_null_should_be_returned_for_hooked_proc() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {
                fn hooked_commands(&self) -> Box<dyn Iterator<Item = LayerVulkanCommand> + '_> {
                    Box::new(
                        vec![
                            LayerVulkanCommand::DestroySurfaceKhr,
                            LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2,
                        ]
                        .into_iter(),
                    )
                }
            }
            vk::InstanceCreateInfo::builder().with_instance::<MockLayer<Tag>>(
                |Context {
                     entry, instance, ..
                 }| {
                    let fp = unsafe {
                        entry.get_instance_proc_addr(
                            instance.handle(),
                            b"vkDestroySurfaceKHR\0".as_ptr() as *const i8,
                        )
                    };
                    assert!(fp.is_none());
                    let fp = unsafe {
                        entry.get_instance_proc_addr(
                            instance.handle(),
                            b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr()
                                as *const i8,
                        )
                    };
                    assert!(fp.is_none());
                },
            );
        }

        #[test]
        fn test_commands_that_should_always_be_intercepted() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let app_info = ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
            vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .with_instance::<MockLayer<Tag>>(
                    |Context {
                         entry,
                         instance,
                         next_entry,
                         ..
                     }| {
                        // TODO: test the actual logic of different functions to remove this test.
                        // vkEnumerateInstanceLayerProperties,
                        // vkEnumerateInstanceExtensionProperties, vkCreateInstance are global
                        // commands, and can't be queried with a valid VkInstance.
                        let commands_must_be_intercepted: &[&'static [u8]] = &[
                            // 4 Android introspection queries.
                            b"vkEnumerateDeviceLayerProperties\0",
                            b"vkEnumerateDeviceExtensionProperties\0",
                            b"vkGetInstanceProcAddr\0",
                            b"vkGetDeviceProcAddr\0",
                            // 2 Physical device enumeration APIs.
                            b"vkEnumeratePhysicalDevices\0",
                            b"vkEnumeratePhysicalDeviceGroups\0",
                            // 3 device, instance creation and destruction APIs.
                            b"vkDestroyInstance\0",
                            b"vkCreateDevice\0",
                            b"vkDestroyDevice\0",
                        ];
                        for command in commands_must_be_intercepted {
                            let fp = unsafe {
                                entry.get_instance_proc_addr(
                                    instance.handle(),
                                    command.as_ptr() as *const i8,
                                )
                            }
                            .map(|fp| fp as usize);
                            let next_fp = unsafe {
                                next_entry.get_instance_proc_addr(
                                    instance.handle(),
                                    command.as_ptr() as *const i8,
                                )
                            }
                            .map(|fp| fp as usize);
                            let command_name = CStr::from_bytes_with_nul(*command).unwrap();
                            assert_ne!(
                                fp,
                                next_fp,
                                "The function pointer to {} should be different.",
                                command_name.to_string_lossy()
                            );
                        }
                    },
                );
        }

        #[test]
        #[ignore]
        fn test_should_return_fp_with_available_device_dispatch_command() {}

        #[test]
        #[ignore]
        fn test_should_call_into_next_get_proc_addr_with_unavailable_device_dispatch_command() {}

        #[test]
        #[ignore]
        fn test_should_call_into_next_get_instance_proc_addr_with_unknown_name() {}

        #[test]
        #[ignore]
        fn test_should_return_null_with_unsupported_device_commands() {}
    }

    #[test]
    #[ignore]
    fn test_should_move_layer_instance_link_forward() {
        todo!()
    }
}

#[test]
#[ignore]
fn test_create_instance_with_0_api_version() {
    todo!()
}

#[test]
#[ignore]
fn test_destroy_instance_with_null_handle() {
    todo!()
}

#[test]
#[ignore]
fn test_destroy_instance_will_actually_destroy_underlying_instance_info() {
    todo!()
}

#[test]
#[ignore]
fn test_destroy_device_with_null_handle() {
    todo!()
}
