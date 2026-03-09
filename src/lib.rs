use anyhow::{bail, Result};
use polars::prelude::*;

/// Parse a GCS path into bucket name and blob name.
///
/// # Example
/// ```
/// use parquet_peek::extract_bucket;
/// let (bucket, blob) = extract_bucket("gs://my-bucket/path/to/file.parquet").unwrap();
/// assert_eq!(bucket, "my-bucket");
/// assert_eq!(blob, "path/to/file.parquet");
/// ```
pub fn extract_bucket(gcs_dir: &str) -> Result<(String, String)> {
    if !gcs_dir.starts_with("gs://") {
        bail!("GCS path must start with 'gs://'");
    }
    let parts: Vec<&str> = gcs_dir.split('/').collect();
    let bucket_name = parts[2].to_string();
    let blob_name = parts[3..].join("/");
    Ok((bucket_name, blob_name))
}

/// Check if a file exists in a GCS bucket.
///
/// Requires the `cloud` feature.
#[cfg(feature = "cloud")]
pub async fn check_file_existence(gcs_file_name: &str) -> Result<bool> {
    use object_store::gcp::GoogleCloudStorageBuilder;
    use object_store::path::Path as ObjectPath;
    use object_store::ObjectStore;

    let (bucket_name, blob_name) = extract_bucket(gcs_file_name)?;
    let store = GoogleCloudStorageBuilder::new()
        .with_bucket_name(&bucket_name)
        .build()?;
    let path = ObjectPath::from(blob_name);
    match store.head(&path).await {
        Ok(_) => Ok(true),
        Err(object_store::Error::NotFound { .. }) => Ok(false),
        Err(e) => Err(e.into()),
    }
}

/// Read the first N rows from a parquet file.
///
/// Supports local paths. Enable the `cloud` feature for gs:// and s3:// paths.
pub fn collect_head(parquet_fn: &str, lines_to_show: u32) -> Result<DataFrame> {
    let df = LazyFrame::scan_parquet(parquet_fn.into(), ScanArgsParquet::default())?
        .limit(lines_to_show)
        .collect()?;
    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GCS_PATH: &str = "gs://anaconda-public-data/nyc-taxi/taxi.parquet/part.98.parquet";

    #[test]
    fn test_extract_bucket() {
        let (bucket, blob) = extract_bucket(TEST_GCS_PATH).unwrap();
        assert_eq!(bucket, "anaconda-public-data");
        assert_eq!(blob, "nyc-taxi/taxi.parquet/part.98.parquet");
    }

    #[test]
    fn test_extract_bucket_invalid_path() {
        assert!(extract_bucket("s3://bucket/path").is_err());
    }

    #[test]
    fn test_extract_bucket_no_blob() {
        let (bucket, blob) = extract_bucket("gs://bucket").unwrap();
        assert_eq!(bucket, "bucket");
        assert_eq!(blob, "");
    }

    #[test]
    fn test_collect_head_nonexistent_file() {
        assert!(collect_head("/nonexistent/path.parquet", 5).is_err());
    }

    #[test]
    fn test_collect_head() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.parquet");

        // Create a test parquet file
        let mut df = df!(
            "col1" => (0i32..100).collect::<Vec<_>>(),
            "col2" => (100i32..200).collect::<Vec<_>>()
        )
        .unwrap();

        let mut file = std::fs::File::create(&path).unwrap();
        ParquetWriter::new(&mut file).finish(&mut df).unwrap();

        let result = collect_head(path.to_str().unwrap(), 5).unwrap();
        assert_eq!(result.height(), 5);
    }
}
