use bevy::prelude::*;
use bevy_async_x::FutureState;
use macros::async_system;
use rxy_ui::bevy::RxyPlugin;
use rxy_ui::prelude::RxyStyleSheetPlugin;
use std::ops::DerefMut;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
    ))
    // .add_systems(Startup, startup)
    .add_systems(Update, state_machine);

    app.run();
}

#[derive(Component, Clone, Copy)]
enum PeopleState {
    Idle,
    Walk,
    JumpStart,
    Jumping,
    JumpEnd,
    Run,
    Climb,
    Swim,
}
enum NextState{
    Default,
    Some(PeopleState),
}
#[async_system]
async fn state_machine(query: Query<(Entity, &mut PeopleState, &mut FutureState)>) {
    loop {
        for (entity, state, mut future_state) in query.iter_mut() {
            bevy_async_x::tick_future(
                (entity, state),
                future_state.deref_mut(),
                |(mut entity,mut  state)| async {
                    let mut next_state = NextState::Default;
                    match *state {
                        PeopleState::Idle => {}
                        PeopleState::Walk => {}
                        PeopleState::Run => {}
                        PeopleState::Climb => {}
                        PeopleState::Swim => {}
                        PeopleState::JumpStart => {
                            *next_state = NextState::Some(PeopleState::Jumping);
                        }
                        PeopleState::Jumping => {
                            // wait ground
                        }
                        PeopleState::JumpEnd => {
                        }
                    }
                },
            );
            // future_state
        }
    }
}
