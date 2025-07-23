use tokio::fs::create_dir_all;
use tokio::io::AsyncWriteExt;

use crate::errors::Error;
use std::path::Path;

pub async fn async_write_safely<P, B>(path: P, bytes: &B) -> Result<(), Error>
where
    P: AsRef<Path>,
    B: AsRef<[u8]>,
{
    let path = path.as_ref();

    // FIXME: I think a path + file is not considered a file?

    // if !path.is_file() {
    //     return Err(format!("Expected file path, found {}", path.to_string_lossy()).into());
    // }

    if let Some(dir) = path.parent() {
        create_dir_all(&dir).await?;
    }

    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(&bytes.as_ref()).await?;

    Ok(())
}
