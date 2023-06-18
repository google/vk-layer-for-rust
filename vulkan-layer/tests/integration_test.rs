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

use ash::vk;
use mockall::predicate::{eq, function};
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan_layer::{
    test_utils::{TestLayer, VkLayerFunction, VkLayerInstanceCreateInfo},
    Extension, Global, Layer, LayerResult, LayerVulkanCommand, VkLayerInstanceLink,
    VulkanBaseInStructChain,
};

pub mod utils;

use utils::{
    create_entry, get_instance_proc_addr, DeviceContext, InstanceContext, InstanceCreateInfoExt,
    InstanceData, MockLayer,
};

use crate::utils::DelInstanceContextExt;

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

            #[derive(Default)]
            struct Tag2;
            impl TestLayer for Tag2 {
                fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                    &[LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2]
                }
            }
            let entry = create_entry::<MockLayer<Tag>>();
            assert!(unsafe {
                entry.get_instance_proc_addr(
                    vk::Instance::null(),
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr() as *const i8,
                )
            }
            .is_none());
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();

            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
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

            #[derive(Default)]
            struct Tag2;
            impl TestLayer for Tag2 {
                fn hooked_global_commands() -> &'static [LayerVulkanCommand] {
                    &[LayerVulkanCommand::CreateInstance]
                }
            }
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let create_instance = unsafe {
                ctx.entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkCreateInstance\0".as_ptr() as *const i8,
                )
            };
            assert!(create_instance.is_none());
        }

        #[test]
        fn test_should_return_fp_for_get_instance_proc_addr() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}

            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
            let get_instance_proc_addr_name = b"vkGetInstanceProcAddr\0".as_ptr() as *const i8;
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
        }

        #[test]
        fn test_should_return_fp_for_core_dispatchable_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
            let destroy_instance = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkDestroyInstance\0".as_ptr() as *const i8,
                )
            };
            assert!(destroy_instance.is_some());
            // vkDestroyInstance will be called in with_instance after this function is
            // returned.
        }

        #[test]
        fn test_should_return_next_proc_addr_for_not_intercepted_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let enabled_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
            let ctx = vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&enabled_extensions)
                .default_instance::<MockLayer<Tag>>();

            let InstanceContext {
                entry,
                instance,
                next_entry,
                ..
            } = ctx.as_ref();
            let destroy_surface_name = b"vkDestroySurfaceKHR\0".as_ptr() as *const i8;
            let destroy_surface =
                unsafe { entry.get_instance_proc_addr(instance.handle(), destroy_surface_name) }
                    .map(|fp| fp as usize);
            // We don't wrap the object, so the VkInstance should be the same.
            let next_destroy_surface = unsafe {
                next_entry.get_instance_proc_addr(instance.handle(), destroy_surface_name)
            }
            .map(|fp| fp as usize);
            assert_eq!(destroy_surface, next_destroy_surface);
        }

        #[test]
        fn test_should_return_fp_for_enabled_instance_extension_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}

            let application_info = ApplicationInfo::builder().api_version(vk::API_VERSION_1_1);
            let ctx = vk::InstanceCreateInfo::builder()
                .application_info(&application_info)
                .default_instance::<MockLayer<Tag>>();

            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
            let fp = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr() as *const i8,
                )
            };
            assert!(fp.is_some());

            let enabled_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
            #[derive(Default)]
            struct Tag2;
            impl TestLayer for Tag2 {
                fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                    &[LayerVulkanCommand::DestroySurfaceKhr]
                }
            }
            let ctx = vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&enabled_extensions)
                .default_instance::<MockLayer<Tag2>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
            let destroy_surface = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkDestroySurfaceKHR\0".as_ptr() as *const i8,
                )
            };
            let destroy_surface: vk::PFN_vkDestroySurfaceKHR =
                unsafe { std::mem::transmute(destroy_surface.unwrap()) };
            let instance_hooks_mock = &Global::<MockLayer<Tag2>>::instance()
                .layer_info
                .get_instance_info(instance.handle())
                .unwrap()
                .mock_hooks;
            instance_hooks_mock
                .lock()
                .unwrap()
                .expect_destroy_surface_khr()
                .times(1)
                .return_const(LayerResult::Unhandled);
            unsafe { destroy_surface(instance.handle(), vk::SurfaceKHR::null(), null()) };
            // TODO: when we allow customize layer extensions, also test with the extension provided
            // by the layer
        }

        #[test]
        fn test_if_extension_is_not_enabled_null_should_be_returned_for_not_hooked_proc() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
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
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr() as *const i8,
                )
            };
            assert!(fp.is_none());
        }

        #[test]
        fn test_if_extension_is_not_enabled_null_should_be_returned_for_hooked_proc() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {
                fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                    &[
                        LayerVulkanCommand::DestroySurfaceKhr,
                        LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2,
                    ]
                }
            }
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
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
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr() as *const i8,
                )
            };
            assert!(fp.is_none());
        }

        #[test]
        fn test_commands_that_should_always_be_intercepted() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let app_info = ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
            let ctx = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry,
                instance,
                next_entry,
                ..
            } = ctx.as_ref();
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
                    entry.get_instance_proc_addr(instance.handle(), command.as_ptr() as *const i8)
                }
                .map(|fp| fp as usize);
                let next_fp = unsafe {
                    next_entry
                        .get_instance_proc_addr(instance.handle(), command.as_ptr() as *const i8)
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
        }

        #[test]
        fn test_should_return_fp_with_available_device_dispatch_command() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                instance, entry, ..
            } = ctx.as_ref();
            let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
            instance_data.set_available_device_extensions(&[Extension::KHRSwapchain]);
            let destroy_swapchain = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkDestroySwapchainKHR\0".as_ptr() as *const i8,
                )
            };
            assert!(destroy_swapchain.is_some());

            #[derive(Default)]
            struct Tag2;
            impl TestLayer for Tag2 {
                fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                    &[LayerVulkanCommand::DestroySwapchainKhr]
                }
            }
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag2>>();
            let InstanceContext {
                instance, entry, ..
            } = ctx.as_ref();
            let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
            instance_data.set_available_device_extensions(&[Extension::KHRSwapchain]);
            let destroy_swapchain = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkDestroySwapchainKHR\0".as_ptr() as *const i8,
                )
            };
            assert!(destroy_swapchain.is_some());
        }

        mod icd_unavaiilable_device_dispatch_command {
            use super::*;
            #[test]
            fn test_should_return_null_if_layer_also_doesnt_implement_the_extension() {
                #[derive(Default)]
                struct Tag;
                impl TestLayer for Tag {
                    fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                        &[LayerVulkanCommand::DestroySwapchainKhr]
                    }
                }
                let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
                let InstanceContext {
                    instance,
                    entry,
                    next_entry,
                    ..
                } = ctx.as_ref();
                let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
                instance_data.set_available_device_extensions(&[]);

                // The mock ICD should return NULL.
                let destroy_swapchain = unsafe {
                    next_entry.get_instance_proc_addr(
                        instance.handle(),
                        b"vkDestroySwapchainKHR\0".as_ptr() as *const i8,
                    )
                };
                assert!(destroy_swapchain.is_none());

                // The layer should also return NULL.
                let destroy_swapchain = unsafe {
                    entry.get_instance_proc_addr(
                        instance.handle(),
                        b"vkDestroySwapchainKHR\0".as_ptr() as *const i8,
                    )
                };
                assert!(destroy_swapchain.is_none());
            }

            #[test]
            #[ignore]
            fn test_should_return_fp_if_layer_implements_the_extension() {
                todo!("Can't implement until we allow our layer to add extensions.")
            }
        }

        #[test]
        fn test_should_call_into_next_get_instance_proc_addr_with_unknown_name() {
            let new_command_name = "vkTestCommand";
            extern "system" fn test_command() {
                unimplemented!()
            }
            let test_command = test_command as unsafe extern "system" fn();

            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {}
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                instance,
                entry,
                next_entry,
                ..
            } = ctx.as_ref();
            let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
            instance_data.add_instance_command(new_command_name, Some(test_command));
            let new_command_name_cstr = CString::new(new_command_name).unwrap();

            // Check what mock ICD returns.
            let actual_new_command = unsafe {
                next_entry.get_instance_proc_addr(instance.handle(), new_command_name_cstr.as_ptr())
            };
            assert_eq!(
                actual_new_command.map(|fp| fp as u64),
                Some(test_command as u64)
            );

            // Check what the layer returns.
            let actual_new_command = unsafe {
                entry.get_instance_proc_addr(instance.handle(), new_command_name_cstr.as_ptr())
            };
            assert_eq!(
                actual_new_command.map(|fp| fp as u64),
                Some(test_command as u64)
            );
        }

        #[test]
        fn test_should_return_null_with_unsupported_device_commands() {
            #[derive(Default)]
            struct Tag;
            impl TestLayer for Tag {
                fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                    &[LayerVulkanCommand::DestroySwapchainKhr]
                }
            }
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext {
                entry, instance, ..
            } = ctx.as_ref();
            unsafe { InstanceData::from_handle(instance.handle()) }
                .set_available_device_extensions(&[]);
            let destroy_swapchain = unsafe {
                entry.get_instance_proc_addr(
                    instance.handle(),
                    b"vkDestroySwapchainKHR\0".as_ptr() as *const i8,
                )
            };
            assert!(destroy_swapchain.is_none());
        }
    }
}

mod create_destroy_instance {

    use super::*;
    #[test]
    fn test_should_move_layer_instance_link_forward() {
        #[derive(Default)]
        struct Tag1;
        impl TestLayer for Tag1 {
            fn hooked_global_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateInstance]
            }
        }
        let entry_layer1 = create_entry::<MockLayer<Tag1>>();

        #[derive(Default)]
        struct Tag2;
        impl TestLayer for Tag2 {
            fn hooked_global_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateInstance]
            }
        }
        let entry_layer2 = create_entry::<MockLayer<Tag2>>();

        let mut second_layer_link = VkLayerInstanceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        };
        let mut first_layer_link = VkLayerInstanceLink {
            pNext: &mut second_layer_link,
            pfnNextGetInstanceProcAddr: entry_layer2.static_fn().get_instance_proc_addr
                as vk::PFN_vkGetInstanceProcAddr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        };
        let mut layer_create_info = VkLayerInstanceCreateInfo {
            sType: vk::StructureType::LOADER_INSTANCE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        *unsafe { layer_create_info.u.pLayerInfo.as_mut() } = &mut first_layer_link;
        let create_instance_info =
            vk::InstanceCreateInfo::builder().push_next(&mut layer_create_info);

        fn match_create_instance(
            next_layer_link: Option<&VkLayerInstanceLink>,
            current_layer_link: &VkLayerInstanceLink,
        ) -> impl Fn(
            &vk::InstanceCreateInfo,
            &VkLayerInstanceLink,
            &Option<&vk::AllocationCallbacks>,
        ) -> bool {
            let next_layer_link_ptr =
                next_layer_link.map_or(0, |layer_link| layer_link as *const _ as u64);
            let current_layer_link_ptr = current_layer_link as *const _ as u64;
            move |create_info, layer_instance_link, _| {
                let mut p_next_chain: VulkanBaseInStructChain =
                    unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() }.into();
                let layer_link_head = p_next_chain.find_map(|in_struct| {
                    let in_struct = in_struct as *const vk::BaseInStructure;
                    let layer_instance_create_info = unsafe {
                        ash::match_in_struct!(match in_struct {
                            in_struct @ VkLayerInstanceCreateInfo => {
                                in_struct
                            }
                            _ => {
                                return None;
                            }
                        })
                    };
                    if layer_instance_create_info.function != VkLayerFunction::VK_LAYER_LINK_INFO {
                        return None;
                    }
                    Some(*unsafe { layer_instance_create_info.u.pLayerInfo.as_ref() })
                });
                let layer_link_head = match layer_link_head {
                    Some(head) => head,
                    None => return false,
                };
                layer_link_head as u64 == next_layer_link_ptr
                    && layer_instance_link as *const _ as u64 == current_layer_link_ptr
            }
        }

        {
            let layer1_info = Arc::clone(&Global::<MockLayer<Tag1>>::instance().layer_info);
            let layer2_info = Arc::clone(&Global::<MockLayer<Tag2>>::instance().layer_info);
            {
                let mut layer1_global_hooks = layer1_info.global_hooks.lock().unwrap();
                layer1_global_hooks
                    .expect_create_instance()
                    .withf_st(match_create_instance(
                        Some(&second_layer_link),
                        &first_layer_link,
                    ))
                    .once()
                    .return_const(LayerResult::Unhandled);
            }
            {
                let mut layer2_global_hooks = layer2_info.global_hooks.lock().unwrap();
                layer2_global_hooks
                    .expect_create_instance()
                    .withf_st(match_create_instance(None, &second_layer_link))
                    .once()
                    .return_const(LayerResult::Unhandled);
            }

            let instance =
                unsafe { entry_layer1.create_instance(&create_instance_info, None) }.unwrap();
            {
                layer1_info.global_hooks.lock().unwrap().checkpoint();
                layer2_info.global_hooks.lock().unwrap().checkpoint();
            }
            unsafe { instance.destroy_instance(None) };
        }
    }

    #[test]
    fn test_create_instance_with_0_api_version() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}

        let app_info = vk::ApplicationInfo::builder().api_version(0);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<MockLayer<Tag>>();
        let InstanceContext {
            entry, instance, ..
        } = ctx.as_ref();
        let destroy_instance = unsafe {
            entry.get_instance_proc_addr(
                instance.handle(),
                b"vkDestroyInstance\0".as_ptr() as *const i8,
            )
        };
        assert!(destroy_instance.is_some());
    }

    #[test]
    fn test_destroy_instance_with_null_handle() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}

        let app_info = vk::ApplicationInfo::builder().api_version(0);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<MockLayer<Tag>>();
        let InstanceContext { instance, .. } = ctx.as_ref();
        let destroy_instance = instance.fp_v1_0().destroy_instance;
        unsafe { destroy_instance(vk::Instance::null(), null()) };
    }

    #[test]
    fn test_destroy_instance_will_actually_destroy_underlying_instance_info() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}

        {
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<MockLayer<Tag>>();
            let InstanceContext { instance, .. } = ctx.as_ref();
            let instance_data = Global::<MockLayer<Tag>>::instance()
                .layer_info
                .get_instance_info(instance.handle())
                .unwrap();
            instance_data.with_mock_drop(|mock_drop| {
                mock_drop.expect_drop().once().return_const(());
            });
            // Calling vkDestroyInstance through RAII.
        }
    }
}

#[test]
fn test_destroy_device_with_null_handle() {
    #[derive(Default)]
    struct Tag;
    impl TestLayer for Tag {}
    let ctx = vk::InstanceCreateInfo::builder()
        .default_instance::<MockLayer<Tag>>()
        .default_device::<MockLayer<Tag>>();
    unsafe { (ctx.device.fp_v1_0().destroy_device)(vk::Device::null(), null()) };
}

mod device_commands {
    use super::*;
    #[test]
    fn test_hooked_device_proc_should_be_called() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroyImage]
            }
        }
        let ctx = vk::InstanceCreateInfo::builder()
            .default_instance::<MockLayer<Tag>>()
            .default_device::<MockLayer<Tag>>();
        let DeviceContext { device, .. } = ctx.as_ref();
        let device_info = Global::<MockLayer<Tag>>::instance()
            .layer_info
            .get_device_info(device.handle())
            .unwrap();
        device_info
            .mock_hooks
            .lock()
            .unwrap()
            .expect_destroy_image()
            .with(
                eq(vk::Image::null()),
                function(|allocator: &Option<_>| allocator.is_none()),
            )
            .once()
            .return_const(LayerResult::Unhandled);
        unsafe { device.destroy_image(vk::Image::null(), None) };
    }

    #[test]
    fn test_unhooked_device_proc_should_not_be_called() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[]
            }
        }
        let ctx = vk::InstanceCreateInfo::builder()
            .default_instance::<MockLayer<Tag>>()
            .default_device::<MockLayer<Tag>>();
        let DeviceContext { device, .. } = ctx.as_ref();
        let device_info = Global::<MockLayer<Tag>>::instance()
            .layer_info
            .get_device_info(device.handle())
            .unwrap();
        device_info
            .mock_hooks
            .lock()
            .unwrap()
            .expect_destroy_image()
            .never();
        unsafe { device.destroy_image(vk::Image::null(), None) };
    }
}
