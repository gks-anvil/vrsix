use pyo3::{create_exception, exceptions, prelude::*, Python};
mod load;
mod sqlite;
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[pyfunction]
pub fn vcf_to_sqlite(vcf_path: PathBuf, db_url: String) -> PyResult<()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(load::load_vcf(vcf_path, &db_url))?;
    Ok(())
}

create_exception!(loading_module, VrsixError, exceptions::PyException);
create_exception!(loading_module, SqliteFileError, VrsixError);
create_exception!(loading_module, VcfError, VrsixError);

#[pymodule]
#[pyo3(name = "_core")]
fn loading_module(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(vcf_to_sqlite, m)?);
    m.add("VrsixError", py.get_type_bound::<VrsixError>())?;
    m.add("SqliteFileError", py.get_type_bound::<SqliteFileError>())?;
    m.add("VcfError", py.get_type_bound::<VcfError>())?;
    Ok(())
}
