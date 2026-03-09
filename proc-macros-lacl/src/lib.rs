use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use proc_macro2::TokenTree as TT2;
use quote::quote;
//use vulkano::device::physical::PhysicalDevice;

/// More convienent syntax to quickly get physical devices
/// first parameter = an "array" of PhysicalDevice Type, ordered from left to right in important to choose (i.e. the first element is plan-a, second is plan-b, etc)
/// second parameter = an "array" of extensions you want to your device to have, case-insensitive elements (i.e. if a physical device has such an extension, then keep it)
/// returns: a single physical device + the index of the compute queue within the queue family
/// potential output: if there are multiple devices that fit within your spec, then it returns the first one + prints out the names of the other devices which were discarded
///
/// Alternative usage: just pass in the name of the device you specifically want to choose, if not found it lists out the names of the devices available (with metadata)
#[proc_macro]
pub fn get_physical_device_and_queue_index(
        input_tokens: TokenStream
) -> TokenStream {
        let it_ts2: TS2 = input_tokens.into();
        for t in it_ts2 {
            match t { // those four variants are exhaustive
                TT2::Group(group) => println!("Found Group: {:?}", group),
                TT2::Ident(ident) => println!("Found Ident: {:?}", ident),
                TT2::Punct(punct) => println!("Found Punct: {:?}", punct),
                TT2::Literal(literal) => println!("Found Literal: {:?}", literal)
            } 
        }

        let output: TS2 = quote! {
                (23, 32)
                //(PhysicalDevice, i8)
        };
        output.into()
}
