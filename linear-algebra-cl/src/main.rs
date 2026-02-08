use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use vulkano::{VulkanLibrary, instance::Instance, library::DynamicLibraryLoader};

mod init_helpers {
    use std::path::Path;
    use std::sync::Arc;
    use vulkano::VulkanLibrary;
    use vulkano::library::DynamicLibraryLoader;

    // Choice 1: Just get the default Vulkan Loader
    // Choice 2: Specify the path to the Loader you want (e.g. String::from("/libvulkan.so"))
    #[derive(Debug)]
    pub enum ChooseLoader<P: AsRef<Path>> {
        Default,
        PathOfLoader(P),
    }

    /// Offload all possible handling of vulkan library loading to this function
    /// Panic on error for this function since everything else depends on accessing vulkan
    pub fn get_vulkan_library<P: AsRef<Path>>(
        choose_loader: ChooseLoader<P>,
    ) -> Arc<VulkanLibrary> {
        match choose_loader {
            ChooseLoader::Default => match VulkanLibrary::new() {
                Ok(vl) => vl,
                Err(e) => panic!("{e}"),
            },
            ChooseLoader::PathOfLoader(s) => match unsafe { DynamicLibraryLoader::new(s) } {
                Ok(dll) => match VulkanLibrary::with_loader(dll) {
                    Ok(vl) => vl,
                    Err(e) => panic!("{e}"),
                },
                Err(e) => panic!("{e}"),
            },
        }
    }
}

fn main() {
    let library: Arc<VulkanLibrary> =
        init_helpers::get_vulkan_library(init_helpers::ChooseLoader::Default);
    let instance = Instance::new(
        library,
        &InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    .unwrap();
    println!("Hello, world!");
}
