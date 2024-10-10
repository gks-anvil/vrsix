# GREGoR VRS Index

Proof of concept for sqlite-based indexing of GREGoR VCFs annotated with VRS IDs and attributes.

From a VCF, ingest a VRS ID and the corresponding VCF-called location (i.e. sufficient inputs for a tabix lookup), and store them in a sqlite database.

```shell
% vrs-index-vcf load gregor_joint_chr3_annotated.vcf
```

<!--Given a VRS ID, retrieve VCF-associated data (output format TBD)-->
<!---->
<!--```shell-->
<!--% vrs-index-vcf fetch-id HBtEUibAIKjZxBALaDFky0RDf56_IKT6-->
<!--DbRow { vrs_id: "HBtUUibAIKjZxIALaDFky0RDf56_LKT6", chr: "3", pos: 10125 }-->
<!--```-->
<!---->
<!--Or fetch all rows within a coordinate range:-->
<!---->
<!--```shell-->
<!--% vrs-index-vc fetch-range 3 10110 10140-->
<!--[DbRow { vrs_id: "HBtEtibAIKjZxBALaDFky0RDft6_IKT6", chr: "3", pos: 10125 }, DbRow { vrs_id: "2d0wfnw9IQpcf7ACZ5tla-NtP-u34Vmj", chr: "3", pos: 10128 }, DbRow { vrs_id: "PXiNdKr3kDiKJtkjolwiYpisen68Vz1d", chr: "3", pos: 10136 }, DbRow { vrs_id: "zs65-UuparftqeXlL_ZhSCfZiBmbpr49", chr: "3", pos: 10137 }, DbRow { vrs_id: "3Cf2iW7hC1bMORih10jXS40nwE5tdaD8", chr: "3", pos: 10138 }, DbRow { vrs_id: "EaMAbMRxCU4EkasuuX5rLupA-SXPpi6O", chr: "3", pos: 10138 }]-->
<!--```-->

## Set up for development

Ensure that a recent version of the [Rust toolchain](https://www.rust-lang.org/tools/install) is available.

The [Rust toolchain](https://www.rust-lang.org/tools/install) must be installed.

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
