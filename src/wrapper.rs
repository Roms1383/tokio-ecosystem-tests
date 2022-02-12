#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use tokio::io;
    use tokio_stream::{wrappers::ReadDirStream, StreamExt};

    #[tokio::test]
    async fn read_dir() -> io::Result<()> {
        let dir = tokio::fs::read_dir(current_dir().unwrap()).await?;
        let mut stream = ReadDirStream::new(dir);

        let mut count = 0;
        while let Some(Ok(entry)) = stream.next().await {
            if entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .starts_with("cargo")
            {
                count += 1;
            }
        }
        assert_eq!(count, 2);

        Ok(())
    }

    #[tokio::test]
    async fn chained() -> io::Result<()> {
        let dir = tokio::fs::read_dir(current_dir().unwrap()).await?;
        let count = ReadDirStream::new(dir)
            .filter(|v| v.is_ok())
            .filter(|v| {
                v.as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .starts_with("cargo")
            })
            .collect::<Vec<_>>()
            .await
            .len();
        assert_eq!(count, 2);

        Ok(())
    }

    #[tokio::test]
    async fn oneliner() -> io::Result<()> {
        let count = futures::stream::StreamExt::count(
            ReadDirStream::new(tokio::fs::read_dir(current_dir().unwrap()).await?)
                .filter(|v| v.is_ok())
                .filter(|v| {
                    v.as_ref()
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .to_lowercase()
                        .starts_with("cargo")
                }),
        )
        .await;
        assert_eq!(count, 2);

        Ok(())
    }
}
