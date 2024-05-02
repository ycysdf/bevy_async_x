use std::future::Future;

use bevy_ecs::event::EventIterator;
use bevy_ecs::prelude::{Event, EventReader};

use crate::when;

pub trait AsyncXEventExt<E>
where
    E: Event,
{
    fn when_read_any(&mut self) -> impl Future<Output = EventIterator<E>>;
}

impl<E> AsyncXEventExt<E> for EventReader<'_, '_, E>
where
    E: Event,
{
    fn when_read_any(&mut self) -> impl Future<Output = EventIterator<E>> {
        async move {
            when(|| !self.is_empty()).await;
            self.read()
        }
    }
}
