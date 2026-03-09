# parquet-peek

[![CI](https://github.com/wckdouglas/parquet-peek/actions/workflows/ci.yaml/badge.svg)](https://github.com/wckdouglas/parquet-peek/actions/workflows/ci.yaml)

just a simple way to check parquet file contents

## Install

```
cargo install --path .
```

For cloud storage support (GCS, S3):

```
cargo install --path . --features cloud
```

## Usage

```
parquet-peek -p /path/to/file.parquet
parquet-peek -p /path/to/file.parquet -l 20 -w 100
```

With cloud feature enabled:

```
parquet-peek -p gs://some-bucket/some-file.parquet
```

## Dev

```
cargo build
cargo test
```
