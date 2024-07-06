pub trait Action {
    fn execute(&self) -> &str;
}

pub struct RunAction;
impl Action for RunAction {
    fn execute(&self) -> &str {
        "Run"
    }
}

pub struct JumpAction;
impl Action for JumpAction {
    fn execute(&self) -> &str {
        "Jump"
    }
}

pub struct ShootAction;
impl Action for ShootAction {
    fn execute(&self) -> &str {
        "Shoot"
    }
}

pub struct ActionBuffer {
    actions: Vec<Box<dyn Action>>,
}

impl Default for ActionBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBuffer {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }

    pub fn execute(&mut self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        while !self.actions.is_empty() {
            let action = self.actions.remove(0);
            result.push(action.execute().to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_pattern() {
        let mut buffer = ActionBuffer::new();
        buffer.add_action(Box::new(RunAction));
        buffer.add_action(Box::new(JumpAction));
        buffer.add_action(Box::new(ShootAction));

        let actions = buffer.execute();
        assert_eq!(actions.len(), 3);
        assert_eq!(actions[0], "Run");
        assert_eq!(actions[1], "Jump");
        assert_eq!(actions[2], "Shoot");
        let actions = buffer.execute();
        assert_eq!(actions.len(), 0);
    }
}
