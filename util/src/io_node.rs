
use super::error::Error;

pub struct IoNodeInterface {
    write: fn(&mut IoNode, offset: isize, data: &[u8]) -> Result<usize, Error>,
}

impl IoNodeInterface {
    pub fn new() -> Self {
        IoNodeInterface {
        }
    }
    fn def_write(&mut self, offset: isize, data: &[u8]) ->
        Result<usize, Error> { Err(Error::NoImpl) }
}

struct IoNode {
    ionif: &IoNodeInterface,
}

impl IoNode {
}
