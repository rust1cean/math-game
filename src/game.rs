use crate::math_task::MathTask;
use std::thread;

const GAME_STATE_RUN: State = State::Run;

#[derive(Debug, PartialEq)]
pub enum State {
    Run,
    Die,
}

pub struct Game {
    state: State,
    tasks: Vec<Task>,
}

impl Game {
    pub fn new() -> Self {
        let state: State = GAME_STATE_RUN;
        let tasks: Vec<Task> = Vec::new();

        Self { state, tasks }.run()
    }

    pub fn run(mut self) -> Self {
        self.state = State::Run;
        self.cycle();
        self
    }

    pub fn stop<'a>(&'a mut self) -> &'a mut Self {
        self.state = State::Die;
        self
    }

    fn cycle<'a>(&'a mut self) -> &'a mut Self {
        while self.state == State::Run {
            self.tasks.iter_mut().for_each(|task| (task.run)());
            self.tasks.clear()
        }
        self
    }
}

pub struct Task {
    run: Box<dyn Fn()>,
}

impl Task {
    pub fn new(run: Box<dyn Fn()>) -> Self {
        Task { run }
    }
}
