mod event;
mod frame;
mod future_tick;
mod input;
mod resource;
mod time;
mod state;
mod asset;

pub use event::*;
pub use asset::*;
pub use state::*;
pub use frame::*;
pub use future_tick::*;
pub use input::*;
pub use resource::*;
pub use time::*;
pub mod macros {
    pub use ::macros::*;
}

pub mod prelude {
    pub use super::*;
    pub use ::macros::*;
    pub use futures_lite::FutureExt;
    pub use bevy_core::FrameCount;
    pub use std::time::Duration;
}

use std::future::{poll_fn, PollFn};
use std::task::{Context, Poll};

pub use futures_lite::future::{or, race, try_zip, yield_now, zip};

#[inline]
pub fn when(mut f: impl FnMut() -> bool) -> PollFn<impl FnMut(&mut Context<'_>) -> Poll<()>> {
    poll_fn(move |_| if f() { Poll::Ready(()) } else { Poll::Pending })
}

#[inline]
pub fn when_some<U>(
    mut f: impl FnMut() -> Option<U>,
) -> PollFn<impl FnMut(&mut Context<'_>) -> Poll<U>> {
    poll_fn(move |_| match f() {
        None => Poll::Pending,
        Some(r) => Poll::Ready(r),
    })
}
