// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-tool-apis: tool apis component model.
//!
//! Real implementation: apiss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apis {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolApisError {
    EmptyApisName,
    DuplicateApis(String),
    EmptyEventSubject,
}

impl fmt::Display for ToolApisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyApisName => write!(f, "apis name must not be empty"),
            Self::DuplicateApis(id) => write!(f, "duplicate apis {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ToolApisError {}

pub struct ToolApis {
    pub name: String,
    apiss: BTreeMap<String, Apis>,
    events: BTreeMap<String, Event>,
}

impl ToolApis {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            apiss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_apis(&mut self, item: Apis) -> Result<(), ToolApisError> {
        if item.name.trim().is_empty() {
            return Err(ToolApisError::EmptyApisName);
        }
        let id = item.id.clone();
        if self.apiss.contains_key(&id) {
            return Err(ToolApisError::DuplicateApis(id.clone()));
        }
        self.apiss.insert(id, item);
        Ok(())
    }

    pub fn apis_count(&self) -> usize {
        self.apiss.len()
    }

    pub fn get_apis(&self, id: &str) -> Option<&Apis> {
        self.apiss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ToolApisError> {
        if event.subject.trim().is_empty() {
            return Err(ToolApisError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_apis(id: &str, name: &str) -> Apis {
        Apis {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ToolApis::new("ToolApis");
        assert_eq!(m.name, "ToolApis");
        assert_eq!(m.apis_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_apis() {
        let mut m = ToolApis::new("ToolApis");
        m.add_apis(sample_apis("a1", "Alpha")).unwrap();
        assert_eq!(m.apis_count(), 1);
        assert!(m.get_apis("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ToolApis::new("ToolApis");
        assert!(m.add_apis(sample_apis("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ToolApis::new("ToolApis");
        m.add_apis(sample_apis("x", "One")).unwrap();
        assert!(m.add_apis(sample_apis("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ToolApis::new("ToolApis");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
