"""Provide utilities for generating data output in response to user queries."""

import csv
import datetime
from pathlib import Path


def generate_csv(rows: list[tuple], output_location: Path) -> None:
    """Generate CSV at indicated location with provided data.

    :param rows: rows returned from sqlite index
    :param output_location: location to save output to
    """
    if output_location.is_dir():
        output_location = (
            output_location
            / f"vrs_vcf_index_results_{datetime.datetime.now(tz=datetime.timezone.utc)}.csv"
        )
    with output_location.open("w") as f:
        writer = csv.writer(f)
        for row in rows:
            writer.writerow(row)
