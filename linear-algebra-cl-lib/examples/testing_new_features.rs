use linear_algebra_cl::init_helpers::{ChooseLoader, get_vulkan_library};
use std::sync::Arc;
use proc_macros_lacl::{get_physical_device_and_queue_index};
use vulkano::VulkanLibrary;
use vulkano::instance::{InstanceCreateInfo, InstanceCreateFlags, Instance};
use vulkano::device::physical::PhysicalDevice;

fn main() {
    // ===================================
    // STEP 1: GET VULKAN SHARED LIBRARY

    // let library: Arc<VulkanLibrary> = get_vulkan_library(ChooseLoader::PathOfLoader(String::from(
    //     "/usr/lib64/libvulkan.so.1.4.313",
    // )));

    let library: Arc<VulkanLibrary> = get_vulkan_library(ChooseLoader::Default);
    println!("{:?}", library);
    println!("---");

    // ===================================
    // STEP 2: INIT THE INSTANCE (MAPPING BETWEEN APP/VULKANO AND SHARED LIBRARY)

    let instance_args = InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..Default::default() // Struct Update Syntax
    };
    let instance: Arc<Instance> = Instance::new(library, instance_args).unwrap();
    println!("{:?}", instance);
    println!("---");

    // ===================================
    // STEP 3: ENUMERATE PHYSICAL DEVICES WITH FITLERS ON EXTENSIONS, RETURN PHYSICAL DEVICES & QUEUES
    let (physical_device, compute_queue_index) =
         get_physical_device_and_queue_index![["khr_storage_buffer_storage_class"], []];
    println!("physical_device -> {:?}, compute_queue_index -> {:?}", physical_device, compute_queue_index);
}
