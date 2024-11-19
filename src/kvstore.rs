use crate::Result;
use std::path::PathBuf;

pub struct KvStore {}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        panic!()
    }

    pub fn get(&self, _key: String) -> Result<Option<String>> {
        panic!()
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        panic!()
    }

    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!()
    }
}
