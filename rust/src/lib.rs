use pyo3::prelude::*;
mod load;
mod sqlite;
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[pyfunction]
pub fn vcf_to_sqlite(vcf_path: PathBuf, db_url: String) -> PyResult<()> {
    let rt = Runtime::new().unwrap();
    _ = rt.block_on(load::load_vcf(vcf_path, &db_url));
    Ok(())
}

#[pymodule]
#[pyo3(name = "_core")]
fn loading_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vcf_to_sqlite, m)?)
}
