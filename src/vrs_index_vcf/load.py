"""Load VCFs into sqlite index."""

from pathlib import Path

from vrs_index_vcf._core import vcf_to_sqlite


def load_vcf(vcf_path: Path, db_location: Path | None = None) -> None:  # noqa: ARG001
    """Load VRS-annotated VCF into sqlite database.

    :param vcf_path: path to VCF (must exist) to ingest
    :param db_location: path to sqlite DB
    """
    # TODO use db_location -- would be good to define default once
    vcf_to_sqlite(vcf_path)
