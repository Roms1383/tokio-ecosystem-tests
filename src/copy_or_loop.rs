//! illustrate correlated usages of [tokio::io::copy] and [tokio_util::io::StreamReader]
//! 
//! ### definitions
//! 
//! [tokio::io::AsyncRead]
//! represents any source that contains bytes (e.g. `&[u8]`)
//! 
//! [tokio::io::AsyncWrite]
//! represents any destination that can hold bytes (e.g. `Vec<u8>`)
//! 
//! [tokio_stream::StreamExt]
//! provides `stream.next()` method to allow for iteration in a `while` loop
//! 
//! (complementary yet different from [futures_util::stream::StreamExt](https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html))
//! 
//! ### utility methods & structs
//! 
//! [tokio::io::copy]
//! comes in handy to copy bytes from a [tokio::io::AsyncRead] to a buffer [tokio::io::AsyncWrite]
//! 
//! [tokio_util::io::StreamReader]
//! comes in handy to turn a [futures_core::stream::Stream] into a [tokio::io::AsyncRead]
//! 
//! [tokio_stream::iter]
//! provides a convenient way to turn an [Iterator] into a [tokio_stream::Stream]
//! 
//! ### manual iteration vs utility method
//! 
//! directly iterating over chunks of bytes is preferable to using [tokio::io::copy]
//! which uses an intermediate buffer, but one can look at [tokio::io::copy_buf] to achieve the same result

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
