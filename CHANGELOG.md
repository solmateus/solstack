# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Empty

## [0.2.0] - 2023-01-28

## Added

- `Trans`
  - `Replace`: pops and pushes a new state, effectively performing a 
  replacement.
  - `Isolate`: quits and pushes a new state, effectively isolating it as the 
  only one on the stack.
  - Macros and functions on `Stack` for the new transitions.

## [0.1.0] - 2023-01-27

- First public release of `solstack`.

## Added

- Fully capable state stack machine.
- `State` callbacks added on this version:
  - `on_start` for when a state is pushed.
  - `on_stop` for when a state is popped.
  - `on_pause` for when a state has another pushed on top.
  - `on_resume` for when a stated gets resumed due to the one above being pushed.
  - `on_tick` for when the stack gets ticked through its `tick` method.
- `Trans`
  - `Push` that pushes a new state on top.
  - `Pop` that pops the topmost state out.
  - `None` that does not change the stack.
  - `Quit` that pops everything out.
- Examples
  - `guessing_game`
- Tests
  - `callback_counting`
  