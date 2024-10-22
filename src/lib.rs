use std::{collections::HashMap, fmt::Debug, path::PathBuf};

pub enum KvsError {}

impl Debug for KvsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!()
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        unimplemented!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!()
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }
}
