"""Provide CLI utility for interfacing with data load and fetch operations."""

from pathlib import Path

import click

from ivrs import load as load_vcf
from ivrs.fetch import fetch_by_pos_range as vcf_fetch_by_pos_range
from ivrs.fetch import fetch_by_vrs_ids
from ivrs.output import generate_csv


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
    "--db-location",
    type=click.Path(
        file_okay=True, dir_okay=True, readable=True, writable=True, path_type=Path
    ),
)
def load(vcfs: tuple[Path], db_location: Path | None) -> None:
    """Index the VRS annotations in a VCF by loading it into the sqlite DB.

    \f
    :param vcf_path: path to VCF to ingest
    """
    if db_location and db_location.is_dir():
        db_location = db_location / "vrs_vcf_index.db"
    for vcf in vcfs:
        load_vcf.load_vcf(vcf)


@cli.command()
@click.argument(
    "vrs-ids",
    nargs=-1,
)
@click.option(
    "--db-location",
    type=click.Path(
        file_okay=True, dir_okay=True, readable=True, writable=True, path_type=Path
    ),
)
@click.option(
    "-o",
    "--output",
    type=click.Path(readable=True, writable=True, path_type=Path),
)
def fetch_by_id(
    vrs_ids: list[str], db_location: Path | None, output: Path | None
) -> None:
    """Fetch VCF positions by VRS ID"""
    if not vrs_ids:
        return
    rows = fetch_by_vrs_ids(vrs_ids, db_location)
    formatted_rows = [(f"ga4gh:VA.{r[0]}", r[1], str(r[2])) for r in rows]
    if output:
        generate_csv(formatted_rows, output)
    else:
        for row in formatted_rows:
            click.echo(",".join(row))


@cli.command()
@click.argument("chrom", required=True)
@click.argument(
    "start",
    type=click.INT,
    required=True,
)
@click.argument(
    "end",
    type=click.INT,
    required=True,
)
@click.option(
    "--db-location",
    type=click.Path(
        file_okay=True, dir_okay=True, readable=True, writable=True, path_type=Path
    ),
)
@click.option(
    "-o",
    "--output",
    type=click.Path(readable=True, writable=True, path_type=Path),
)
def fetch_by_range(
    chrom: str, start: int, end: int, db_location: Path | None, output: Path | None
) -> None:
    """Fetch VCF rows by position range.

    :param chrom: chromosome
    :param start: starting position
    :param end: ending position
    """
    rows = vcf_fetch_by_pos_range(chrom, start, end, db_location)
    formatted_rows = [(f"ga4gh:VA.{r[0]}", r[1], str(r[2])) for r in rows]
    if output:
        generate_csv(formatted_rows, output)
    else:
        for row in formatted_rows:
            click.echo(",".join(row))
