//#[cfg(test)]
mod tests {
    use mutual_helpers::{ChooseLoader, get_vulkan_library, dev_db};
    use std::sync::Arc;
    use vulkano::{VulkanLibrary};
    use vulkano::instance::{InstanceCreateInfo, InstanceCreateFlags, Instance};
 
    #[test]
    fn my_test() {

        let library: Arc<VulkanLibrary> = get_vulkan_library(ChooseLoader::Default);
        let instance_args = InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default() // Struct Update Syntax
        };
        let instance: Arc<Instance> = Instance::new(library, instance_args).unwrap();
        dev_db::create_or_refresh_physical_device_database(instance.clone());
    //     assert_eq!(mutual_helpers::create_or_refresh_physical_device_database(), "");
    }

    #[test]
    fn just_printing() {
        println!("This is my debugging message");
    }
}
