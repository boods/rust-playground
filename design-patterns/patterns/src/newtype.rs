use std::fmt::Display;

pub struct Milliseconds(u32);

impl Milliseconds {
    pub fn new(value: u32) -> Milliseconds {
        Milliseconds(value)
    }
}

impl Display for Milliseconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_pattern() {
        let value = Milliseconds(1000);
        assert_eq!(value.to_string(), "1000 ms");
    }
}
