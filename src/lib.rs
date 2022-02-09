//! # tokio-ecosystem-tests
//!
//! set of tests and notes about [tokio](https://tokio.rs/tokio/tutorial).
//!
//!
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
//! see [benchmark](./benches/copy_or_loop/report/index.html)
//!
//! #### examples
//!
//! ```

#![doc = include_str!("copy_or_loop.rs")]

//! ```
//!
//!
//!
//! ## range
//!
//! #### definitions
//!
//! [`stream!`]
//! returns an anonymous type implementing the [`Stream`] trait
//!
//! #### stream transformations
//!
//! create dynamic streams from input, map stream's output, chain different streams
//!
//! #### examples
//!
//! ```

#![doc = include_str!("range.rs")]

//! ```
//!
//! [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [`Bytes`]: https://docs.rs/bytes/latest/bytes/struct.Bytes.html
//! [`AsyncRead`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html
//! [`AsyncWrite`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWrite.html
//! [`Stream`]: https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html
//! [`StreamExt`]: https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html
//! [`futures_util::stream::StreamExt`]: https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html
//! [`copy`]: https://docs.rs/tokio/latest/tokio/io/fn.copy.html
//! [`copy_buf`]: https://docs.rs/tokio/latest/tokio/io/fn.copy_buf.html
//! [`iter`]: https://docs.rs/tokio-stream/latest/tokio_stream/fn.iter.html
//! [`StreamReader`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html
//! [`stream!`]: https://docs.rs/async-stream/latest/async_stream/macro.stream.html

pub mod copy_or_loop;
pub mod range;
