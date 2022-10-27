const APP_STATE_RUN: State = State::Run;

type Task = dyn Fn(&mut App);

#[derive(PartialEq)]
pub enum State {
    Run,
    Stop,
}

pub struct App {
    state: State,
    tasks: Vec<Box<Task>>,
}

impl App {
    pub fn new() -> Self {
        let state: State = APP_STATE_RUN;
        let tasks: Vec<Box<Task>> = Vec::new();

        Self { state, tasks }
    }

    pub fn run<'a>(&'a mut self) -> &'a mut Self {
        self.state = State::Run;
        self.cycle();
        self
    }

    pub fn stop<'a>(&'a mut self) -> &'a mut Self {
        self.state = State::Stop;
        self
    }

    fn cycle<'a>(&'a mut self) -> &'a mut Self {
        while self.state == State::Run {
            self.complete_tasks();
        }
        self
    }

    fn complete_tasks<'a>(&'a mut self) -> &'a mut Self {
        if self.tasks.len() > 0 {
            let mut tasks: Vec<Box<Task>> = self.tasks.drain(..).collect();
            tasks.retain(|task| {
                task(self);
                false
            });
        }
        self
    }

    pub fn add_task<'a>(&'a mut self, task: &'static Task) -> &'a mut Self {
        let task: Box<Task> = Box::new(task);
        self.tasks.push(task);
        self
    }
}
