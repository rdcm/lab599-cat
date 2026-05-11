use super::action::Action;

pub struct EventBus {
    queue: Vec<Action>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn publish(&mut self, action: Action) {
        self.queue.push(action);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Action> + '_ {
        self.queue.drain(..)
    }
}
