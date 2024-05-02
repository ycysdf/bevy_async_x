use std::future::Future;

use bevy_ecs::schedule::{State, States};
use bevy_ecs::system::Res;

use crate::AsyncXResExt;

pub trait AsyncXStateExt<S> {
    fn when_next<'a>(&'a self) -> impl Future<Output = &'a S>
    where
        S: 'a;
}

impl<S> AsyncXStateExt<S> for Res<'_, State<S>>
where
    S: States,
{
    fn when_next<'a>(&'a self) -> impl Future<Output = &'a S>
    where
        S: 'a,
    {
        async move {
            self.when_changed_and_non_added().await;
            self.get()
        }
    }
}
