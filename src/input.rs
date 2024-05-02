use std::future::Future;
use std::hash::Hash;

use bevy_input::ButtonInput;

use crate::when;

pub trait AsyncXButtonInputExt<T> {
    fn when_pressed(&self, input: T) -> impl Future<Output = ()>;
    fn when_released(&self, input: T) -> impl Future<Output = ()>;
    fn when_clicked(&self, input: T) -> impl Future<Output = ()>;
    fn when_any_pressed(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()>;
    fn when_any_released(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()>;
    fn when_any_clicked(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()>;
}

impl<T> AsyncXButtonInputExt<T> for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    fn when_pressed(&self, input: T) -> impl Future<Output = ()> {
        when(move || self.just_pressed(input))
    }

    fn when_released(&self, input: T) -> impl Future<Output = ()> {
        when(move || self.just_released(input))
    }

    fn when_clicked(&self, input: T) -> impl Future<Output = ()> {
        async move {
            self.when_pressed(input).await;
            self.when_released(input).await;
        }
    }

    fn when_any_pressed(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()> {
        when(move || self.any_just_pressed(inputs.clone()))
    }

    fn when_any_released(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()> {
        when(move || self.any_just_released(inputs.clone()))
    }

    fn when_any_clicked(
        &self,
        inputs: impl IntoIterator<Item = T> + Clone,
    ) -> impl Future<Output = ()> {
        async move {
            self.when_any_pressed(inputs.clone()).await;
            self.when_any_released(inputs.clone()).await;
        }
    }
}
