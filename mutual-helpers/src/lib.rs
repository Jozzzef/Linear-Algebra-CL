//use std::path::{Path, PathBuf};
use std::sync::Arc;
use vulkano::VulkanLibrary;
use vulkano::library::DynamicLibraryLoader;
use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Queue;
//use core::iter::ExactSizeIterator;
use std::any::type_name;
pub mod dev_db;


// Choice 1: Just get the default Vulkan Loader
// Choice 2: Specify the path to the Loader you want (e.g. String::from("/libvulkan.so"))
#[derive(Debug)]
pub enum ChooseLoader {
    Default,
    PathOfLoader(String),
}

/// Offload all possible handling of vulkan library loading to this function
/// Panic on error for this function since everything else depends on accessing vulkan
pub fn get_vulkan_library(choose_loader: ChooseLoader) -> Arc<VulkanLibrary> {
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

pub fn query_devices_and_ext(inst: Arc<Instance>) -> Vec<(Arc<PhysicalDevice>, Vec<String>)> {
    let dev_iter: Vec<Arc<PhysicalDevice>> = inst.enumerate_physical_devices().unwrap().collect();
    let mut collect_vec: Vec<(Arc<PhysicalDevice>, Vec<String>)> = vec![];
    for pd in dev_iter {
        let se: Vec<String> = pd.supported_extensions().into_iter().map(|s| String::from(s.0)).collect();
        println!(
            "{} | Extensions -> {:#?} | Features -< {:#?} | Compute Memory Size --> {}",
            pd.properties().device_name,
            se ,
            pd.supported_features(),
            pd.properties().max_compute_shared_memory_size
        );
        println!("supported extension types ->");
        print_type(pd.supported_extensions());
        collect_vec.push( (pd, se) ); 
    }

    collect_vec
}
 
pub fn print_type<T>(_: T) {
    println!("Type: {}", type_name::<T>());
}

// v1: a vector of PhysicalDevice Type, ordered from left to right in important to choose (i.e. the first element is plan-a, second is plan-b, etc)
// v2: a vector of extensions you want to your device to have, case-insensitive elements (i.e. if a physical device has such an extension, then keep it)

//pub fn get_physical_device_and_queue_index(
//    instance: Instance, 
//    v1: Option<Vec<&str>>, 
//    v2: Option<Vec<&str>>
//) -> (PhysicalDevice, Queue) {
//    
//    let p: PhysicalDevice = match v1 {
//        // user-chosen device, either exact or fuzzy-found string
//        Some(s) => {
//            println!("Some(s) Inside match statement for v1 {:?}", s);
//        },
//        None => {
//            println!("None Inside match statement for v1 {:?}", s);
//        }
//    }
//
//    let e: Queue = match v2 {
//        // user-chosen device, either exact or fuzzy-found string
//        Some(s) => {
//            println!("Some(s) Inside match statement for v1 {:?}", s);
//        },
//        None => {
//            println!("None Inside match statement for v1 {:?}", s);
//        }
//    }
//    
//}
