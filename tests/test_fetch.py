from pathlib import Path

import pytest

from gks_anvil.vrsix import fetch


@pytest.fixture()
def db_fixture(fixture_dir: Path) -> Path:
    return fixture_dir / "index.db"


def test_fetch_by_id(db_fixture: Path):
    result = fetch.fetch_by_vrs_ids(
        ["ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J"], db_fixture
    )
    assert result == [("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674)]

    result = fetch.fetch_by_vrs_ids(
        [
            "ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J",
            "nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT",
        ],
        db_fixture,
    )
    result.sort()
    assert result == [
        ("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674),
        ("ga4gh:VA.nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT", "3", 10180),
    ]

    result = fetch.fetch_by_vrs_ids(["sdfljksdfaj;kl"], db_fixture)
    assert result == []


def test_fetch_by_range(db_fixture: Path):
    result = fetch.fetch_by_pos_range("Y", 2781670, 2781675, db_location=db_fixture)
    result.sort()
    assert result == [
        ("ga4gh:VA.6flVGYer2yZRLjSfuJm_tpSAoT_ttTaF", "Y", 2781674),
        ("ga4gh:VA.eXPR_T0angq2prNwkqkRQr800N1mRE7J", "Y", 2781674),
    ]

    result = fetch.fetch_by_pos_range("Y", 2781670, 2781671, db_location=db_fixture)
    assert result == []

    result = fetch.fetch_by_pos_range("3", 10180, 10181, db_location=db_fixture)
    result.sort()
    assert result == [
        ("ga4gh:VA.aPDv4__2RdPqXPinSRQMKNQsaGw2eak7", "3", 10181),
        ("ga4gh:VA.nSND_n_mYcrCnOTxJyWJ3OzRdkwND4rT", "3", 10180),
    ]
