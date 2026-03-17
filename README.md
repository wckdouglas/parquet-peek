# parquet-peek

[![PyPI version](https://badge.fury.io/py/parquet-peek.svg)](https://badge.fury.io/py/parquet-peek) [![CI](https://github.com/wckdouglas/parquet-peek/actions/workflows/ci.yaml/badge.svg)](https://github.com/wckdouglas/parquet-peek/actions/workflows/ci.yaml)

just a simple way to check parquet file contents

## Install

### Python

```
pip install parquet-peek
```

### Rust

```
cargo install --path .
```

For cloud storage support (GCS, S3):

```
cargo install --path . --features cloud
```

## Usage

```
parquet-peek -p gs://some-bucket/some-file.parquet
parquet-peek -p /path/to/file.parquet -l 20 -w 100
```

## Dev

### Python

```
rye sync
rye test
```

### Rust

```
cargo build
cargo test
```
