//! SigmaOS-dstat: dstat system stats model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for dstat system stats.
pub struct DstatPlugin {
    pub id: String,
    pub name: String,
    pub plugin_name: String,
    pub enabled: bool,
}

/// Secondary domain entity for dstat system stats.
pub struct DstatMetric {
    pub id: String,
    pub metric_name: String,
    pub description: String,
}

/// Errors for the dstat system stats domain.
#[derive(Debug, PartialEq)]
pub enum DstatError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for DstatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for DstatError {}

/// Registry and manager for dstat system stats.
pub struct DstatManager {
    pub name: String,
    items: BTreeMap<String, DstatPlugin>,
    configs: BTreeMap<String, DstatMetric>,
}

impl DstatManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: DstatPlugin) -> Result<(), DstatError> {
        if item.name.is_empty() {
            return Err(DstatError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(DstatError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&DstatPlugin> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: DstatMetric) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), DstatError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| DstatError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_DstatPlugin(id: &str, name: &str) -> DstatPlugin {
        DstatPlugin {
            id: id.to_string(),
            name: name.to_string(),
            plugin_name: "dstat-default".to_string(),
            enabled: true,
        }
    }

    fn make_DstatMetric(id: &str, val: &str) -> DstatMetric {
        DstatMetric {
            id: id.to_string(),
            metric_name: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = DstatManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = DstatManager::new("test");
        let item = make_DstatPlugin("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = DstatManager::new("test");
        let item = make_DstatPlugin("item1", "");
        assert_eq!(mgr.add_item(item), Err(DstatError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = DstatManager::new("test");
        let item1 = make_DstatPlugin("item1", "First");
        let item2 = make_DstatPlugin("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(DstatError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = DstatManager::new("test");
        let cfg = make_DstatMetric("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = DstatManager::new("test");
        let item = make_DstatPlugin("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = DstatManager::new("test");
        let item = make_DstatPlugin("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(DstatError::NotFound("none".to_string())));
    }
}
