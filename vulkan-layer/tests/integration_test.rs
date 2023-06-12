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

use std::{
    ffi::{c_void, CStr, CString},
    ptr::{null, null_mut},
};

use ash::vk::{self, Handle};
use vulkan_layer::{
    test_utils::{MockLayer, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink},
    Global, Layer,
};

fn create_entry<T: Layer>() -> ash::Entry {
    unsafe {
        ash::Entry::from_static_fn(vk::StaticFn {
            get_instance_proc_addr: Global::<T>::get_instance_proc_addr,
        })
    }
}

fn with_instance<T: Layer>(f: impl FnOnce(&ash::Entry, &ash::Instance)) {
    let entry = create_entry::<T>();
    extern "system" fn get_instance_proc_addr(
        instance: vk::Instance,
        p_name: *const i8,
    ) -> vk::PFN_vkVoidFunction {
        extern "system" fn create_instance(
            _: *const vk::InstanceCreateInfo,
            _: *const vk::AllocationCallbacks,
            instance: *mut vk::Instance,
        ) -> vk::Result {
            let inner: Box<*const c_void> = Box::new(1234 as *const c_void);
            *unsafe { instance.as_mut() }.unwrap() =
                vk::Instance::from_raw(Box::leak(inner) as *mut _ as _);
            vk::Result::SUCCESS
        }
        extern "system" fn destroy_instance(
            instance: vk::Instance,
            _: *const vk::AllocationCallbacks,
        ) {
            let inner = unsafe { Box::from_raw(instance.as_raw() as *mut *const c_void) };
            drop(inner);
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
            match name {
                "vkDestroyInstance" => Some(unsafe {
                    std::mem::transmute(destroy_instance as vk::PFN_vkDestroyInstance)
                }),
                _ => None,
            }
        }
    }
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
    let create_instance_info = vk::InstanceCreateInfo::builder().push_next(&mut layer_create_info);
    let instance = unsafe { entry.create_instance(&create_instance_info, None) }.unwrap();
    assert_ne!(instance.handle(), vk::Instance::null());
    f(&entry, &instance);
    unsafe { instance.destroy_instance(None) };
}

mod get_instance_proc_addr {
    use super::*;
    mod null_instance {
        use super::*;
        #[test]
        fn test_should_return_fp_when_called_with_get_instance_proc_addr_name() {
            #[derive(Default)]
            struct Tag;
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
        #[ignore]
        fn test_should_return_null_when_called_with_core_instance_dispatchable_command() {
            #[derive(Default)]
            struct Tag;
            let entry = create_entry::<MockLayer<Tag>>();
            assert!(unsafe {
                entry.get_instance_proc_addr(
                    vk::Instance::null(),
                    b"vkDestroyInstance\0".as_ptr() as *const i8,
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

        use super::*;

        #[test]
        fn test_should_return_null_for_global_commands() {
            #[derive(Default)]
            struct Tag;
            with_instance::<MockLayer<Tag>>(|entry, instance| {
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
            });
        }

        #[test]
        fn test_should_return_fp_for_get_instance_proc_addr() {
            #[derive(Default)]
            struct Tag;
            with_instance::<MockLayer<Tag>>(|entry, instance| {
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
            });
        }

        #[test]
        fn test_should_return_fp_for_core_dispatchable_command() {
            #[derive(Default)]
            struct Tag;
            with_instance::<MockLayer<Tag>>(|entry, instance| {
                let destroy_instance = unsafe {
                    entry.get_instance_proc_addr(
                        instance.handle(),
                        b"vkDestroyInstance\0".as_ptr() as *const i8,
                    )
                };
                assert!(destroy_instance.is_some());
                // vkDestroyInstance will be called in with_instance after this function is
                // returned.
            });
        }

        #[test]
        #[ignore]
        fn test_should_return_fp_for_enabled_instance_extension_command() {
            todo!("Not easy to test until we allow customize layer extensions.")
        }

        #[test]
        #[ignore]
        fn test_if_extension_is_not_enabled_null_should_be_returned() {
            #[derive(Default)]
            struct Tag;
            with_instance::<MockLayer<Tag>>(|entry, instance| {
                let destroy_surface = unsafe {
                    entry.get_instance_proc_addr(
                        instance.handle(),
                        b"vkDestroySurfaceKHR\0".as_ptr() as *const i8,
                    )
                };
                assert!(destroy_surface.is_none());
            });
        }

        #[test]
        #[ignore]
        fn test_should_call_into_next_get_instance_proc_addr_with_unknown_name() {}
    }

    #[test]
    #[ignore]
    fn test_should_move_layer_instance_link_forward() {
        todo!()
    }
}
