"""Fetch data from a SQLite index to support tabix-based lookups."""

import sqlite3
from pathlib import Path

from vrsix.sqlite import DEFAULT_SQLITE_LOCATION


def _get_connection(db_location: Path | None) -> sqlite3.Connection:
    if not db_location:
        db_location = DEFAULT_SQLITE_LOCATION
    return sqlite3.connect(db_location)


def fetch_by_vrs_ids(
    vrs_ids: list[str], db_location: Path | None = None
) -> list[tuple]:
    """Access index by VRS ID.

    :param vrs_id: VRS ID or allele hash
    :param db_location: path to sqlite file (assumed to exist)
    :return: location description tuple if available
    """
    vrs_ids = [
        vrs_id[9:] if vrs_id.startswith("ga4gh:VA.") else vrs_id for vrs_id in vrs_ids
    ]
    conn = _get_connection(db_location)
    # have to manually make placeholders for python sqlite API --
    # should still be safe against injection by using parameterized query
    placeholders = ",".join("?" for _ in vrs_ids)
    result = conn.cursor().execute(
        f"SELECT vrs_id, chr, pos FROM vrs_locations WHERE vrs_id IN ({placeholders})",  # noqa: S608
        vrs_ids,
    )
    data = result.fetchall()
    conn.close()
    return [(f"ga4gh:VA.{row[0]}", row[1], row[2]) for row in data]


def fetch_by_pos_range(
    chrom: str, start: int, end: int, db_location: Path | None = None
) -> list[tuple]:
    """Access index by location range.

    :param chrom: chromosome name
    :param start: start of range
    :param end: end of range
    :param db_location: path to sqlite file (assumed to exist)
    """
    conn = _get_connection(db_location)
    result = conn.cursor().execute(
        "SELECT vrs_id, chr, pos FROM vrs_locations WHERE chr = ? AND pos BETWEEN ? AND ?",
        (chrom, start, end),
    )
    data = result.fetchall()
    conn.close()
    return [(f"ga4gh:VA.{row[0]}", row[1], row[2]) for row in data]
