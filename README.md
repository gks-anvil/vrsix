# GREGoR VRS Index

Proof of concept for sqlite-based indexing of GREGoR VCFs annotated with VRS IDs and attributes.

From a VCF, ingest a VRS ID and the corresponding VCF-called location (i.e. sufficient inputs for a tabix lookup), and store them in a sqlite database.

```shell
% vrs-index-vcf load gregor_joint_chr3_annotated.vcf
```

Given a VRS ID, retrieve VCF-associated data (output format TBD)

```shell
% vrs-index-vcf fetch-id HBtEUibAIKjZxBALaDFky0RDf56_IKT6
DbRow { vrs_id: "HBtUUibAIKjZxIALaDFky0RDf56_LKT6", chr: "3", pos: 10125 }
```

Or fetch all rows within a coordinate range:

```shell
% vrs-index-vc fetch-range 3 10110 10140
[DbRow { vrs_id: "HBtEtibAIKjZxBALaDFky0RDft6_IKT6", chr: "3", pos: 10125 }, DbRow { vrs_id: "2d0wfnw9IQpcf7ACZ5tla-NtP-u34Vmj", chr: "3", pos: 10128 }, DbRow { vrs_id: "PXiNdKr3kDiKJtkjolwiYpisen68Vz1d", chr: "3", pos: 10136 }, DbRow { vrs_id: "zs65-UuparftqeXlL_ZhSCfZiBmbpr49", chr: "3", pos: 10137 }, DbRow { vrs_id: "3Cf2iW7hC1bMORih10jXS40nwE5tdaD8", chr: "3", pos: 10138 }, DbRow { vrs_id: "EaMAbMRxCU4EkasuuX5rLupA-SXPpi6O", chr: "3", pos: 10138 }]
```

## Set up for development

Ensure that a recent version of the [Rust toolchain](https://www.rust-lang.org/tools/install) is available.

To compile:

```shell
% cargo build
```

This creates a binary under the `target/debug` subdirectory, executable a la:

```shell
./target/debug/vrs-index-vcf
```

`cargo run` compiles and executes. Pass CLI args after the `--` argument delimiter:

```shell
cargo run -- fetch-id HBtEUibAIKjZxBALaDFky0RDf56_IKT6
```

To test (e.g. time) a release-quality build (see [here](https://doc.rust-lang.org/book/ch14-01-release-profiles.html) for background):

```shell
cargo build --release
./target/release/vrs-index-vcf
```

## Run tests

```shell
cargo test
```

Caveat: I haven't written any tests yet

## Notes toward deployment

* Create binaries in CI/CD with [cargo-dist](https://github.com/axodotdev/cargo-dist?tab=readme-ov-file) -> fetch + run from GitHub release
