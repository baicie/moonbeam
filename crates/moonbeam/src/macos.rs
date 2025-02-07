use crate::{
    blocking::macos::BlockingDeviceImpl, blocking::Brightness as BlockingBrightness, Error,
};
use async_trait::async_trait;
use blocking::unblock;
use futures::stream::{self, once, StreamExt};
use futures::Stream;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct AsyncDeviceImpl(Arc<BlockingDeviceImpl>);

#[async_trait]
impl crate::Brightness for AsyncDeviceImpl {
    async fn device_name(&self) -> Result<String, Error> {
        BlockingBrightness::device_name(&*self.0)
    }

    async fn get(&self) -> Result<u32, Error> {
        let cloned = Arc::clone(&self.0);
        unblock(move || cloned.get()).await
    }

    async fn set(&mut self, percentage: u32) -> Result<(), Error> {
        let cloned = Arc::clone(&self.0);
        unblock(move || cloned.set(percentage)).await
    }
}

pub(crate) fn brightness_devices() -> impl Stream<Item = Result<AsyncDeviceImpl, Error>> {
    once(unblock(crate::blocking::macos::brightness_devices))
        .map(stream::iter)
        .flatten()
        .map(|d| d.map(|d| AsyncDeviceImpl(Arc::new(d))).map_err(Into::into))
}
