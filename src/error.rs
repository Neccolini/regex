#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    SyntaxError(String),
    ParseError(String),
    StateIDOverflow { max: usize },
    InvalidState(String),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn syntax(message: &str) -> Self {
        Self {
            kind: ErrorKind::SyntaxError(message.to_string()),
        }
    }

    pub(crate) fn parse(message: &str) -> Self {
        Self {
            kind: ErrorKind::ParseError(message.to_string()),
        }
    }

    pub(crate) fn state_id_overflow(max: usize) -> Self {
        Self {
            kind: ErrorKind::StateIDOverflow { max },
        }
    }
    pub(crate) fn invalid_state(message: &str) -> Self {
        Self {
            kind: ErrorKind::InvalidState(message.to_string()),
        }
    }
}
