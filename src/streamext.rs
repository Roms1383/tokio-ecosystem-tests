#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;
    use tokio;
    use tokio::io;

    #[allow(non_snake_case)]
    #[tokio::test]
    async fn tokio_doc_trait_StreamExt() -> io::Result<()> {
        let a = tokio_stream::iter(vec![1, 3, 5]);
        let b = tokio_stream::iter(vec![2, 4, 6]);

        let merged = tokio_stream::StreamExt::merge(a, b);
        let first: Vec<_> = merged.collect().await;

        assert_eq!(first, vec![1, 2, 3, 4, 5, 6]);
        Ok(())
    }

    #[allow(non_snake_case)]
    #[tokio::test]
    async fn mixing_both_methods() -> io::Result<()> {
        let a = tokio_stream::iter(vec![1, 3, 5, 7, 9, 11]);
        let b = tokio_stream::iter(vec![2, 4, 6, 8, 10, 12]);

        let output = tokio_stream::StreamExt::merge(a, b);
        let output = output.filter(|x| futures::future::ready(x % 3 == 0));
        let output = tokio_stream::StreamExt::filter(output, |x| x % 2 == 0);
        let output = output.filter(|x| futures::future::ready(x % 4 == 0));

        let output: Vec<_> = output.collect().await;
        assert_eq!(output, vec![12]);
        Ok(())
    }
}
