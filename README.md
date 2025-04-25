# vrsix: Indexing VRS-Annotated VCFs

## Overview

`vrsix` provides a file-based indexing strategy to support fast lookup of AnVIL-hosted VCFs using IDs and annotations drawn from the [GA4GH Variation Representation Specification](https://www.ga4gh.org/product/variation-representation/).

See the [vrsix Terra workflow](https://github.com/gks-anvil/vrsix-workflow) for a readymade Terra implementation.

## Usage

To get started, you will need a fully VRS-annotated VCF. Confirm that your VCF contains the following info fields:
- VRS_Allele_IDs
- VRS_Starts
- VRS_Stops
- VRS_States

For example, confirm that `chr1.vcf` has the required fields.

```bash
bcftools view -h chr1.vcf | grep '^##INFO='
```

From a VRSified VCF, ingest a VRS ID and the corresponding VCF-called location (i.e. sufficient inputs for a tabix lookup), and store them in a sqlite database.

```shell
vrsix load chr1.vcf
```

Each variation is stored with an associated file URI to support later retrieval. By default, this URI is simply the input VCF's location in the file system, but you may declare a custom URI instead as an optional argument:

```shell
vrsix load chr1.vcf gs://my_stuff/chr1.vcf
```

By default, all records are ingested into a sqlite file located at `~/.local/share/vrsix.db`. This can be overridden with either the environment variable `VRS_VCF_INDEX`, or with an optional flag to the CLI:

```shell
vrsix load --db-location=./vrsix.db input.vcf
```

## Development

Ensure that a recent version of the [Rust toolchain](https://www.rust-lang.org/tools/install) is available.

Create a virtual environment and install developer dependencies:

```shell
python3 -m venv venv
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

Run tests from the project root with `pytest`:
```shell
pytest
```

Some granular tests are written directly into the Rust backend as well:

```shell
cd rust/
cargo test
```
