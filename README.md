# vrsix: Indexing VRS-Annotated VCFs

Proof of concept for sqlite-based indexing of ANViL-hosted VCFs annotated with VRS IDs and attributes.

From a VCF, ingest a VRS ID and the corresponding VCF-called location (i.e. sufficient inputs for a tabix lookup), and store them in a sqlite database.

```shell
% vrsix load chr1.vcf
```

Given a VRS ID, retrieve VCF-associated data (output format TBD)

```shell
% vrsix fetch-by-id --db-location=sqlite.db dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG
ga4gh:VA.dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG,1,783006
```

Or fetch all rows within a coordinate range:

```shell
% vrsix fetch-by-range --db-location=sqlite.db 1 783000 783200
ga4gh:VA.dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG,1,783006
ga4gh:VA.MiasxyXMXtOpsZgGelL3c4QgtflCNLHD,1,783006
ga4gh:VA.5cY2k53xdW7WeHw2WG1HA7jl50iH-r9p,1,783175
ga4gh:VA.jHaXepIvlbnapfPtH_62y-Qm81hCrBYn,1,783175
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

<!--Run tests with `pytest`:-->
<!---->
<!--```shell-->
<!--pytest-->
<!--```-->
