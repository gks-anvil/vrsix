"""Load VCFs into sqlite index."""

import logging
from pathlib import Path

from vrsix._core import (
    FiletypeError,
    SqliteFileError,
    VcfError,
    VrsixDbError,
    VrsixError,
    vcf_to_sqlite,
)
from vrsix.sqlite import DEFAULT_SQLITE_LOCATION

__all__ = [
    "load_vcf",
    "vcf_to_sqlite",
    "FiletypeError",
    "SqliteFileError",
    "VrsixError",
    "VcfError",
    "VrsixDbError",
]

_logger = logging.getLogger(__name__)


def load_vcf(vcf_path: Path, db_location: Path | None = None) -> None:
    """Load VRS-annotated VCF into sqlite database.

    :param vcf_path: path to VCF (must exist) to ingest
    :param db_location: path to sqlite DB
    """
    sqlite_uri = (
        f"sqlite://{DEFAULT_SQLITE_LOCATION}"
        if db_location is None
        else f"sqlite://{db_location}"
    )
    _logger.debug("Using sqlite file located at %s", sqlite_uri)
    vcf_to_sqlite(vcf_path, sqlite_uri)
