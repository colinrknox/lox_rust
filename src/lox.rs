#[derive(Clone)]
pub struct Error {
    line: usize,
    place: String,
    message: String,
}

impl Error {
    pub fn new(line: usize, place: String, message: String) -> Error {
        Error {
            line,
            place,
            message,
        }
    }
}

#[derive(Clone)]
pub struct Lox {
    pub had_error: bool,
    pub errors: Vec<Error>,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message)
    }

    fn report(&mut self, line: usize, place: String, message: String) {
        self.had_error = true;
        let error = Error::new(line, place, message);
        eprintln!(
            "[line {}] Error {}: {}",
            error.line, error.place, error.message
        );
        self.errors.push(error);
    }
}
