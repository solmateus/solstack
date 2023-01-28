use solstack::prelude::*;

struct GameData {
    popped_dumbstate: bool,
    value: i32,
}

struct DumbState;
impl State<GameData> for DumbState {
    fn on_tick(
        &mut self,
        data: &mut GameData,
    ) -> solstack::trans::Trans<GameData> {
        data.value += 1;
        println!("Tick Popping DumbState {}", data.value);
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
        if data.popped_dumbstate {
            data.value += 1;
            println!("Tick Quitting {}", data.value);
            Trans::Quit
        } else {
            data.value += 1;
            data.popped_dumbstate = true;
            println!("Tick Pushing DumbState {}", data.value);
            Trans::Push(Box::new(DumbState))
        }
    }
}

fn main() {

    let mut data = GameData { value: 0, popped_dumbstate: false };
    let mut stack = Stack::<GameData>::new();

    stack.push(Box::new(GameState), &mut data);
    while stack.is_running() {
        stack.tick(&mut data);
    }

    assert_eq!(data.value, 7);

}
