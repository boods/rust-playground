use patterns::{Action, ActionBuffer, Interpreter, JumpAction, RunAction, ShootAction};

fn demo_command() {
    let mut buffer = ActionBuffer::new();
    buffer.add_action(Box::new(RunAction));
    buffer.add_action(Box::new(JumpAction));
    buffer.add_action(Box::new(ShootAction));
    println!("Command Pattern: {:?}", buffer.execute());
}

fn demo_interpreter() {
    let mut interpreter = Interpreter::new("Run, Run, Run, Jump, Shoot");
    let mut actions: Vec<Box<dyn Action>> = vec![];
    interpreter.interpret(&mut actions);

    let mut buffer = ActionBuffer::new();
    for action in actions {
        buffer.add_action(action);
    }
    println!("Interpreter Pattern: {:?}", buffer.execute());
}

fn main() {
    demo_command();
    demo_interpreter();
}
