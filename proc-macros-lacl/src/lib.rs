use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use proc_macro2::TokenTree as TT2;
use quote::quote;
use std::cmp::min;
use mutual_helpers::{ChooseLoader, get_vulkan_library, query_devices_and_ext};

//use vulkano::device::physical::PhysicalDevice;

/// More convienent syntax to quickly get physical devices
/// zero-th parameter: the instance, we will use this to query the devices 
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

        // !! NEED TO POP OUT THE INSTANCE INTO ITS OWN VARIABLE

        // get the upper level groups, should only be two
        let mut vec_groups: Vec<proc_macro2::Group> = vec![];
        let mut vec_arguments: Vec<Vec<String>> = vec![vec![]];
        for t in it_ts2 {
            if let TT2::Group(group) = t { vec_groups.push(group); }
        };
        let vec_groups_len = vec_groups.len();
        if (vec_groups_len == 1) | (vec_groups_len > 2) {
            println!("Error: the number of parameters must be 0 or 2");
        } else if vec_groups_len == 0 {
            println!("opinionated defaults");
        } else {
            println!("VEC_GROUPS -> {:?}", vec_groups);
            for (index, g) in vec_groups.iter().enumerate() {
                for gs in g.stream() {
                    if let TT2::Literal(literal) = gs { 
                        // instead of creating a new vector for each individual element, we want to
                    // keep just for a single group
                        if index + 1 > vec_arguments.len() {
                            vec_arguments.push(vec![]); // add a new vector to act as the new list of
                        }
                    // params
                        vec_arguments[index].push(literal.to_string().replace("\"", ""));
                        println!("{} {:?}", index, &vec_arguments[index]);
                        println!("===================");
                    }
                } 
            }
            // Now can do work within normal rust syntax; this way of doing it is ideal for
            // debugging but maybe not performance, need to reconsider this entire function/macro later
            println!("VEC_ARGS -> {:?}", vec_arguments);

            // How to prioritize strings for search
            // 1) all strings -> lowercase + find entire substring
                // 1.FALSE) Leventstein distance, return the one with lowest distance
                // 1.TRUE) 
                    // 2) If only one is found, return that
                        // 2.FALSE) multiple found, prioritize based on:
                            // 3) Ranking: Normalized vectors in the dimensions [CUs, Memory],
                            //    return the vector with the greatest magnitude
        
            //collect physical devices + extensions available into a vec of tuples
            let tpl_pd_ext = query_devices_and_ext();
            
            
        }

        let output: TS2 = quote! {
                (23, 32)
                //(PhysicalDevice, i8)
        };
        output.into()
}


// TEXT HELPERS ====================================
// This was based on mbrlabs/distance crate
fn levenshtein(s: &str, t: &str) -> usize {
    // get length of unicode chars
    let len_s = s.chars().count();
    let len_t = t.chars().count();

    // initialize the matrix
    let mut mat: Vec<Vec<usize>> = vec![vec![0; len_t + 1]; len_s + 1];
    for i in 1..(len_s + 1) { 
        mat[i][0] = i; 
    }
    for i in 1..(len_t + 1) { 
        mat[0][i] = i; 
    }

    // apply edit operations
    for (i, s_char) in s.chars().enumerate() {
        for (j, t_char) in t.chars().enumerate() {
            let substitution = if s_char == t_char {0} else {1};
            mat[i+1][j+1] = min(
                min(
                    mat[i][j+1] + 1, // deletion
                    mat[i+1][j] + 1 // insertion
                ), 
                mat[i][j] + substitution    // substitution
            );
        }
    }
    mat[len_s][len_t]
}
