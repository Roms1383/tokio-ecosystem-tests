use bytes::{BufMut, Bytes};
use tokio::io;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

use criterion::async_executor::FuturesExecutor;
use criterion::*;

async fn copy(
    stream: tokio_stream::Iter<std::vec::IntoIter<Result<bytes::Bytes, std::io::Error>>>,
) -> io::Result<Vec<u8>> {
    let mut reader = StreamReader::new(stream);
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut reader, &mut writer).await?;

    Ok(writer)
}

async fn copy_buf(
    stream: tokio_stream::Iter<std::vec::IntoIter<Result<bytes::Bytes, std::io::Error>>>,
) -> io::Result<Vec<u8>> {
    let mut reader = StreamReader::new(stream);
    let mut writer: Vec<u8> = vec![];

    io::copy_buf(&mut reader, &mut writer).await?;

    Ok(writer)
}

async fn iterate(
    mut stream: tokio_stream::Iter<std::vec::IntoIter<Result<bytes::Bytes, std::io::Error>>>,
) -> io::Result<Vec<u8>> {
    let mut buf = Vec::<u8>::new();

    while let Some(output) = stream.next().await {
        buf.put(output?);
    }

    Ok(buf)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("copy_or_loop");
    group.bench_function("copy", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let stream = tokio_stream::iter(vec![
                io::Result::Ok(Bytes::from_static(&[0, 1, 2, 3])),
                io::Result::Ok(Bytes::from_static(&[4, 5, 6, 7])),
                io::Result::Ok(Bytes::from_static(&[8, 9, 10, 11])),
            ]);
            copy(stream).await
        })
    });
    group.bench_function("copy_buf", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let stream = tokio_stream::iter(vec![
                io::Result::Ok(Bytes::from_static(&[0, 1, 2, 3])),
                io::Result::Ok(Bytes::from_static(&[4, 5, 6, 7])),
                io::Result::Ok(Bytes::from_static(&[8, 9, 10, 11])),
            ]);
            copy_buf(stream).await
        })
    });
    group.bench_function("loop", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let stream = tokio_stream::iter(vec![
                io::Result::Ok(Bytes::from_static(&[0, 1, 2, 3])),
                io::Result::Ok(Bytes::from_static(&[4, 5, 6, 7])),
                io::Result::Ok(Bytes::from_static(&[8, 9, 10, 11])),
            ]);
            iterate(stream).await
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
