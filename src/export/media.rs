//! Media file management

use std::collections::HashMap;

/// Collection of media files
#[derive(Debug, Clone, Default)]
pub struct MediaFiles {
    files: HashMap<String, Vec<u8>>,
}

impl MediaFiles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, data: Vec<u8>) {
        self.files.insert(name, data);
    }

    pub fn get(&self, name: &str) -> Option<&[u8]> {
        self.files.get(name).map(|v| v.as_slice())
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn files(&self) -> &HashMap<String, Vec<u8>> {
        &self.files
    }
}
