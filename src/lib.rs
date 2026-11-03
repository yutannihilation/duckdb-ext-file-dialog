use duckdb_loadable_macros::duckdb_entrypoint_c_api_v2;
use duckdb_neo::{
    DuckDBType, Result,
    connection::Context,
    data_chunk::VectorCollection,
    enums::{FunctionProperty, FunctionPropertyStability},
    error::Error,
    extension::Extension,
    scalar::{ScalarCallbacks, ScalarFunctionBuilder},
    signature::{FunctionDocumentationBuilder, FunctionParameter, SignatureBuilder},
    vector::{Unknown, Vector},
};
use rfd::FileDialog;

const FUNCTION_NAME: &str = "choose_file";

struct ChooseFileFunc;

impl ScalarCallbacks for ChooseFileFunc {
    type BindData = ();
    type InitData = ();

    fn exec(
        &self,
        _bind_data: Option<&Self::BindData>,
        _init_data: Option<&mut Self::InitData>,
        _context: &Context,
        input: &mut VectorCollection,
        output: Vector<'_, Unknown>,
    ) -> Result<()> {
        let dialog = match std::env::current_dir() {
            Ok(cur_dir) => FileDialog::new().set_directory(cur_dir),
            Err(_) => FileDialog::new(),
        };

        let filter = if input.col_count() > 0 {
            let vec = input.get_vector_at::<String>(0)?;
            vec.reader()?.get(0)?.map(|s| {
                if let Some(stripped) = s.strip_prefix('.') {
                    stripped.to_string()
                } else {
                    s.to_string()
                }
            })
        } else {
            None
        };

        let dialog = match filter.as_deref() {
            Some(filter) => dialog.add_filter("specified extension", &[filter]),
            None => dialog,
        };

        let path = dialog
            .pick_file()
            .ok_or_else(|| Error::invalid_input("Failed to get file"))?;
        let path_str = path
            .to_str()
            .ok_or_else(|| Error::invalid_input("The path contains non-UTF-8 character"))?;

        let row_count = input.row_count()?.max(1);
        let mut output = output.cast::<String>()?;
        output.set_size(row_count)?;
        let mut writer = output.writer()?;
        for i in 0..row_count {
            writer.set(i, Some(path_str))?;
        }
        Ok(())
    }
}

pub fn register(extension: &Extension) -> Result<()> {
    ScalarFunctionBuilder::new(
        FUNCTION_NAME,
        SignatureBuilder::new()
            .param(FunctionParameter::with_default("filter", None::<String>))
            .return_type(String::logical_type(extension)?)
            .documentation(
                FunctionDocumentationBuilder::new()
                    .description("Open a file dialog to pick a file and return the path.")
                    .add_example("Any file", "SELECT choose_file()")
                    .add_example("Filter by extension", "SELECT choose_file('.csv')"),
            ),
        ChooseFileFunc,
    )
    .set_property(FunctionProperty::HasSpecialNullHandling(true))
    .set_property(FunctionProperty::Stability(
        FunctionPropertyStability::Volatile,
    ))
    .register(extension)
}

#[duckdb_entrypoint_c_api_v2(extension_name = "file_dialog", api_version = "2.0.0")]
pub fn entrypoint(extension: &Extension) -> Result<()> {
    register(extension)
}
