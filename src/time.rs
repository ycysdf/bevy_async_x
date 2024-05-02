use std::future::Future;
use std::time::Duration;
use bevy_time::Time;
use crate::when;

pub trait AsyncXTimeExt {
   fn when_elapsed(&self, duration: Duration) -> impl Future<Output = ()>;
}

impl AsyncXTimeExt for Time {
   #[inline]
   fn when_elapsed(&self, duration: Duration) -> impl Future<Output = ()> {
      let elapsed = self.elapsed();
      when(move || (self.elapsed() - elapsed) > duration)
   }
}