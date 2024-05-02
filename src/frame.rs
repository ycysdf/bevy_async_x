use std::future::Future;
use bevy_core::FrameCount;
use crate::when;

pub trait AsyncXFrameExt {
   fn when_next(&self) -> impl Future<Output = ()>;
   fn when_nth(&self, count: u32) -> impl Future<Output = ()>;
}

impl AsyncXFrameExt for FrameCount {
   #[inline]
   fn when_next(&self) -> impl Future<Output = ()> {
      self.when_nth(1)
   }

   fn when_nth(&self, count: u32) -> impl Future<Output = ()> {
      let current = self.0;
      async move { when(move || self.0 == (current + count)).await }
   }
}