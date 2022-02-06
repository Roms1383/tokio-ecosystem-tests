//! illustrate correlated usages of [`copy`] and [`StreamReader`]
//! 
//! [`copy`]: https://docs.rs/tokio/latest/tokio/io/fn.copy.html
//! [`StreamReader`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BufMut};
    use tokio;
    use tokio::io;
    use tokio_stream::StreamExt;
    use tokio_util::io::StreamReader;

    #[tokio::test]
    async fn tokio_doc_fn_copy() -> io::Result<()> {
        let mut reader: &[u8] = b"hello";
        let mut writer: Vec<u8> = vec![];

        io::copy(&mut reader, &mut writer).await?;

        assert_eq!(&b"hello"[..], &writer[..]);
        Ok(())
    }

    #[allow(non_snake_case)]
    #[tokio::test]
    async fn struct_StreamReader() -> io::Result<()> {
        let stream = tokio_stream::iter(vec![
            io::Result::Ok(Bytes::from_static(&[0, 1, 2, 3])),
            io::Result::Ok(Bytes::from_static(&[4, 5, 6, 7])),
            io::Result::Ok(Bytes::from_static(&[8, 9, 10, 11])),
        ]);

        let mut reader = StreamReader::new(stream);
        let mut writer: Vec<u8> = vec![];

        // note that the reader is consumed after copy (and left empty)
        io::copy(&mut reader, &mut writer).await?;

        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], &writer[..]);
        Ok(())
    }

    #[tokio::test]
    async fn loop_over_chunks() -> io::Result<()> {
        let mut stream = tokio_stream::iter(vec![
            io::Result::Ok(Bytes::from_static(&[0, 1, 2, 3])),
            io::Result::Ok(Bytes::from_static(&[4, 5, 6, 7])),
            io::Result::Ok(Bytes::from_static(&[8, 9, 10, 11])),
        ]);

        let mut buf = Vec::<u8>::new();
        while let Some(io::Result::Ok(bytes)) = stream.next().await {
            buf.put(bytes);
        }

        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], &buf[..]);
        Ok(())
    }
}
