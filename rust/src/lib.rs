use pyo3::{create_exception, exceptions, prelude::*, Python};
pub mod load;
pub mod parse;
pub mod sqlite;
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[pyfunction]
#[pyo3(signature = (vcf_path, db_url, vcf_uri = None))]
pub fn vcf_to_sqlite(vcf_path: PathBuf, db_url: String, vcf_uri: Option<String>) -> PyResult<()> {
    let uri_value =
        vcf_uri.unwrap_or_else(|| format!("file://{}", vcf_path.to_string_lossy().into_owned()));
    let rt = Runtime::new().unwrap();
    rt.block_on(load::load_vcf(vcf_path, &db_url, uri_value))?;
    let _ = sqlite::cleanup_tempfiles(&db_url);
    Ok(())
}

// hard-coded VRSIX schema value.
// At some point, we could provide some kind of migration support if we ever need to
// change things, but in the meantime this provides some basic confirmation that the
// target index file should probably have a familiar schema.
const VRSIX_SCHEMA_VERSION: &str = "1";

create_exception!(loading_module, VrsixError, exceptions::PyException);
create_exception!(loading_module, SqliteFileError, VrsixError);
create_exception!(loading_module, VcfError, VrsixError);
create_exception!(loading_module, VrsixDbError, VrsixError);
create_exception!(loading_module, FiletypeError, VrsixError);

#[pymodule]
#[pyo3(name = "_core")]
fn loading_module(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(vcf_to_sqlite, m)?);
    m.add("VrsixError", py.get_type::<VrsixError>())?;
    m.add("SqliteFileError", py.get_type::<SqliteFileError>())?;
    m.add("VcfError", py.get_type::<VcfError>())?;
    m.add("VrsixDbError", py.get_type::<VrsixDbError>())?;
    m.add("FiletypeError", py.get_type::<FiletypeError>())?;
    Ok(())
}
