from unittest.mock import patch

import polars as pl

from parquet_peek import collect_head, extract_bucket

TEST_PARQUET_GCS_PATH = (
    "gs://anaconda-public-data/nyc-taxi/taxi.parquet/part.98.parquet"
)


def test_extract_bucket():
    bucket, blob = extract_bucket(TEST_PARQUET_GCS_PATH)
    assert bucket == "anaconda-public-data"
    assert blob == "nyc-taxi/taxi.parquet/part.98.parquet"


@patch("parquet_peek.pl.scan_parquet")
def test_collect_head(mock_scan_parquet):
    mock_scan_parquet.return_value = pl.DataFrame(
        {"col1": range(100), "col2": range(100, 200)}
    ).lazy()

    df = collect_head(
        TEST_PARQUET_GCS_PATH,
        lines_to_show=5,
    )
    assert df.height == 5
