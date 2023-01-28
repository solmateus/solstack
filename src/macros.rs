// Stack

/// Alternative to [`stack.push(&mut data, Box::new(FooState {}))`](crate::stack::Stack::push).
/// Use `stack_push!(stack, data, FooState {}, BarState {}, ...)`
/// - Accepts multiple states. In which case it will push all of the provided.
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
#[macro_export]
macro_rules! stack_push {
    ($stack:expr, $data:expr, $($state:expr),+) => {
        $($stack.push(&mut $data, Box::new($state));)+
    }
}

pub use stack_push;

/// Alternative to [`stack.pop(&mut data)`](crate::stack::Stack::pop).
/// Use `stack_pop!(stack, data)`
/// Or `stack_pop!(stack, data, 3)` 3 being the amount of states to pop.
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
/// - Accepts an `amount` of times to pop.
#[macro_export]
macro_rules! stack_pop {
    ($stack:expr, $data:expr) => {
        $stack.pop(&mut $data)
    };

    ($stack:expr, $data:expr, $amount:expr) => {
        for _ in 0..$amount {
            $stack.pop(&mut $data)
        }
    };
}

pub use stack_pop;

/// Alternative to [`stack.replace(&mut data, Box::new(FooState {}))`](crate::stack::Stack::replace).
/// Use `stack_replace!(stack, data, FooState {}, BarState {}, ...)`
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
/// - Accepts multiple `State`s. In which case it will pop once and push the
/// provided states.
#[macro_export]
macro_rules! stack_replace {
    ($stack:expr, $data:expr, $($state:expr),+) => {
        $(
            $stack.pop(&mut data);
            $stack.push(Box::new($state));
        )+
    }
}

pub use stack_replace;

/// Alternative to [`stack.isolate(&mut data, Box::new(FooState {}))`](crate::stack::Stack::isolate).
/// Use `stack_isolate!(stack, data, FooState {}, BarState {}, ...)`
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
/// - Accepts multiple `State`s. In which case it will pop everything from the 
/// stack and then push the provided states.
#[macro_export]
macro_rules! stack_isolate{
    ($stack:expr, $data:expr, $($state:expr),+) => {
        // Pops everything from the stack.
        $stack.quit(&mut data);
        // Adds the provided states for isolation.
        $($stack.push(&mut $data, Box::new($state));)+
    }
}

pub use stack_isolate;

/// Alternative to [`stack.quit(&mut data)`](crate::stack::Stack::quit).
/// Use `stack_quit!(stack, data).`
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
#[macro_export]
macro_rules! stack_quit {
    ($stack:expr, $data:expr) => {
        $stack.quit(&mut $data)
    };
}

pub use stack_quit;

/// Alternative to [`stack.tick(&mut data)`](crate::stack::Stack::tick).
/// Use `stack_tick!(stack, data)`.
/// OBS.: Don't use `&mut data` as a parameter, but simply `data`.
#[macro_export]
macro_rules! stack_tick {
    ($stack:expr, $data:expr) => {
        $stack.tick(&mut $data)
    };
}

pub use stack_tick;

// Transitions

/// Alternative to [`Trans::Push(Box::new(FooState {}))`](crate::trans::Trans::Push).
/// Use `trans_push!(FooState {})`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_push {
    ($state:expr) => {
        solstack::trans::Trans::Push(Box::new($state))
    };
}

pub use trans_push;

/// Alternative to [`Trans::Pop`](crate::trans::Trans::Pop).
/// `Use trans_pop!()`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_pop {
    () => {
        solstack::trans::Trans::Pop
    };
}

pub use trans_pop;

/// Alternative to [`Trans::Replace(Box::new(FooState {}))`](crate::trans::Trans::Replace).
/// Use `trans_replace!(FooState {})`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_replace{
    ($state:expr) => {
        solstack::trans::Trans::Replace(Box::new($state))
    };
}

pub use trans_replace;

/// Alternative to [`Trans::Isolate(Box::new(FooState {}))`](crate::trans::Trans::Isolate).
/// Use `trans_isolate!(FooState {})`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_isolate{
    ($state:expr) => {
        solstack::trans::Trans::Isolate(Box::new($state))
    };
}

pub use trans_isolate;

/// Alternative to [`Trans::None`](crate::trans::Trans::None).
/// Use `trans_none!()`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_none {
    () => {
        solstack::trans::Trans::None
    };
}

pub use trans_none;

/// Alternative to [`Trans::Quit`](crate::trans::Trans::Quit).
/// Use `trans_quit!()`
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! trans_quit {
    () => {
        solstack::trans::Trans::Quit
    };
}

pub use trans_quit;
