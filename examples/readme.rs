use bevy::prelude::*;

use bevy_async_x::prelude::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_systems(Main, (test, test2))
        .init_state::<TestState>()
        .run()
}

#[async_system]
async fn test(
    time: Res<Time>,
    key_code: Res<ButtonInput<KeyCode>>,
    frame: Res<FrameCount>,
    state: Res<State<TestState>>,
) {
    println!("wait next state");
    let _state = state.when_next().await;

    let frame_count = frame.0;
    println!("wait next frame");
    frame.when_next().await;
    assert_eq!(frame.0, frame_count + 1);

    println!("wait 5 frame");
    frame.when_nth(5).await;
    assert_eq!(frame.0, frame_count + 6);

    println!("wait 3 seconds or press enter key");
    time.when_elapsed(Duration::from_secs(3))
        .or(key_code.when_pressed(KeyCode::Enter))
        .await;

    loop {
        async {
            key_code.when_clicked(KeyCode::Enter).await;
            println!("Enter Click");
        }
        .or(async {
            key_code.when_clicked(KeyCode::Space).await;
            println!("Space Click");
        })
        .await;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum TestState {
    #[default]
    A,
    B,
    C,
}

#[async_system]
async fn test2(
    key_code: Res<ButtonInput<KeyCode>>,
    frame: Res<FrameCount>,
    state: ResMut<NextState<TestState>>,
) {
    loop {
        let test_state = async {
            key_code.when_pressed(KeyCode::KeyC).await;
            TestState::C
        }
        .or(async {
            key_code.when_pressed(KeyCode::KeyB).await;
            TestState::B
        })
        .or(async {
            key_code.when_pressed(KeyCode::KeyA).await;
            TestState::A
        })
        .await;
        frame.when_next().await;
        println!("set state: {:?}", test_state);
        state.set(test_state);
    }
}
