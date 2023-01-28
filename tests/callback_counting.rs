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
        println!("Tick Popping DummyState {}", data.value);
        Trans::Pop
    }
}

struct GameState;
impl State<GameData> for GameState {
    fn on_start(&mut self, data: &mut GameData) {
        data.value += 1;
        println!("Start {}", data.value);
    }

    fn on_stop(&mut self, data: &mut GameData) {
        data.value += 1;
        println!("Stop {}", data.value);
    }

    fn on_pause(&mut self, data: &mut GameData) {
        data.value += 1;
        println!("Pause {}", data.value);
    }

    fn on_resume(&mut self, data: &mut GameData) {
        data.value += 1;
        println!("Resume {}", data.value);
    }

    fn on_tick(
        &mut self,
        data: &mut GameData,
    ) -> solstack::trans::Trans<GameData> {
        if data.popped_dummystate {
            data.value += 1;
            println!("Tick Quitting {}", data.value);
            Trans::Quit
        } else {
            data.value += 1;
            data.popped_dummystate = true;
            println!("Tick Pushing DumbState {}", data.value);
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
