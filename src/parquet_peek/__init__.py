#!/usr/bin/env python
import polars as pl
import rich_click as click
from google.cloud import storage


def extract_bucket(gcs_dir: str):
    if not gcs_dir.startswith("gs://"):
        raise ValueError("GCS path must start with 'gs://'")
    split_fields = gcs_dir.split("/")
    bucket_name = split_fields[2]
    blob_name = "/".join(split_fields[3:])
    return bucket_name, blob_name


def check_file_existence(gcs_file_name: str) -> bool:
    """Checks if a file exists in a GCS bucket."""
    bucket_name, blob_name = extract_bucket(gcs_file_name)
    storage_client = storage.Client()
    bucket = storage_client.bucket(bucket_name)
    blob = bucket.blob(blob_name)
    return blob.exists()


def collect_head(parquet_fn: str, lines_to_show: int):
    df = pl.scan_parquet(parquet_fn)
    return df.head(lines_to_show).collect()


@click.command(help="A tool to sneak peek parquet files")
@click.option(
    "-p", "--parquet-fn", help="Parquet file to peek", type=str, required=True
)
@click.option(
    "-l", "--lines-to-show", help="how many lines to show", type=int, default=10
)
@click.option(
    "-w",
    "--width",
    help="char width to print in each columns of the data frame",
    type=int,
    default=250,
)
def main(parquet_fn, lines_to_show, width):
    if not check_file_existence(parquet_fn):
        raise FileNotFoundError(f"File {parquet_fn} not found in GCS.")
    df = collect_head(parquet_fn=parquet_fn, lines_to_show=lines_to_show)
    with pl.Config(fmt_str_lengths=width, tbl_width_chars=width, tbl_cols=-1):
        print(df)


if __name__ == "__main__":
    main()
