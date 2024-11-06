import sqlite3
import tempfile
from collections.abc import Generator
from pathlib import Path

import pytest

from vrsix import load


@pytest.fixture()
def temp_dir() -> Generator:
    with tempfile.TemporaryDirectory() as temp_dir:
        yield Path(temp_dir)


def test_load_vcf(fixture_dir: Path, temp_dir: Path):
    input_vcf = fixture_dir / "input.vcf"
    temp_db = temp_dir / "tmp.db"
    load.load_vcf(input_vcf, temp_db)

    conn = sqlite3.connect(temp_db)
    results = conn.execute("SELECT * FROM vrs_locations").fetchall()
    assert len(results) == 10
    assert results == [
        (1, "dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG", "1", 783006),
        (2, "MiasxyXMXtOpsZgGelL3c4QgtflCNLHD", "1", 783006),
        (3, "5cY2k53xdW7WeHw2WG1HA7jl50iH-r9p", "1", 783175),
        (4, "jHaXepIvlbnapfPtH_62y-Qm81hCrBYn", "1", 783175),
        (5, "-NGsjBEx0UbPF3uYjStZ_2r-m2LbUtUB", "1", 784860),
        (6, "HLinVo6Q-i-PryQOiq8QAtOeC9oQ9Q3p", "1", 784860),
        (7, "qdyeeiC3cLfXeT23zxT9-qlJNN64MKVB", "1", 785417),
        (8, "cNWXR3OLq9D3L19vQFvbHw-aH0vlA5cN", "1", 785417),
        (9, "DVMcfA37Llc9QUOA0XfLJbJ-agKyGpGo", "1", 797392),
        (10, "OTiBHLE2WW93M4-4zGVrWSqP2GBj8-qM", "1", 797392),
    ]
    conn.close()


def test_load_gz(fixture_dir: Path, temp_dir: Path):
    input_vcf = fixture_dir / "input.vcf.gz"
    temp_db = temp_dir / "tmp.db"
    load.load_vcf(input_vcf, temp_db)

    conn = sqlite3.connect(temp_db)
    results = conn.execute("SELECT * FROM vrs_locations").fetchall()
    assert len(results) == 10
    assert results == [
        (1, "dwwiZdvVtfAmomu0OBsiHue1O-bw5SpG", "1", 783006),
        (2, "MiasxyXMXtOpsZgGelL3c4QgtflCNLHD", "1", 783006),
        (3, "5cY2k53xdW7WeHw2WG1HA7jl50iH-r9p", "1", 783175),
        (4, "jHaXepIvlbnapfPtH_62y-Qm81hCrBYn", "1", 783175),
        (5, "-NGsjBEx0UbPF3uYjStZ_2r-m2LbUtUB", "1", 784860),
        (6, "HLinVo6Q-i-PryQOiq8QAtOeC9oQ9Q3p", "1", 784860),
        (7, "qdyeeiC3cLfXeT23zxT9-qlJNN64MKVB", "1", 785417),
        (8, "cNWXR3OLq9D3L19vQFvbHw-aH0vlA5cN", "1", 785417),
        (9, "DVMcfA37Llc9QUOA0XfLJbJ-agKyGpGo", "1", 797392),
        (10, "OTiBHLE2WW93M4-4zGVrWSqP2GBj8-qM", "1", 797392),
    ]
    conn.close()

    with pytest.raises(OSError, match="invalid BGZF header"):
        load.load_vcf(fixture_dir / "input_not_bgzip.vcf.gz", temp_db)


def test_error_cases(fixture_dir: Path, temp_dir: Path):
    # test file doesn't exist
    input_vcf = Path() / "input.vcf"
    temp_db = temp_dir / "tmp.db"
    with pytest.raises(FileNotFoundError):
        load.load_vcf(input_vcf, temp_db)

    # test handling of invalid sqlite file
    input_vcf = fixture_dir / "input.vcf"
    db = fixture_dir / "not_a_db.db"
    with pytest.raises(load.SqliteFileError):
        load.load_vcf(input_vcf, db)

    # test unsupported filetype
    input_file = fixture_dir / "wrong_file_extension.bam"
    db = temp_dir / "another_db.db"
    with pytest.raises(load.FiletypeError):
        load.load_vcf(input_file, db)

    # test handling of invalid vcf
    input_vcf = fixture_dir / "invalid_vcf.vcf"
    with pytest.raises(load.VcfError):
        load.load_vcf(input_vcf, db)
