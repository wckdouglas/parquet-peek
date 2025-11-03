from parquet_peek import collect_head, extract_bucket

TEST_PARQUET_GCS_PATH = (
    "gs://anaconda-public-data/nyc-taxi/taxi.parquet/part.98.parquet"
)


def test_extract_bucket():
    bucket, blob = extract_bucket(TEST_PARQUET_GCS_PATH)
    assert bucket == "anaconda-public-data"
    assert blob == "nyc-taxi/taxi.parquet/part.98.parquet"


def test_collect_head():
    df = collect_head(
        TEST_PARQUET_GCS_PATH,
        lines_to_show=5,
    )
    assert df.height == 5
