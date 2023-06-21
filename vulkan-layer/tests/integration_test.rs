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
use mockall::predicate::{always, eq};
use std::{
    ffi::{CStr, CString},
    iter::zip,
    mem::MaybeUninit,
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan_layer::{
    test_utils::{
        TestLayer, VkLayerDeviceCreateInfo, VkLayerDeviceLink, VkLayerFunction,
        VkLayerInstanceCreateInfo,
    },
    ApiVersion, Extension, ExtensionProperties, Global, Layer, LayerResult, LayerVulkanCommand,
    VkLayerInstanceLink, VulkanBaseInStructChain,
};

pub mod utils;

use utils::{
    create_entry, DeviceContext, FromVulkanHandle, InstanceContext, InstanceCreateInfoExt,
    InstanceData, Layers, MockLayer,
};

use crate::utils::ArcDelInstanceContextExt;

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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();

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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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

            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
                .default_instance::<(MockLayer<Tag>,)>();

            let InstanceContext {
                entry,
                instance,
                icd_entry,
                ..
            } = ctx.as_ref();
            let destroy_surface_name = b"vkDestroySurfaceKHR\0".as_ptr() as *const i8;
            let destroy_surface =
                unsafe { entry.get_instance_proc_addr(instance.handle(), destroy_surface_name) }
                    .map(|fp| fp as usize);
            // We don't wrap the object, so the VkInstance should be the same.
            let next_destroy_surface = unsafe {
                icd_entry.get_instance_proc_addr(instance.handle(), destroy_surface_name)
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
                .default_instance::<(MockLayer<Tag>,)>();

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
                .default_instance::<(MockLayer<Tag2>,)>();
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
                .default_instance::<(MockLayer<Tag>,)>();
            let InstanceContext {
                entry,
                instance,
                icd_entry,
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
                    icd_entry
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag2>,)>();
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
                let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
                let InstanceContext {
                    instance,
                    entry,
                    icd_entry,
                    ..
                } = ctx.as_ref();
                let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
                instance_data.set_available_device_extensions(&[]);

                // The mock ICD should return NULL.
                let destroy_swapchain = unsafe {
                    icd_entry.get_instance_proc_addr(
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
            let InstanceContext {
                instance,
                entry,
                icd_entry,
                ..
            } = ctx.as_ref();
            let instance_data = unsafe { InstanceData::from_handle(instance.handle()) };
            instance_data.add_instance_command(new_command_name, Some(test_command));
            let new_command_name_cstr = CString::new(new_command_name).unwrap();

            // Check what mock ICD returns.
            let actual_new_command = unsafe {
                icd_entry.get_instance_proc_addr(instance.handle(), new_command_name_cstr.as_ptr())
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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
        #[derive(Default)]
        struct Tag2;
        impl TestLayer for Tag2 {
            fn hooked_global_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateInstance]
            }
        }
        let icd_layer_link = <()>::head_instance_link();
        let second_layer_link = <(MockLayer<Tag2>,)>::head_instance_link();

        fn layer_instance_link_equal(
            lhs: &Option<&VkLayerInstanceLink>,
            rhs: &Option<&VkLayerInstanceLink>,
        ) -> bool {
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => {
                    lhs.pfnNextGetInstanceProcAddr == rhs.pfnNextGetInstanceProcAddr
                        && lhs.pfnNextGetPhysicalDeviceProcAddr
                            == rhs.pfnNextGetPhysicalDeviceProcAddr
                }
                (None, None) => true,
                _ => false,
            }
        }
        fn match_create_instance(
            next_layer_link: Option<&VkLayerInstanceLink>,
            current_layer_link: &VkLayerInstanceLink,
        ) -> impl Fn(
            &vk::InstanceCreateInfo,
            &VkLayerInstanceLink,
            &Option<&vk::AllocationCallbacks>,
        ) -> bool {
            let next_layer_link = next_layer_link.map(|layer_link| VkLayerInstanceLink {
                pNext: null_mut(),
                ..*layer_link
            });
            let current_layer_link = VkLayerInstanceLink {
                pNext: null_mut(),
                ..*current_layer_link
            };
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
                    let layer_link_head =
                        *unsafe { layer_instance_create_info.u.pLayerInfo.as_ref() };
                    Some(unsafe { layer_link_head.as_ref() }?)
                });
                layer_instance_link_equal(&layer_link_head, &next_layer_link.as_ref())
                    && layer_instance_link_equal(
                        &Some(layer_instance_link),
                        &Some(&current_layer_link),
                    )
            }
        }

        {
            let layer1_info = Arc::clone(&Global::<MockLayer<Tag1>>::instance().layer_info);
            let layer2_info = Arc::clone(&Global::<MockLayer<Tag2>>::instance().layer_info);
            {
                let mut layer1_global_hooks = layer1_info.get_global_hooks();
                layer1_global_hooks
                    .expect_create_instance()
                    .withf_st(match_create_instance(
                        Some(&icd_layer_link),
                        &second_layer_link,
                    ))
                    .once()
                    .return_const(LayerResult::Unhandled);
            }
            {
                let mut layer2_global_hooks = layer2_info.get_global_hooks();
                layer2_global_hooks
                    .expect_create_instance()
                    .withf_st(match_create_instance(None, &icd_layer_link))
                    .once()
                    .return_const(LayerResult::Unhandled);
            }

            let instance = vk::InstanceCreateInfo::builder()
                .default_instance::<(MockLayer<Tag1>, MockLayer<Tag2>)>();
            {
                layer1_info.get_global_hooks().checkpoint();
                layer2_info.get_global_hooks().checkpoint();
            }
            drop(instance);
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
            .default_instance::<(MockLayer<Tag>,)>();
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
            .default_instance::<(MockLayer<Tag>,)>();
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
            let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
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

mod get_device_proc_addr {
    use super::*;
    #[test]
    fn test_should_return_null_for_global_commands() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}
        let ctx = vk::InstanceCreateInfo::builder()
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();

        let global_commands = [
            "vkEnumerateInstanceVersion",
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkCreateInstance",
        ];
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        for global_command in &global_commands {
            let global_command = CString::new(*global_command).unwrap();
            let proc_addr = unsafe {
                (instance_context.instance.fp_v1_0().get_device_proc_addr)(
                    device.handle(),
                    global_command.as_ptr(),
                )
            };
            assert!(
                proc_addr.is_none(),
                "{} should be null",
                global_command.to_string_lossy()
            );
        }
    }

    #[test]
    fn test_should_return_null_for_instance_commands() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2]
            }
        }
        let app_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_2);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();

        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let get_physical_device_sparse_image_format_properties2 = unsafe {
            (instance_context.instance.fp_v1_0().get_device_proc_addr)(
                device.handle(),
                b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr() as *const i8,
            )
        };
        assert!(get_physical_device_sparse_image_format_properties2.is_none());
    }

    #[test]
    fn test_should_return_null_for_available_but_not_enabled_commands() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySwapchainKhr]
            }
        }
        let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
        let InstanceContext {
            instance, entry, ..
        } = ctx.as_ref();
        unsafe { InstanceData::from_handle(instance.handle()) }
            .set_available_device_extensions(&[Extension::KHRSwapchain]);

        let destroy_swapchain_name = CString::new("vkDestroySwapchainKHR").unwrap();

        // vkGetInstanceAddr should return fp for available extensions.
        let destroy_swapchain = unsafe {
            entry.get_instance_proc_addr(instance.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(destroy_swapchain.is_some());

        let ctx = ctx.default_device().unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let destroy_swapchain = unsafe {
            (instance_context
                .next_instance_dispatch
                .fp_v1_0()
                .get_device_proc_addr)(device.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(
            destroy_swapchain.is_none(),
            "The mock ICD should return NULL for vkDestroySwapchainKHR."
        );
        let destroy_swapchain = unsafe {
            (instance_context.instance.fp_v1_0().get_device_proc_addr)(
                device.handle(),
                destroy_swapchain_name.as_ptr(),
            )
        };
        assert!(destroy_swapchain.is_none());
    }

    #[test]
    fn test_should_return_fp_for_requested_core_version_device_commands() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySamplerYcbcrConversion]
            }
        }
        let app_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_1);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let destroy_sampler_ycbcr_conversion = unsafe {
            (instance_context.instance.fp_v1_0().get_device_proc_addr)(
                device.handle(),
                b"vkDestroySamplerYcbcrConversion\0".as_ptr() as *const i8,
            )
        };
        let destroy_sampler_ycbcr_conversion: vk::PFN_vkDestroySamplerYcbcrConversion =
            unsafe { std::mem::transmute(destroy_sampler_ycbcr_conversion.unwrap()) };
        let layer_device_info = Global::<MockLayer<Tag>>::instance()
            .layer_info
            .get_device_info(device.handle())
            .unwrap();
        layer_device_info
            .mock_hooks
            .lock()
            .unwrap()
            .expect_destroy_sampler_ycbcr_conversion()
            .with(eq(vk::SamplerYcbcrConversion::null()), always())
            .once()
            .return_const(LayerResult::Unhandled);
        unsafe {
            destroy_sampler_ycbcr_conversion(
                device.handle(),
                vk::SamplerYcbcrConversion::null(),
                null(),
            )
        };
        layer_device_info.mock_hooks.lock().unwrap().checkpoint();
    }

    #[test]
    fn test_should_return_null_for_unrequested_core_version_device_commands() {
        // For this test case, the specified `VkApplicationInfo::apiVersion` is low.
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySamplerYcbcrConversion]
            }
        }
        let app_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_0);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let destroy_sampler_ycbcr_conversion = unsafe {
            (instance_context.instance.fp_v1_0().get_device_proc_addr)(
                device.handle(),
                b"vkDestroySamplerYcbcrConversion\0".as_ptr() as *const i8,
            )
        };
        assert!(destroy_sampler_ycbcr_conversion.is_none());
    }

    #[test]
    fn test_should_return_null_for_unsupported_core_version_device_commands() {
        // For this test case, the specified `VkApplicationInfo::apiVersion` is high enough, but the
        // `VkPhysicalDeviceProperties::apiVersion` is low.
        // For this test case, the specified `VkApplicationInfo::apiVersion` is low.
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySamplerYcbcrConversion]
            }
        }
        let app_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_1);
        let ctx = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .default_instance::<(MockLayer<Tag>,)>();
        unsafe { InstanceData::from_handle(ctx.instance.handle()) }
            .set_supported_device_version(&ApiVersion::V1_0);

        let ctx = ctx.default_device().unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let InstanceContext {
            instance,
            icd_entry,
            next_instance_dispatch,
            ..
        } = instance_context.as_ref();

        let destroy_sampler_ycbcr_conversion_name =
            CString::new("vkDestroySamplerYcbcrConversion").unwrap();

        // Make sure that the mock ICD will return null for unsupported device command in
        // vkGetInstanceProcAddr.
        let destroy_sampler_ycbcr_conversion = unsafe {
            icd_entry.get_instance_proc_addr(
                instance.handle(),
                destroy_sampler_ycbcr_conversion_name.as_ptr(),
            )
        };
        assert!(destroy_sampler_ycbcr_conversion.is_none());

        // Make sure that the mock ICD will return null for unsupported device command in
        // vkGetDeviceProcAddr.
        let destroy_sampler_ycbcr_conversion = unsafe {
            next_instance_dispatch.get_device_proc_addr(
                device.handle(),
                destroy_sampler_ycbcr_conversion_name.as_ptr(),
            )
        };
        assert!(destroy_sampler_ycbcr_conversion.is_none());

        let destroy_sampler_ycbcr_conversion = unsafe {
            instance.get_device_proc_addr(
                device.handle(),
                destroy_sampler_ycbcr_conversion_name.as_ptr(),
            )
        };
        assert!(destroy_sampler_ycbcr_conversion.is_none());
    }

    #[test]
    fn test_should_return_fp_for_enabled_extension_device_commands() {
        let instance_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
        let device_extensions = [vk::KhrSwapchainFn::name().as_ptr()];
        let destroy_swapchain_name = CString::new("vkDestroySwapchainKHR").unwrap();
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}
        let ctx = vk::InstanceCreateInfo::builder()
            .enabled_extension_names(&instance_extensions)
            .default_instance::<(MockLayer<Tag>,)>()
            .create_device(|create_info_builder, create_device| {
                let create_info_builder =
                    create_info_builder.enabled_extension_names(&device_extensions);
                create_device(create_info_builder)
            })
            .unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let destroy_swapchain = unsafe {
            instance_context
                .instance
                .get_device_proc_addr(device.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(destroy_swapchain.is_some());

        #[derive(Default)]
        struct Tag2;
        impl TestLayer for Tag2 {
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySwapchainKhr]
            }
        }
        let ctx = vk::InstanceCreateInfo::builder()
            .enabled_extension_names(&instance_extensions)
            .default_instance::<(MockLayer<Tag2>,)>()
            .create_device(|create_info_builder, create_device| {
                let create_info_builder =
                    create_info_builder.enabled_extension_names(&device_extensions);
                create_device(create_info_builder)
            })
            .unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let destroy_swapchain = unsafe {
            instance_context
                .instance
                .get_device_proc_addr(device.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(destroy_swapchain.is_some());
    }

    #[test]
    fn test_should_return_fp_for_enabled_layer_extension_device_commands() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRSwapchain,
                spec_version: 1,
            }];
            fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::DestroySwapchainKhr]
            }
        }
        let instance_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
        let device_extensions = [vk::KhrSwapchainFn::name().as_ptr()];
        let ctx = vk::InstanceCreateInfo::builder()
            .enabled_extension_names(&instance_extensions)
            .default_instance::<(MockLayer<Tag>,)>();
        unsafe { InstanceData::from_handle(ctx.instance.handle()) }
            .set_available_device_extensions(&[]);

        let ctx = ctx
            .create_device(|create_info_builder, create_device| {
                let create_info_builder =
                    create_info_builder.enabled_extension_names(&device_extensions);
                create_device(create_info_builder)
            })
            .unwrap();
        let DeviceContext {
            device,
            instance_context,
            ..
        } = ctx.as_ref();
        let InstanceContext {
            instance,
            next_instance_dispatch,
            ..
        } = instance_context.as_ref();
        let destroy_swapchain_name = CString::new("vkDestroySwapchainKHR").unwrap();
        // The mock ICD should return null for this command.
        let destroy_swapchain = unsafe {
            next_instance_dispatch
                .get_device_proc_addr(device.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(destroy_swapchain.is_none());
        let destroy_swapchain = unsafe {
            instance.get_device_proc_addr(device.handle(), destroy_swapchain_name.as_ptr())
        };
        assert!(destroy_swapchain.is_some());
    }
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
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();
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
            .withf(|image, allocator| *image == vk::Image::null() && allocator.is_none())
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
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();
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

mod enumerate_device_extensions {
    use super::*;
    #[test]
    fn test_should_return_defined_extensions() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRExternalMemory,
                spec_version: 1,
            }];
        }
        let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
        let InstanceContext { instance, .. } = ctx.as_ref();
        unsafe { InstanceData::from_handle(instance.handle()) }
            .set_available_device_extensions(&[]);
        let physical_device = *unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .first()
            .unwrap();
        let layer_name = CString::new(MockLayer::<Tag>::LAYER_NAME).unwrap();
        let properties = {
            let mut property_count = MaybeUninit::uninit();
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    physical_device,
                    layer_name.as_ptr(),
                    property_count.as_mut_ptr(),
                    null_mut(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            let mut property_count = unsafe { property_count.assume_init() };
            let mut properties = Vec::<vk::ExtensionProperties>::new();
            properties.resize_with(property_count.try_into().unwrap(), Default::default);
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    physical_device,
                    layer_name.as_ptr(),
                    &mut property_count,
                    properties.as_mut_ptr(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            properties.into_boxed_slice()
        };
        let expected_properties = Tag::DEVICE_EXTENSIONS
            .iter()
            .cloned()
            .map(Into::<vk::ExtensionProperties>::into)
            .collect::<Vec<_>>();
        assert_eq!(expected_properties.len(), properties.len());
        for (expected_property, property) in zip(expected_properties.iter(), properties.iter()) {
            assert_eq!(expected_property.spec_version, property.spec_version);
            let expected_name_bytes = expected_property
                .extension_name
                .iter()
                .take_while(|byte| **byte != b'\0'.try_into().unwrap())
                .collect::<Vec<_>>();
            let name_bytes = property
                .extension_name
                .iter()
                .take_while(|byte| **byte != b'\0'.try_into().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(expected_name_bytes, name_bytes);
        }
    }

    // TODO: test if the loader will add the implicit layer extension in e2e test.

    #[test]
    fn test_should_call_into_the_next_chain_if_layer_name_doesnt_match() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRExternalMemory,
                spec_version: 1,
            }];
        }
        let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
        let InstanceContext { instance, .. } = ctx.as_ref();
        let available_device_extensions = &[Extension::KHRSwapchain];
        unsafe { InstanceData::from_handle(instance.handle()) }
            .set_available_device_extensions(available_device_extensions);
        let physical_device = *unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .first()
            .unwrap();
        let properties = {
            let mut property_count = MaybeUninit::uninit();
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    physical_device,
                    null(),
                    property_count.as_mut_ptr(),
                    null_mut(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            let mut property_count = unsafe { property_count.assume_init() };
            let mut properties = Vec::<vk::ExtensionProperties>::new();
            properties.resize_with(property_count.try_into().unwrap(), Default::default);
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    physical_device,
                    null(),
                    &mut property_count,
                    properties.as_mut_ptr(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            properties.into_boxed_slice()
        };
        assert_eq!(available_device_extensions.len(), properties.len());
        for (expected_extension_name, property) in
            zip(available_device_extensions.iter(), properties.iter())
        {
            let expected_name: &str = expected_extension_name.clone().into();
            let actual_name = unsafe {
                std::slice::from_raw_parts(
                    property.extension_name.as_ptr() as *const u8,
                    property.extension_name.len(),
                )
            };
            let actual_name = CStr::from_bytes_until_nul(actual_name)
                .unwrap()
                .to_str()
                .unwrap();
            assert_eq!(expected_name, actual_name);
        }
    }

    #[test]
    fn test_should_return_defined_extensions_with_null_physical_device() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRExternalMemory,
                spec_version: 1,
            }];
        }
        let ctx = vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
        let InstanceContext { instance, .. } = ctx.as_ref();
        unsafe { InstanceData::from_handle(instance.handle()) }
            .set_available_device_extensions(&[]);
        let layer_name = CString::new(MockLayer::<Tag>::LAYER_NAME).unwrap();
        let properties = {
            let mut property_count = MaybeUninit::uninit();
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    vk::PhysicalDevice::null(),
                    layer_name.as_ptr(),
                    property_count.as_mut_ptr(),
                    null_mut(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            let mut property_count = unsafe { property_count.assume_init() };
            let mut properties = Vec::<vk::ExtensionProperties>::new();
            properties.resize_with(property_count.try_into().unwrap(), Default::default);
            let res = unsafe {
                (instance.fp_v1_0().enumerate_device_extension_properties)(
                    vk::PhysicalDevice::null(),
                    layer_name.as_ptr(),
                    &mut property_count,
                    properties.as_mut_ptr(),
                )
            };
            assert_eq!(res, vk::Result::SUCCESS);
            properties.into_boxed_slice()
        };
        let expected_properties = Tag::DEVICE_EXTENSIONS
            .iter()
            .cloned()
            .map(Into::<vk::ExtensionProperties>::into)
            .collect::<Vec<_>>();
        assert_eq!(expected_properties.len(), properties.len());
        for (expected_property, property) in zip(expected_properties.iter(), properties.iter()) {
            assert_eq!(expected_property.spec_version, property.spec_version);
            let expected_name_bytes = expected_property
                .extension_name
                .iter()
                .take_while(|byte| **byte != b'\0'.try_into().unwrap())
                .collect::<Vec<_>>();
            let name_bytes = property
                .extension_name
                .iter()
                .take_while(|byte| **byte != b'\0'.try_into().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(expected_name_bytes, name_bytes);
        }
    }
}

mod create_destroy_device {
    use vulkan_layer::InstanceInfo;

    use crate::utils::DeviceData;

    use super::*;
    #[test]
    fn test_should_always_pass_through_non_layer_extensions() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRGetPhysicalDeviceProperties2,
                spec_version: 1,
            }];
        }

        let instance_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
        let instance_ctx = vk::InstanceCreateInfo::builder()
            .enabled_extension_names(&instance_extensions)
            .default_instance::<(MockLayer<Tag>,)>();
        unsafe { InstanceData::from_handle(instance_ctx.instance.handle()) }
            .set_available_device_extensions(&[Extension::KHRSwapchain]);
        let enabled_device_extensions = [
            // Available extension.
            vk::KhrSwapchainFn::name().as_ptr(),
            vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr(),
        ];
        let device_ctx = instance_ctx
            .clone()
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&enabled_device_extensions))
            })
            .unwrap();
        let enabled_icd_device_extensions =
            unsafe { DeviceData::from_handle(device_ctx.device.handle()) }
                .enabled_extensions
                .iter()
                .cloned()
                .collect::<Vec<_>>();
        assert_eq!(enabled_icd_device_extensions, vec![Extension::KHRSwapchain]);

        let enabled_device_extensions = [
            // Unavailable extension.
            vk::KhrExternalMemoryCapabilitiesFn::name().as_ptr(),
            vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr(),
        ];
        let device_create_res = instance_ctx
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&enabled_device_extensions))
            })
            .map(drop);
        assert_eq!(
            device_create_res.unwrap_err(),
            vk::Result::ERROR_EXTENSION_NOT_PRESENT
        );
    }

    #[test]
    fn test_should_always_filter_out_layer_extensions() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRGetPhysicalDeviceProperties2,
                spec_version: 1,
            }];
        }

        let instance_ctx =
            vk::InstanceCreateInfo::builder().default_instance::<(MockLayer<Tag>,)>();
        let enabled_device_extensions = [vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr()];

        unsafe { InstanceData::from_handle(instance_ctx.instance.handle()) }
            .set_available_device_extensions(&[Extension::KHRGetPhysicalDeviceProperties2]);
        let device_ctx = instance_ctx
            .clone()
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&enabled_device_extensions))
            })
            .unwrap();
        let icd_enabled_extensions = unsafe { DeviceData::from_handle(device_ctx.device.handle()) }
            .enabled_extensions
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(icd_enabled_extensions, vec![]);

        unsafe { InstanceData::from_handle(instance_ctx.instance.handle()) }
            .set_available_device_extensions(&[]);
        let device_ctx = instance_ctx
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&enabled_device_extensions))
            })
            .unwrap();
        let icd_enabled_extensions = unsafe { DeviceData::from_handle(device_ctx.device.handle()) }
            .enabled_extensions
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(icd_enabled_extensions, vec![]);
    }

    #[test]
    fn test_should_always_pass_through_unknown_extensions() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRGetPhysicalDeviceProperties2,
                spec_version: 1,
            }];
        }

        let device_extensions = [CString::new("VK_UNKNOWN_unknown_extension").unwrap()];
        let device_extensions = device_extensions.map(|extension| extension.as_ptr());
        let device_create_res = vk::InstanceCreateInfo::builder()
            .default_instance::<(MockLayer<Tag>,)>()
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&device_extensions))
            })
            .map(drop);
        assert_eq!(
            device_create_res.unwrap_err(),
            vk::Result::ERROR_EXTENSION_NOT_PRESENT
        );
    }

    #[test]
    fn test_should_move_layer_device_link_forward() {
        #[derive(Default)]
        struct Tag1;
        impl TestLayer for Tag1 {
            fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateDevice]
            }
        }

        #[derive(Default)]
        struct Tag2;
        impl TestLayer for Tag2 {
            fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateDevice]
            }
        }

        let instance_ctx = vk::InstanceCreateInfo::builder()
            .default_instance::<(MockLayer<Tag1>, MockLayer<Tag2>)>();
        let InstanceContext { instance, .. } = instance_ctx.as_ref();

        let icd_layer_link = <()>::head_device_link(instance.handle());
        let second_layer_link = <(MockLayer<Tag2>,)>::head_device_link(instance.handle());

        fn layer_device_link_equal(
            lhs: &Option<&VkLayerDeviceLink>,
            rhs: &Option<&VkLayerDeviceLink>,
        ) -> bool {
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => {
                    lhs.pfnNextGetInstanceProcAddr == rhs.pfnNextGetInstanceProcAddr
                        && lhs.pfnNextGetDeviceProcAddr == rhs.pfnNextGetDeviceProcAddr
                }
                (None, None) => true,
                _ => false,
            }
        }

        fn match_create_device(
            next_layer_link: Option<&VkLayerDeviceLink>,
            current_layer_link: &VkLayerDeviceLink,
        ) -> impl Fn(
            &vk::PhysicalDevice,
            &vk::DeviceCreateInfo,
            &VkLayerDeviceLink,
            &Option<&vk::AllocationCallbacks>,
        ) -> bool {
            let next_layer_link = next_layer_link.map(|layer_link| VkLayerDeviceLink {
                pNext: null_mut(),
                ..*layer_link
            });
            let current_layer_link = VkLayerDeviceLink {
                pNext: null_mut(),
                ..*current_layer_link
            };
            move |_, create_info, layer_instance_link, _| {
                let mut p_next_chain: VulkanBaseInStructChain =
                    unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() }.into();
                let layer_link_head = p_next_chain.find_map(|in_struct| {
                    let in_struct = in_struct as *const vk::BaseInStructure;
                    let layer_device_create_info = unsafe {
                        ash::match_in_struct!(match in_struct {
                            in_struct @ VkLayerDeviceCreateInfo => {
                                in_struct
                            }
                            _ => {
                                return None;
                            }
                        })
                    };
                    if layer_device_create_info.function != VkLayerFunction::VK_LAYER_LINK_INFO {
                        return None;
                    }
                    let layer_link_head = unsafe { layer_device_create_info.u.pLayerInfo };
                    Some(unsafe { layer_link_head.as_ref() }?)
                });
                layer_device_link_equal(&layer_link_head, &next_layer_link.as_ref())
                    && layer_device_link_equal(
                        &Some(layer_instance_link),
                        &Some(&current_layer_link),
                    )
            }
        }
        let layer1_instance_info = Arc::clone(&Global::<MockLayer<Tag1>>::instance().layer_info)
            .get_instance_info(instance.handle())
            .unwrap();
        let layer2_instance_info = Arc::clone(&Global::<MockLayer<Tag2>>::instance().layer_info)
            .get_instance_info(instance.handle())
            .unwrap();
        layer1_instance_info
            .hooks()
            .expect_create_device()
            .withf_st(match_create_device(
                Some(&icd_layer_link),
                &second_layer_link,
            ))
            .once()
            .return_const(LayerResult::Unhandled);
        layer2_instance_info
            .hooks()
            .expect_create_device()
            .withf_st(match_create_device(None, &icd_layer_link))
            .once()
            .return_const(LayerResult::Unhandled);
        let device_ctx = instance_ctx.default_device().unwrap();
        layer1_instance_info.hooks().checkpoint();
        layer2_instance_info.hooks().checkpoint();
        drop(device_ctx);
    }

    #[test]
    fn test_should_receive_the_original_extension_list() {
        let instance_extensions = [vk::KhrSurfaceFn::name().as_ptr()];
        let expected_device_extensions: Vec<&'static CStr> = vec![vk::KhrSwapchainFn::name()];
        let device_extensions = expected_device_extensions
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();

        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {
            const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
                name: Extension::KHRSwapchain,
                spec_version: 1,
            }];

            fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
                &[LayerVulkanCommand::CreateDevice]
            }
        }

        let instance_ctx = vk::InstanceCreateInfo::builder()
            .enabled_extension_names(&instance_extensions)
            .default_instance::<(MockLayer<Tag>,)>();
        let instance_info = Global::<MockLayer<Tag>>::instance()
            .layer_info
            .get_instance_info(instance_ctx.instance.handle())
            .unwrap();
        instance_info
            .hooks()
            .expect_create_device()
            .withf(move |_, create_info, _, _| {
                let extensions = unsafe {
                    std::slice::from_raw_parts(
                        create_info.pp_enabled_extension_names,
                        create_info.enabled_extension_count.try_into().unwrap(),
                    )
                };
                let extensions = extensions
                    .iter()
                    .map(|extension| unsafe { CStr::from_ptr(*extension) })
                    .collect::<Vec<_>>();
                extensions == expected_device_extensions
            })
            .once()
            .return_const(LayerResult::Unhandled);
        let device_ctx = instance_ctx
            .create_device(|create_info, create_device| {
                create_device(create_info.enabled_extension_names(&device_extensions))
            })
            .unwrap();
        instance_info.hooks().checkpoint();
        drop(device_ctx);
    }

    #[test]
    fn test_destroy_device_with_null_handle() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}
        let ctx = vk::InstanceCreateInfo::builder()
            .default_instance::<(MockLayer<Tag>,)>()
            .default_device()
            .unwrap();
        unsafe { (ctx.device.fp_v1_0().destroy_device)(vk::Device::null(), null()) };
    }

    #[test]
    fn test_destroy_device_will_actually_destroy_underlying_device_info() {
        #[derive(Default)]
        struct Tag;
        impl TestLayer for Tag {}

        {
            let ctx = vk::InstanceCreateInfo::builder()
                .default_instance::<(MockLayer<Tag>,)>()
                .default_device()
                .unwrap();
            let DeviceContext { device, .. } = ctx.as_ref();
            let device_data = Global::<MockLayer<Tag>>::instance()
                .layer_info
                .get_device_info(device.handle())
                .unwrap();
            device_data.with_mock_drop(|mock_drop| {
                mock_drop.expect_drop().once().return_const(());
            });
            // Calling vkDestroyDevice through RAII.
        }
    }
}
