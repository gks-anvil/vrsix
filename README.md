# vrsix: Indexing VRS-Annotated VCFs

Proof of concept for sqlite-based indexing of ANViL-hosted VCFs annotated with VRS IDs and attributes.

From a VCF, ingest a VRS ID and the corresponding VCF-called location (i.e. sufficient inputs for a tabix lookup), and store them in a sqlite database.

```shell
% vrsix load chr1.vcf
```

## Set up for development

Ensure that a recent version of the [Rust toolchain](https://www.rust-lang.org/tools/install) is available.

Create a virtual environment and install developer dependencies:

```shell
python3 -m virtualenv venv
source venv/bin/activate
python3 -m pip install -e '.[dev,tests]'
```

This installs Python code as editable, but after any changes to Rust code, run ``maturin develop`` to rebuild the Rust binary:

```shell
maturin develop
```

Be sure to install pre-commit hooks:

```shell
pre-commit install
```

Check Python style with `ruff`:

```shell
python3 -m ruff format . && python3 -m ruff check --fix .
```

Use `cargo fmt` to check Rust style (must be run from within the `rust/` subdirectory):

```shell
cd rust/
cargo fmt
```

Run tests with `pytest`:
```shell
pytest
```
