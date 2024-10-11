"""Provide CLI utility for interfacing with data load and fetch operations."""

from pathlib import Path

import click

from vrs_index_vcf import load as load_vcf
from vrs_index_vcf.fetch import fetch_by_vrs_ids
from vrs_index_vcf.output import generate_csv


@click.group()
def cli() -> None:
    """Index VRS-annotated VCFs"""


@cli.command()
@click.argument(
    "vcfs",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, readable=True, path_type=Path
    ),
    nargs=-1,
)
@click.option(
    "db-location",
    type=click.Path(
        file_okay=True, dir_okay=True, readable=True, writable=True, path_type=Path
    ),
)
def load(vcfs: tuple[Path], db_location: Path) -> None:
    """Index the VRS annotations in a VCF by loading it into the sqlite DB.

    \f
    :param vcf_path: path to VCF to ingest
    """
    if db_location.is_dir():
        db_location = db_location / "vrs_vcf_index.db"
    for vcf in vcfs:
        load_vcf.load_vcf(vcf)


@cli.command()
@click.argument(
    "vrs-ids",
    nargs=-1,
)
@click.option(
    "-o",
    "--output",
    type=click.Path(readable=True, writable=True, path_type=Path),
)
def fetch_by_id(vrs_ids: list[str], output: Path | None) -> None:
    """Fetch VCF positions by VRS ID"""
    if not vrs_ids:
        return
    rows = fetch_by_vrs_ids(vrs_ids)
    formatted_rows = [(f"ga4gh:VA.{r[0]}", r[1], str(r[2])) for r in rows]
    if output:
        generate_csv(rows, output)
    else:
        for row in formatted_rows:
            click.echo(",".join(row))
