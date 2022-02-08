//! illustrate usage of [`stream!`]
//!
//! [`stream!`]: https://docs.rs/async-stream/latest/async_stream/macro.stream.html

use std::ops::RangeInclusive;

use async_stream::stream;
use tokio::io;
use tokio_stream::Stream;

pub fn from_range(range: RangeInclusive<u32>) -> impl Stream<Item = io::Result<u32>> {
    stream! {
        for i in range {
            yield io::Result::Ok(i); // notice here that it yields a tokio::io::Result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::from_range;
    use async_stream::stream;
    use futures::pin_mut;
    use tokio::io;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn tokio_doc_macro_stream() -> io::Result<()> {
        let first = stream! {
            for i in 0..=3 {
                yield i; // notice here that it yields the number directly
            }
        };
        let second = from_range(4..=7);
        let third = from_range(8..=11);

        pin_mut!(first);
        pin_mut!(second);
        pin_mut!(third);

        let mut stream = first
            // map first digits into tokio::io::Result
            .map(io::Result::Ok)
            // subsequent streams already yield tokio::io::Result
            .chain(second)
            .chain(third);

        let mut buf = Vec::<u32>::new();
        while let Some(io::Result::Ok(item)) = stream.next().await {
            buf.push(item);
        }

        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], &buf[..]);
        Ok(())
    }
}
