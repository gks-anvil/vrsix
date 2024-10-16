from pathlib import Path

import pytest

from vrsix import fetch


@pytest.fixture(scope="session")
def fixture_db() -> Path:
    return Path(__file__).parents[0].resolve() / "fixtures" / "index.db"


def test_fetch_by_id(fixture_db: Path):
    result = fetch.fetch_by_vrs_ids(
        ["ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J"], fixture_db
    )
    assert result == [("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674)]

    result = fetch.fetch_by_vrs_ids(
        [
            "ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J",
            "nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT",
        ],
        fixture_db,
    )
    result.sort()
    assert result == [
        ("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674),
        ("ga4gh:VA.nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT", "3", 10180),
    ]

    result = fetch.fetch_by_vrs_ids(
        [
            "ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J",
            "nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT",
        ],
        fixture_db,
    )
    result.sort()
    assert result == [
        ("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674),
        ("ga4gh:VA.nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT", "3", 10180),
    ]

    result = fetch.fetch_by_vrs_ids(["sdfljksdfajkl"])
    assert result == []


def test_fetch_by_range(fixture_db: Path):
    result = fetch.fetch_by_pos_range("Y", 2781670, 2781675, db_location=fixture_db)
    result.sort()
    assert result == [
        ("ga4gh:VA.6flVGYer2yZRLjSfuJm_tpSAoT_ttTaF", "Y", 2781674),
        ("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674),
    ]

    result = fetch.fetch_by_pos_range("Y", 2781670, 2781671, db_location=fixture_db)
    assert result == []

    result = fetch.fetch_by_pos_range("3", 10180, 10181, db_location=fixture_db)
    result.sort()
    assert result == [
        ("ga4gh:VA.aPDv4__2RdPqXPinSRQMKNQsaGw2eak7", "3", 10181),
        ("ga4gh:VA.nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT", "3", 10180),
    ]
