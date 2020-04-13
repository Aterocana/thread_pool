use std::fmt;

pub struct PoolCreationError {
    invalid_size: usize,
}

impl PoolCreationError {
    pub fn new(size: usize) -> PoolCreationError {
        PoolCreationError { invalid_size: size }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Pool creation size: {}", self.invalid_size)
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ err: Invalid Pool creation size: {}, file: {}, line: {} }}",
            self.invalid_size,
            file!(),
            line!()
        )
    }
}
