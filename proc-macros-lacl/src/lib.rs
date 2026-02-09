use vulkano::device::physical::PhysicalDevice;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;

/// More convienent syntax to quickly get physical devices
/// first parameter = an array of PhysicalDevice Type, ordered from left to right in important to choose (i.e. the first element is plan-a, second is plan-b, etc)
/// second parameter = an array of extensions you want to your device to have, case-insensitive elements (i.e. if a physical device has such an extension, then keep it)
/// returns: a single physical device + the index of the compute queue within the queue family
/// potential output: if there are multiple devices that fit within your spec, then it returns the first one + prints out the names of the other devices which were discarded
///
/// Alternative usage: just pass in the name of the device you specifically want to choose, if not found it lists out the names of the devices available (with metadata)
#[proc_macro]
pub fn get_physical_device_and_queue_index(
        device_priority_order: TokenStream,
        necessary_extensions: TokenStream,
    ) -> TokenStream {
        let device_priority_order_ts2: TS2 = device_priority_order.into();
        let necessary_extensions_ts2: TS2 = necessary_extensions.into();


        let output: TS2 = quote! {
                
                (PhysicalDevice, i8)
        };
        output.into()
}



// #[macro_export]
// macro_rules! get_physical_device_and_queue_index {
//     ([$($dev_type:expr),*], [$($ext:expr),*]) => {
//         // extensions we want our physical devices to have
//         for dev_type in $ordered_device_types.iter() {
//             let dev_ext_to_filter_on = DeviceExtensions {
//                 khr_storage_buffer_storage_class: true,
//                 ..DeviceExtensions::empty()
//             };
//         }

//         println!("First: {:?}, Second: {:?}", $ordered_device_types, $extensions)
//         //return value (PhysicalDevice, i8)
//         (<>, <>)

//         // println!(
//         //     "Using device: {} (type: {:?})",
//         //     physical_device.properties().device_name,
//         //     physical_device.properties().device_type,
//         // );
//     };

//     // alternative usage -> get specific physical device, and if not found then just return the list of available physical devices
//     ($e1:expr) => {
//         println!("First: {:?}", $e1);
//     };
}
