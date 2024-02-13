pub struct LogMessage(pub Vec<(String, String)>);

pub trait LogFormatter {
    fn format(&self, message: &LogMessage, output: &mut String);
}

pub struct SimpleFormatter;
impl LogFormatter for SimpleFormatter {
    fn format(&self, message: &LogMessage, output: &mut String) {
        for message in message.0.iter() {
            output.push_str(format!("{}={},", message.0, message.1).as_str());
        }
    }
}

pub struct RedactedFormatter {
    redacted_fields: Vec<String>,
}

impl RedactedFormatter {
    pub fn new(fields: &Vec<String>) -> RedactedFormatter {
        RedactedFormatter {
            redacted_fields: fields.to_owned(),
        }
    }
    pub fn add_redacted_field(&mut self, key: &str) {
        self.redacted_fields.push(key.to_owned());
    }
}

impl LogFormatter for RedactedFormatter {
    fn format(&self, message: &LogMessage, output: &mut String) {
        for message in message.0.iter() {
            let mut value = message.1.clone();
            if self.redacted_fields.contains(&message.0) {
                value = "*****".to_string();
            }
            output.push_str(format!("{}={},", message.0, value).as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_pattern() {
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
        assert_eq!(simple_output, "username=phil,password=password123,");

        let mut redacted_output = String::new();
        formatters[1].format(&message, &mut redacted_output);
        assert_eq!(redacted_output, "username=phil,password=*****,");
    }
}
