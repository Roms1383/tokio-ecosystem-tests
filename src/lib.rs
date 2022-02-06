//! # tokio-ecosystem-tests
//! 
//! set of tests and notes about [tokio](https://tokio.rs/tokio/tutorial).
//! 
//! ## copy or loop
//! 
//! #### definitions
//! 
//! [`AsyncRead`]
//! represents any source that contains bytes (e.g. `&[u8]`)
//! 
//! [`AsyncWrite`]
//! represents any destination that can hold bytes (e.g. `Vec<u8>`)
//! 
//! [`StreamExt`]
//! provides `stream.next()` method to allow for iteration in a `while` loop
//! 
//! (complementary yet different from [`futures_util::stream::StreamExt`])
//! 
//! #### utility methods & structs
//! 
//! [`copy`]
//! comes in handy to copy bytes from a [`AsyncRead`] to a buffer [`AsyncWrite`]
//! 
//! [`StreamReader`]
//! comes in handy to turn a [`Stream`]::<[`Bytes`]> into a [`AsyncRead`]
//! 
//! [`iter`]
//! provides a convenient way to turn an [`Iterator`] into a [`Stream`]::<[`Bytes`]>
//! 
//! #### manual iteration vs utility method
//! 
//! directly iterating over chunks of bytes is preferable to using [`copy`]
//! which uses an intermediate buffer, but one can look at [`copy_buf`] to achieve the same result
//! 
//! #### examples
//! 
//! ```

#![doc = include_str!("copy_or_loop.rs")]

//! ```
//! 
//! [`Iterator`]: std::iter::Iterator
//! [`Bytes`]: bytes::Bytes
//! [`AsyncRead`]: tokio::io::AsyncRead
//! [`AsyncWrite`]: tokio::io::AsyncWrite
//! [`Stream`]: futures_core::stream::Stream
//! [`StreamExt`]: tokio_stream::StreamExt
//! [`futures_util::stream::StreamExt`]: futures_util::stream::StreamExt
//! [`copy`]: tokio::io::copy
//! [`copy_buf`]: tokio::io::copy_buf
//! [`iter`]: tokio_stream::iter
//! [`StreamReader`]: tokio_util::io::StreamReader

pub mod copy_or_loop;
