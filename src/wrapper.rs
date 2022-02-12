#[cfg(test)]
mod tests {
    use std::{env::current_dir, time::Duration};

    use tokio::{
        io, join,
        sync::broadcast,
        time::{sleep, Instant},
    };
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

    async fn a_long_task(emitter: broadcast::Sender<u32>) -> io::Result<()> {
        sleep(Duration::from_millis(50)).await;
        let _ = emitter.send(10);
        sleep(Duration::from_millis(100)).await;
        let _ = emitter.send(20);
        sleep(Duration::from_millis(200)).await;
        let _ = emitter.send(50);
        sleep(Duration::from_millis(500)).await;
        let _ = emitter.send(100);
        Ok(())
    }

    async fn some_task(emitter: broadcast::Sender<u32>) -> io::Result<()> {
        sleep(Duration::from_millis(30)).await;
        let _ = emitter.send(101);
        Ok(())
    }

    #[tokio::test]
    async fn broadcast_progress() -> io::Result<()> {
        let (tx, mut rx1) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();
        let tx2 = tx.clone();
        let starts = Instant::now();

        tokio::spawn(async move {
            assert_eq!(rx1.recv().await.unwrap(), 101);
            assert_eq!(rx1.recv().await.unwrap(), 10);
            assert_eq!(rx1.recv().await.unwrap(), 20);
            assert!(starts.elapsed().as_millis() > 100);
            assert_eq!(rx1.recv().await.unwrap(), 50);
            assert!(starts.elapsed().as_millis() > 300);
            assert_eq!(rx1.recv().await.unwrap(), 100);
        });

        tokio::spawn(async move {
            sleep(Duration::from_millis(200)).await;
            assert_eq!(rx2.recv().await.unwrap(), 101);
            assert_eq!(rx2.recv().await.unwrap(), 10);
            assert_eq!(rx2.recv().await.unwrap(), 280);
            assert_eq!(rx2.recv().await.unwrap(), 50);
            assert_eq!(rx2.recv().await.unwrap(), 100);
        });

        let _ = join!(a_long_task(tx), some_task(tx2));

        Ok(())
    }
}
