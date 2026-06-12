use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;

/// These are the extensions that can be used by any ability within this crate to do computations.
/// I.e. if something in this crate relies on this extension, then add it here.
/// This is done to keep the amount of extensions to a minimum
/// These also act as columns in our physical device lookup
const GLOBAL_NEEDED_EXTENSIONS: [&str; 6] = [
    "VK_KHR_shader_subgroup", // subgroup operations
    "VK_KHR_push_descriptor", // push descriptors
    "VK_KHR_compute_shader_derivatives", // if you need derivatives in compute shaders
    "VK_KHR_shader_float_controls", // precise control over floating-point semantics
    "VK_EXT_sampler_filter_minmax", // custom filtering in image loads
    "VK_KHR_shader_atomic_int64", // 64-bit atomic operations
];

/// This is to actually get the list of physical devices
/// Since we will interface with a database, there is no need for this to be public
fn query_physical_devices(inst: Arc<Instance>) -> Vec<(Arc<PhysicalDevice>, String, Vec<String>)> {
    let dev_iter: Vec<Arc<PhysicalDevice>> = inst.enumerate_physical_devices().unwrap().collect();
    let mut collect_vec: Vec<(Arc<PhysicalDevice>, String, Vec<String>)> = vec![];
    for pd in dev_iter {
        let se: Vec<String> = pd.supported_extensions().into_iter().map(|s| String::from(s.0)).collect();
        let dn: String = pd.properties().device_name.clone();
        collect_vec.push( (pd, dn, se) ); 
    }
    collect_vec
}

/// This is to actually get the list of physical devices properties, not the physical device type
/// Since we will interface with a database, there is no need for this to be public
fn query_physical_device_props(inst: Arc<Instance>) -> Vec<(String, Vec<String>)> {
    let dev_iter: Vec<Arc<PhysicalDevice>> = inst.enumerate_physical_devices().unwrap().collect();
    let mut collect_vec: Vec<(String, Vec<String>)> = vec![];
    for pd in dev_iter {
        let se: Vec<String> = pd.supported_extensions().into_iter().map(|s| String::from(s.0)).collect();
        let dn: String = pd.properties().device_name.clone();
        collect_vec.push( (dn, se) ); 
    }
    collect_vec
}

#[derive(Clone, Debug)]
enum StringOrBool {
    S(String),
    B(bool)
}

// spawn a worker thread to do this, since we need this list only last after building the users
// computation statements
// save it as parquet for easy retrieval
pub fn create_or_refresh_physical_device_database(inst: Arc<Instance>) {

    // IF FILE NOT EXISTS
    let ext_vec = GLOBAL_NEEDED_EXTENSIONS.to_vec();
    let mut fields: Vec<Field> = ext_vec.iter().map(|x| Field::new(*x, DataType::Boolean, false)).collect();
    fields.insert(0, Field::new("device_name", DataType::Utf8, false));
    let schema = Schema::new(fields.clone());
    println!("{:?}", schema);

    // REFRESH
    let vec_of_dev_and_ext = query_physical_device_props(inst);
    let mut vec_of_columns: Vec<Vec<StringOrBool>> = vec![Vec::new(); fields.len()];
    for x in vec_of_dev_and_ext {
        println!("------------");
        println!("{:?}", x);

        for (i, f) in fields.iter().enumerate() {
            if i == 0 {
                vec_of_columns[0].push(StringOrBool::S(x.0.clone()))
            } else if x.1.iter().any(|e| e.to_lowercase() == f.name().to_lowercase()) { 
                vec_of_columns[i].push(StringOrBool::B(true))
            } else {
                vec_of_columns[i].push(StringOrBool::B(false))
            }
        }
        println!("+++");
        println!("{:?}", vec_of_columns);

        
    }

    

}

// If the user wants to explicitely use a device it should be used whatever the final command in
// the sequence of statements is to actually deploy the computation, i.e. pass in the name of the device as a parameter
pub fn retrieve_necessary_device_or_user_specified_device(minimal_ext_needed: Vec<String>) {
}



