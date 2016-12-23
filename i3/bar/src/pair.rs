use std::sync::mpsc::*;

pub struct ChannelPair<T> {
    pub tx: SyncSender<T>,
    pub rx: Receiver<T>
}

impl<T> ChannelPair<T> {
    #[inline]
    pub fn new(buffer_size: usize) -> ChannelPair<T> {
        ChannelPair::from_tuple(sync_channel::<T>(buffer_size))
    }

    #[inline]
    pub fn from_tuple(pair: (SyncSender<T>, Receiver<T>)) -> ChannelPair<T> {
        ChannelPair { tx: pair.0, rx: pair.1 }
    }
}
