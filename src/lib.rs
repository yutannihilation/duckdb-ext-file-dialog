extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, Inserter as _, LogicalTypeId},
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use std::error::Error;

struct ChooseFileFunc;

impl VScalar for ChooseFileFunc {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        _input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let output_flat = output.flat_vector();
        output_flat.insert(0, "hello!");
        Ok(())
    }

    fn signatures() -> Vec<duckdb::vscalar::ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![],
            LogicalTypeId::Varchar.into(),
        )]
    }
}

const FUNCITON_NAME: &str = "choose_file";

#[duckdb_entrypoint_c_api]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_scalar_function::<ChooseFileFunc>(FUNCITON_NAME)
        .expect("Failed to register choose_file()");
    Ok(())
}
