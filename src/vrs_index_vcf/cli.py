"""Provide CLI utility for interfacing with data load and fetch operations."""

from pathlib import Path

import click

from vrs_index_vcf import load


@click.group()
def cli() -> None:
    """TODO"""


@cli.command()
@click.argument(
    "vcf_path",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, readable=True, path_type=Path
    ),
)
def load_vcf(vcf_path: Path) -> None:
    """Index the VRS annotations in a VCF by loading it into the sqlite DB.

    \f
    :param vcf_path: path to VCF to ingest
    """
    load.load_vcf(vcf_path)
