use crate::proto::Packet;

#[derive(Debug)]
pub(crate) struct LoggingFramed<T>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    inner: tokio_util::codec::Framed<T, crate::proto::PacketCodec>,
}

impl<T> LoggingFramed<T>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    pub(crate) fn new(io: T) -> Self {
        LoggingFramed {
            inner: tokio_util::codec::Framed::new(io, Default::default()),
        }
    }
}

impl<T> futures_util::Sink<Packet> for LoggingFramed<T>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    type Error = <crate::proto::PacketCodec as tokio_util::codec::Encoder<Packet>>::Error;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.inner).poll_ready(cx)
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: Packet) -> Result<(), Self::Error> {
        log::trace!(">>> {:?}", item);
        std::pin::Pin::new(&mut self.inner).start_send(item)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.inner).poll_close(cx)
    }
}

impl<T> futures_util::Stream for LoggingFramed<T>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    type Item = Result<
        <crate::proto::PacketCodec as tokio_util::codec::Decoder>::Item,
        <crate::proto::PacketCodec as tokio_util::codec::Decoder>::Error,
    >;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let result = std::pin::Pin::new(&mut self.inner).poll_next(cx);
        if let std::task::Poll::Ready(Some(Ok(item))) = &result {
            log::trace!("<<< {:?}", item);
        }
        result
    }
}
