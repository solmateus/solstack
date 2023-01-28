use solstack::prelude::*;

struct GameData {
    popped_dummystate: bool,
    value: i32,
}

struct DummyState;
impl State<GameData> for DummyState {
    fn on_tick(
        &mut self,
        data: &mut GameData,
    ) -> solstack::trans::Trans<GameData> {
        data.value += 1;
        Trans::Pop
    }
}

struct GameState;
impl State<GameData> for GameState {
    fn on_start(&mut self, data: &mut GameData) {
        data.value += 1;
    }

    fn on_stop(&mut self, data: &mut GameData) {
        data.value += 1;
    }

    fn on_pause(&mut self, data: &mut GameData) {
        data.value += 1;
    }

    fn on_resume(&mut self, data: &mut GameData) {
        data.value += 1;
    }

    fn on_tick(
        &mut self,
        data: &mut GameData,
    ) -> solstack::trans::Trans<GameData> {
        if data.popped_dummystate {
            data.value += 1;
            Trans::Quit
        } else {
            data.value += 1;
            data.popped_dummystate = true;
            Trans::Push(Box::new(DummyState))
        }
    }
}

#[test]
fn callback_counting() {

    let mut data = GameData { value: 0, popped_dummystate: false };
    let mut stack = Stack::<GameData>::new();

    stack.push(&mut data, Box::new(GameState));
    while stack.is_running() {
        stack.tick(&mut data);
    }

    assert_eq!(data.value, 7);

}
