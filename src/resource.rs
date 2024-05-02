use crate::when;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::prelude::{Res, ResMut, Resource};
use std::future::Future;

pub trait AsyncXResExt<T> {
    fn when_changed(&self) -> impl Future<Output = ()>;
    fn when_changed_and_non_added(&self) -> impl Future<Output = ()>;
}

impl<'a, T> AsyncXResExt<T> for Res<'a, T>
where
    T: Resource,
{
    fn when_changed(&self) -> impl Future<Output = ()> {
        when(move || self.is_changed())
    }

    fn when_changed_and_non_added(&self) -> impl Future<Output = ()> {
        when(move || self.is_changed() && !self.is_added())
    }
}

impl<'a, T> AsyncXResExt<T> for ResMut<'a, T>
where
    T: Resource,
{
    fn when_changed(&self) -> impl Future<Output = ()> {
        when(|| self.is_changed())
    }
    fn when_changed_and_non_added(&self) -> impl Future<Output = ()> {
        when(move || self.is_changed() && !self.is_added())
    }
}
