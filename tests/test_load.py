import sqlite3
import tempfile
from collections.abc import Generator
from pathlib import Path

import pytest

from vrsix import load


@pytest.fixture
def temp_dir() -> Generator:
    with tempfile.TemporaryDirectory() as temp_dir:
        yield Path(temp_dir)


@pytest.mark.parametrize("input_filename", ["input.vcf", "input.vcf.gz"])
def test_load(fixture_dir: Path, temp_dir: Path, input_filename: str):
    input_file = fixture_dir / input_filename
    temp_db = temp_dir / "tmp.db"
    load.load_vcf(input_file, temp_db)

    conn = sqlite3.connect(temp_db)
    results = conn.execute("SELECT * FROM vrs_locations").fetchall()
    assert len(results) == 10
    assert results == [
        (1, "dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG", "chr1", 783006, 1),
        (2, "MiasxyXMXtOpsZgGelL3c4QgtflCNLHD", "chr1", 783006, 1),
        (3, "5cY2k53xdW7WeHw2WG1HA7jl50iH-r9p", "chr1", 783175, 1),
        (4, "jHaXepIvlbnapfPtH_62y-Qm81hCrBYn", "chr1", 783175, 1),
        (5, "-NGsjBEx0UbPF3uYjStZ_2r-m2LbUtUB", "chr1", 784860, 1),
        (6, "HLinVo6Q-i-PryQOiq8QAtOeC9oQ9Q3p", "chr1", 784860, 1),
        (7, "qdyeeiC3cLfXeT23zxT9-qlJNN64MKVB", "chr1", 785417, 1),
        (8, "cNWXR3OLq9D3L19vQFvbHw-aH0vlA5cN", "chr1", 785417, 1),
        (9, "DVMcfA37Llc9QUOA0XfLJbJ-agKyGpGo", "chr1", 797392, 1),
        (10, "OTiBHLE2WW93M4-4zGVrWSqP2GBj8-qM", "chr1", 797392, 1),
    ]
    conn.close()


def test_load_specify_uri(fixture_dir: Path, temp_dir: Path):
    input_file = fixture_dir / "input.vcf"
    temp_db = temp_dir / "tmp.db"
    input_uri = "gs://my/input/file.vcf"
    load.load_vcf(input_file, temp_db, input_uri)

    conn = sqlite3.connect(temp_db)
    results = conn.execute("SELECT * FROM vrs_locations").fetchall()
    assert len(results) == 10
    assert results == [
        (1, "dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG", "chr1", 783006, 1),
        (2, "MiasxyXMXtOpsZgGelL3c4QgtflCNLHD", "chr1", 783006, 1),
        (3, "5cY2k53xdW7WeHw2WG1HA7jl50iH-r9p", "chr1", 783175, 1),
        (4, "jHaXepIvlbnapfPtH_62y-Qm81hCrBYn", "chr1", 783175, 1),
        (5, "-NGsjBEx0UbPF3uYjStZ_2r-m2LbUtUB", "chr1", 784860, 1),
        (6, "HLinVo6Q-i-PryQOiq8QAtOeC9oQ9Q3p", "chr1", 784860, 1),
        (7, "qdyeeiC3cLfXeT23zxT9-qlJNN64MKVB", "chr1", 785417, 1),
        (8, "cNWXR3OLq9D3L19vQFvbHw-aH0vlA5cN", "chr1", 785417, 1),
        (9, "DVMcfA37Llc9QUOA0XfLJbJ-agKyGpGo", "chr1", 797392, 1),
        (10, "OTiBHLE2WW93M4-4zGVrWSqP2GBj8-qM", "chr1", 797392, 1),
    ]

    results = conn.execute("SELECT * FROM file_uris").fetchall()
    assert len(results) == 1
    assert results == [(1, input_uri)]

    conn.close()


def test_nonexistent_file(temp_dir: Path):
    input_vcf = Path() / "input.vcf"  # doesn't exist
    temp_db = temp_dir / "tmp.db"
    with pytest.raises(FileNotFoundError):
        load.load_vcf(input_vcf, temp_db)


def test_invalid_sqlite_file(fixture_dir: Path):
    input_vcf = fixture_dir / "input.vcf"
    db = fixture_dir / "not_a_db.db"
    with pytest.raises(load.SqliteFileError):
        load.load_vcf(input_vcf, db)


def test_unsupported_filetype(fixture_dir: Path, temp_dir: Path):
    input_file = fixture_dir / "wrong_file_extension.bam"
    db = temp_dir / "another_db.db"
    with pytest.raises(load.FiletypeError):
        load.load_vcf(input_file, db)


def test_invalid_vcf(fixture_dir: Path, temp_dir: Path):
    input_vcf = fixture_dir / "invalid_vcf.vcf"
    db = temp_dir / "tmp.db"
    with pytest.raises(load.VcfError):
        load.load_vcf(input_vcf, db)


def test_non_block_gzip(fixture_dir: Path, temp_dir: Path):
    temp_db = temp_dir / "tmp.db"
    with pytest.raises(OSError, match="invalid BGZF header"):
        load.load_vcf(fixture_dir / "input_not_bgzip.vcf.gz", temp_db)


def test_load_redundant_rows(fixture_dir: Path, temp_dir: Path):
    input_file = fixture_dir / "input.vcf"
    temp_db = temp_dir / "tmp.db"
    load.load_vcf(input_file, temp_db)
    load.load_vcf(input_file, temp_db)
    conn = sqlite3.connect(temp_db)
    results = conn.execute("SELECT * FROM vrs_locations").fetchall()
    assert len(results) == 10
