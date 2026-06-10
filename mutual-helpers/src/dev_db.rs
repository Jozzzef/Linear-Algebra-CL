use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::file::writer::{FileWriter, Writer};
use parquet::schema::types::SchemaDescriptor;
use std::fs::File;

pub fn refresh_physical_device_database() {

    // IF FILE NOT EXISTS
    let schema = Schema::new(vec![
        Field::new("device_name", arrow::datatypes::GenericStringType, false),
        Field::new("exts_supported", arrow::datatypes::GenericStringType, false)
    ]);


    // REFRESH

}

pub fn select_pd_exact_match() {

}

pub fn select_pd_fuzzy_match() {

}
