use std::fmt::Display;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum KvsError {}

impl Display for KvsError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
