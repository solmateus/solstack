use crate::trans::Trans;

/// Describes an application state.
///
/// See the crates docs or the book for more information.
/// ### Generics
/// - D: Data available to all [`State`](crate::state::State)s to perform actions to.
pub trait State<D> {

    /// Called when this [`State`](crate::state::State) first enters the stack.
    fn on_start(&mut self, _data: &mut D) {}

    /// Called when this [`State`](crate::state::State) is popped from the stack.
    fn on_stop(&mut self, _data: &mut D) {}

    /// Called when this [`State`](crate::state::State) loses its topmost position in the stack to
    /// another [`State`](crate::state::State) that has been pushed ontop of it.
    fn on_pause(&mut self, _data: &mut D) {}

    /// Called when this [`State`](crate::state::State) regains the first position in the stack.
    /// Id est, when other [`State`](crate::state::State)s above it are popped.
    fn on_resume(&mut self, _data: &mut D) {}

    /// Represents a single tick/update.
    /// It's called when the [`Stack`](crate::stack::Stack)'s `.tick()` is called.
    /// Your loop logic should call your stack's tick, not this directly.
    fn on_tick(&mut self, _data: &mut D) -> Trans<D> { 

        Trans::None
    }
}

