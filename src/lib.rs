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
use libduckdb_sys::{self as ffi, duckdb_string_t, duckdb_string_t_data, duckdb_string_t_length};
use rfd::FileDialog;
use std::error::Error;

struct ChooseFileFunc;

impl VScalar for ChooseFileFunc {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dialog = match std::env::current_dir() {
            Ok(cur_dir) => FileDialog::new().set_directory(cur_dir),
            Err(_) => FileDialog::new(),
        };

        let dialog = match get_filter_option(input) {
            Some(filter) => dialog.add_filter("specified extension", &[filter]),
            None => dialog,
        };

        let paths = dialog.pick_files().ok_or("Failed to get file")?;
        let paths_str = paths
            .iter()
            .map(|x| x.to_str().ok_or("The path contains non-UTF-8 character"))
            .collect::<Result<Vec<&str>, &str>>()?;

        let output_flat = output.flat_vector();
        for (i, p) in paths_str.into_iter().enumerate() {
            output_flat.insert(i, p);
        }
        Ok(())
    }

    fn signatures() -> Vec<duckdb::vscalar::ScalarFunctionSignature> {
        // i.e. choose_file()
        let sig_any = ScalarFunctionSignature::exact(vec![], varchar_ty());
        // i.e. choose_file('.csv')
        let sig_with_filter = ScalarFunctionSignature::exact(vec![varchar_ty()], varchar_ty());

        vec![sig_any, sig_with_filter]
    }
}

fn get_filter_option(input: &mut DataChunkHandle) -> Option<String> {
    match input.num_columns() {
        0 => None,
        1 => {
            let input_vec = input.flat_vector(0);
            let mut option_raw = input_vec.as_slice_with_len::<duckdb_string_t>(input.len())[0];

            // TODO: duckdb-rs doesn't prvide a way to get string from a FlatVector.
            // (DuckString::new() is pub(crate) so unavailable).
            let option = unsafe {
                let len = duckdb_string_t_length(option_raw);
                let c_ptr = duckdb_string_t_data(&mut option_raw);
                let string = String::from_utf8_lossy(std::slice::from_raw_parts(
                    c_ptr as *const u8,
                    len as usize,
                ));

                // This is very important. Cow refers to DuckDB's memory, which
                // might can be erased. So, it needs to be copied into Rust.
                string.to_string()
            };

            // in case option is provided with dot, remove it (e.g. ".csv" -> "csv")
            if let Some(stripped) = option.strip_prefix('.') {
                Some(stripped.to_string())
            } else {
                Some(option)
            }
        }
        _ => unreachable!("Wrong number of options are provided"),
    }
}

fn varchar_ty() -> duckdb::core::LogicalTypeHandle {
    LogicalTypeId::Varchar.into()
}

const FUNCITON_NAME: &str = "choose_file";

#[duckdb_entrypoint_c_api]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_scalar_function::<ChooseFileFunc>(FUNCITON_NAME)
        .expect("Failed to register choose_file()");
    Ok(())
}
