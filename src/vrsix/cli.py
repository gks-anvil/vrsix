"""Provide CLI utility for interfacing with data loading operations."""

import logging
from pathlib import Path
from timeit import default_timer as timer

import click

from vrsix import load as load_vcf

_logger = logging.getLogger(__name__)


def _configure_logging() -> None:
    """Configure Python-side logging."""
    logging.basicConfig(
        filename=f"{__package__}.log",
        format="[%(asctime)s] - %(name)s - %(levelname)s : %(message)s",
    )
    logging.getLogger(__package__).setLevel(logging.INFO)


@click.group()
def cli() -> None:
    """Index VRS-annotated VCFs"""
    _configure_logging()


@cli.command()
@click.argument(
    "vcf",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, readable=True, path_type=Path
    ),
    nargs=1,
)
@click.argument(
    "uri",
    default=None,
    required=False,
)
@click.option(
    "--db-location",
    type=click.Path(
        file_okay=True, dir_okay=True, readable=True, writable=True, path_type=Path
    ),
)
def load(vcf: Path, uri: str, db_location: Path | None) -> None:
    """Index the VRS annotations in a VCF by loading it into the sqlite DB.

    Optionally provide a custom file URI to describe how to retrieve VCF records after
    index lookup:

        % vrsix load input.vcf gs://my_storage/input.vcf

    \f
    :param vcf_path: path to VCF to ingest
    """
    if db_location and db_location.is_dir():
        db_location = db_location / "vrs_vcf_index.db"
    start = timer()
    load_vcf.load_vcf(vcf, db_location, uri)
    end = timer()
    _logger.info("Processed `%s` in %s seconds", vcf, end - start)
