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
use rfd::FileDialog;
use std::error::Error;

struct ChooseFileFunc;

impl VScalar for ChooseFileFunc {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        _input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dialog = match std::env::current_dir() {
            Ok(cur_dir) => FileDialog::new().set_directory(cur_dir),
            Err(_) => FileDialog::new(),
        };
        let path = dialog.pick_file().ok_or("Failed to get file")?;
        let path_str = path
            .to_str()
            .ok_or("The path contains non-UTF-8 character")?;

        let output_flat = output.flat_vector();
        output_flat.insert(0, path_str);
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
