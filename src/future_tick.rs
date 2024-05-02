use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use bevy_ecs::prelude::Local;

#[derive(Default)]
pub enum FutureState {
    #[default]
    NoStart,
    Started {
        inner: FutureInnerData,
        future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    },
    Completed,
}

pub struct FutureInnerData(Option<*mut u8>);

impl FutureInnerData {
    pub fn scoped<T, U>(&mut self, data: T, f: impl FnOnce() -> U) -> U {
        let inner_data = self.0.unwrap();
        unsafe {
            let inner_data = inner_data as *mut T;
            *inner_data = data;
        }
        let result = f();
        result
    }
}

unsafe impl Send for FutureInnerData {}
const NOOP: RawWaker = {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        // Cloning just returns a new no-op raw waker
        |_| NOOP,
        // `wake` does nothing
        |_| {},
        // `wake_by_ref` does nothing
        |_| {},
        // Dropping does nothing as we don't allocate anything
        |_| {},
    );
    RawWaker::new(ptr::null(), &VTABLE)
};

pub fn tick_future<'a,T, Fur>(
    params: T,
    mut future_state: Local<FutureState>,
    future_factory: impl FnOnce(&'a mut T) -> Fur + Send,
) where
    T: Send +'a,
    Fur: Future<Output = ()> + Send,
{
    let weaker: Waker = unsafe { Waker::from_raw(NOOP) };
    let next_state = match future_state.deref_mut() {
        FutureState::NoStart => {
            let mut inner_ptr = FutureInnerData(None);
            let future = Box::pin({
                let inner_ptr = &mut inner_ptr;
                async move {
                    let mut inner_data = vec![0u8;core::mem::size_of::<T>()];
                    inner_ptr.0 = Some(inner_data.as_mut_ptr());
                    let data = unsafe { &mut *(inner_data.as_mut_ptr() as *mut T) };
                    *data = params;
                    future_factory(data).await;
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>;
            let mut future: Pin<Box<dyn Future<Output = ()> + Send + 'static>> =
                unsafe { core::mem::transmute(future) };

            let mut ctx = Context::from_waker(&weaker);
            match Pin::new(&mut future).poll(&mut ctx) {
                Poll::Ready(_) => Some(FutureState::Completed),
                Poll::Pending => Some(FutureState::Started {
                    inner: inner_ptr,
                    future,
                }),
            }
        }
        FutureState::Started {
            ref mut future,
            inner: inner_ptr,
        } => inner_ptr.scoped(params, || {
            let mut ctx = Context::from_waker(&weaker);
            match Pin::new(future).poll(&mut ctx) {
                Poll::Ready(_) => Some(FutureState::Completed),
                Poll::Pending => None,
            }
        }),
        FutureState::Completed => None,
    };
    if let Some(next_state) = next_state {
        *future_state = next_state;
    }
}
