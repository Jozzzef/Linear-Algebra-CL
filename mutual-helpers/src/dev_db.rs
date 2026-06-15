use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow::array::{StringArray, BooleanArray, ArrayRef};
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;

/// These are the extensions that can be used by any ability within this crate to do computations.
/// I.e. if something in this crate relies on this extension, then add it here.
/// This is done to keep the amount of extensions to a minimum
/// These also act as columns in our physical device lookup
const GLOBAL_NEEDED_EXTENSIONS: [&str; 4] = [
    "VK_KHR_shader_subgroup", // subgroup operations
    "VK_KHR_push_descriptor", // push descriptors
    "VK_KHR_compute_shader_derivatives", // if you need derivatives in compute shaders
    "VK_KHR_shader_float_controls", // precise control over floating-point semantics "VK_EXT_sampler_filter_minmax", // custom filtering in image loads "VK_KHR_shader_atomic_int64", // 64-bit atomic operations
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

impl StringOrBool {
    pub fn as_string(&self) -> String {
        match self {
            StringOrBool::S(s) => s.clone(),
            StringOrBool::B(b) => b.to_string(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            StringOrBool::S(_) => false,
            StringOrBool::B(b) => *b,
        }
    }

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

        let batch = matrix_to_record_batch(&vec_of_columns, &schema);
    
        //write to file
        let file = File::create("data.parquet").unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(schema.clone()), None).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();
        
    }

    

}

fn matrix_to_record_batch(m: &Vec<Vec<StringOrBool>>, schema: &Schema) -> RecordBatch {
   
    let mut vec_of_arrays: Vec<ArrayRef> = vec![];
    for c in m {
        match c[0] {
            StringOrBool::S(_) => {
                let vec_string: Vec<String> = c.iter().map(|s| s.as_string()).collect();
                vec_of_arrays.push(Arc::new(StringArray::from(vec_string)))
            }
            StringOrBool::B(_) => {
                let vec_bool: Vec<bool> = c.iter().map(|s| s.as_bool()).collect();
                vec_of_arrays.push(Arc::new(BooleanArray::from(vec_bool)))
            }
        }
    }

    let output = arrow::record_batch::RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec_of_arrays,
    ); 

    match output {
        Ok(o) => {
            println!("RecordBatch Ok => {:?}", o);
            o
        }
        Err(e) => {
            println!("ERROR in RecordBatch => {:?}", e);
            let vec_of_arrays: Vec<ArrayRef> = vec![Arc::new(StringArray::from(vec!["ERROR"]))];
            arrow::record_batch::RecordBatch::try_new(
                Arc::new( Schema::new(vec![Field::new("error_placeholder", DataType::Utf8, false)]) ),
                vec_of_arrays,
            ).unwrap()
        }
    }
}

// If the user wants to explicitely use a device it should be used whatever the final command in
// the sequence of statements is to actually deploy the computation, i.e. pass in the name of the device as a parameter
pub fn retrieve_necessary_device_or_user_specified_device(minimal_ext_needed: Vec<String>) {
}



