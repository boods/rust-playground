use patterns::{
    Action, ActionBuffer, Interpreter, JumpAction, LogFormatter, LogMessage, Milliseconds,
    RedactedFormatter, RunAction, ShootAction, SimpleFormatter,
};

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

fn demo_newtype() {
    let value = Milliseconds::new(1000);
    println!("Newtype Patterm: {}", value);
}

fn demo_strategy() {
    let formatters: Vec<Box<dyn LogFormatter>> = vec![
        Box::new(SimpleFormatter),
        Box::new(RedactedFormatter::new(&vec!["password".to_string()])),
    ];

    let mut message = LogMessage(Vec::new());
    message.0.push(("username".to_string(), "phil".to_string()));
    message
        .0
        .push(("password".to_string(), "password123".to_string()));

    let mut simple_output = String::new();
    formatters[0].format(&message, &mut simple_output);

    let mut redacted_output = String::new();
    formatters[1].format(&message, &mut redacted_output);

    println!(
        "Strategy Pattern: Simple Formatter={} Redacted Formatter={}",
        simple_output, redacted_output
    );
}

fn main() {
    demo_command();
    demo_interpreter();
    demo_newtype();
    demo_strategy();
}
