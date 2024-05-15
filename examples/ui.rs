use bevy::color::palettes::{basic, tailwind};
use bevy::prelude::*;
use bevy_async_x::AsyncXTimeExt;
use futures_lite::future::yield_now;
use futures_lite::FutureExt;
use macros::async_system;
use rxy_ui::bevy::RxyRootEntity;
use rxy_ui::prelude::*;
use std::future::Future;
use std::num::Wrapping;
use std::process::Output;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, update2);

    app.run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct ViewDrawer<'a, 'w, 's, T>
where
    T: ViewKey<BevyRenderer>,
{
    view_key: Option<T>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's, T> Drop for ViewDrawer<'a, 'w, 's, T>
where
    T: ViewKey<BevyRenderer>,
{
    fn drop(&mut self) {
        if let Some(view_key) = self.view_key.take() {
            self.commands.add(|world: &mut World| {
                view_key.remove(world);
            })
        }
    }
}

impl<'a, 'w, 's, T> ViewDrawer<'a, 'w, 's, T>
where
    T: ViewKey<BevyRenderer>,
{
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            view_key: None,
            commands,
        }
    }

    pub async fn draw_on<V>(
        &mut self,
        parent: Option<Entity>,
        view: impl IntoView<BevyRenderer, View = V>,
    ) where
        V: View<BevyRenderer, Key = T>,
    {
        let view: V = view.into_view();
        if self.view_key.is_none() {
            let (sender, receiver) = oneshot::channel();
            self.commands.add(move |world: &mut World| {
                let parent = parent.unwrap_or_else(|| world.resource::<RxyRootEntity>().0);
                let view_key = view.build(ViewCtx { world, parent }, None, true);
                let _ = sender.send(view_key);
            });
            self.view_key = Some(receiver.await.unwrap());
        } else {
            let view_key = self.view_key.clone().unwrap();
            self.commands.add(move |world: &mut World| {
                let parent = parent.unwrap_or_else(|| world.resource::<RxyRootEntity>().0);
                view.rebuild(ViewCtx { world, parent }, view_key);
            });
            yield_now().await;
        }
    }

    pub async fn draw<V>(&mut self, view: impl IntoView<BevyRenderer, View = V>)
    where
        V: View<BevyRenderer, Key = T>,
    {
        self.draw_on(None, view).await
    }
}

pub trait CommandsExt {
    async fn draw<V>(&mut self, view_f: impl FnMut() -> V)
    where
        V: IntoView<BevyRenderer>,
    {
        self.draw_on(None, view_f).await
    }
    async fn draw_on<V>(&mut self, entity: Option<Entity>, view_f: impl FnMut() -> V)
    where
        V: IntoView<BevyRenderer>;

    async fn draw_async<V, O>(&mut self, view_f: impl FnMut() -> O)
    where
        O: Future<Output = V>,
        V: IntoView<BevyRenderer>,
    {
        self.draw_on_async(None, view_f).await
    }
    async fn draw_on_async<V, O>(&mut self, entity: Option<Entity>, view_f: impl FnMut() -> O)
    where
        O: Future<Output = V>,
        V: IntoView<BevyRenderer>;
}

impl CommandsExt for Commands<'_, '_> {
    async fn draw_on<V>(&mut self, entity: Option<Entity>, mut view_f: impl FnMut() -> V)
    where
        V: IntoView<BevyRenderer>,
    {
        let mut drawer = ViewDrawer::new(self);
        async {
            loop {
                drawer.draw_on(entity, view_f()).await;
            }
        }
        .await;
        drop(drawer);
    }

    async fn draw_on_async<V, O>(&mut self, entity: Option<Entity>, mut view_f: impl FnMut() -> O)
    where
        O: Future<Output = V>,
        V: IntoView<BevyRenderer>,
    {
        let mut drawer = ViewDrawer::new(self);
        async {
            loop {
                drawer.draw_on(entity, view_f().await).await;
            }
        }
        .await;
        drop(drawer);
    }
}

#[async_system]
async fn update2(commands: Commands, time: Res<Time>) {
    let time_view = || div().children(format!("Elapsed second: {:?}", time.elapsed().as_secs()));
    let horizontal_axis_len = 300;
    let vertical_axis_len = 300;
    let axis_thickness = 2;
    let axis_color = palettes::BLACK;
    commands
        .draw(|| {
            div().p(25).bg_color(palettes::GRAY).children(
                (div().relative().size(400).children((
                    div()
                        .bottom(0)
                        .left(0)
                        .absolute()
                        .w(horizontal_axis_len)
                        .h(axis_thickness)
                        .bg_color(axis_color),
                    div()
                        .bottom(0)
                        .left(0)
                        .absolute()
                        .h(vertical_axis_len)
                        .w(axis_thickness)
                        .bg_color(axis_color),
                ))),
            )
        })
        // .or(time.when_elapsed(Duration::from_secs(1)))
        .await;
}

#[async_system]
async fn update(commands: Commands, time: Res<Time>) {
    let time_view = || div().children(format!("Elapsed second: {:?}", time.elapsed().as_secs()));
    commands
        .draw(|| {
            (
                div()
                    .flex_col()
                    .p(25)
                    .gap(8)
                    .children(("One", "Two", "Tree")),
                time_view(),
            )
        })
        .or(time.when_elapsed(Duration::from_secs(1)))
        .await;
    let mut b = 0u8;
    let mut reverse = false;
    loop {
        commands
            .draw(|| (div().p(100).bg_color(Srgba::rgb_u8(55, 55, b)), time_view()))
            .or(time.when_elapsed(Duration::from_millis(2)))
            .await;
        if reverse {
            b -= 1;
            if b == 0 {
                reverse = false;
            }
        } else {
            b += 1;
            if b == 255 {
                reverse = true;
            }
        }
    }
}
//
// async fn draw_ui(mut commands: Commands, view: impl IntoView<BevyRenderer>) {
//     if view_key.is_none() {
//         let (sender,receiver) = oneshot::channel();
//         commands.add(move |world: &mut World| {
//             let parent = world.resource::<RxyRootEntity>().0;
//             let view_key = view.build(ViewCtx { world, parent }, None, true);
//             let _ = sender.send(view_key);
//         });
//         view_key = Some(receiver.await.unwrap());
//     } else {
//         let view_key = view_key.clone().unwrap();
//         commands.add(|world: &mut World| {
//             let parent = world.resource::<RxyRootEntity>().0;
//             view.rebuild(ViewCtx { world, parent }, view_key);
//         });
//         yield_now().await;
//     }
// }
