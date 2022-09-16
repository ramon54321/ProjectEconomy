use std::collections::HashMap;

pub struct Store {
    items: HashMap<String, isize>,
}
impl Store {
    pub(super) fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    ///
    /// Returns true if the store contains at least 1 of item.
    ///
    pub(super) fn has(&self, item: &str) -> bool {
        if self.count(item) == 0 {
            return false;
        }
        return true;
    }
    ///
    /// Counts the number of items of type 'item' in the store. If no items have been added to the
    /// store, 0 is returned.
    ///
    pub(super) fn count(&self, item: &str) -> isize {
        if !self.items.contains_key(item) {
            return 0;
        }
        *self.items.get(item).unwrap()
    }
    ///
    /// Adds the specific count of item to the store.
    ///
    pub(super) fn add(&mut self, item: &str, count: isize) {
        if !self.has(item) {
            self.items.insert(item.to_string(), count);
            return;
        }
        *self.items.get_mut(item).unwrap() += count;
    }
    ///
    /// Sets the specific count of item to the store.
    ///
    pub(super) fn set(&mut self, item: &str, count: isize) {
        self.items.insert(item.to_string(), count);
    }
    ///
    /// Takes items from the store. Returns the number of items which was taken.
    ///
    pub(super) fn take(&mut self, item: &str, count: isize) -> isize {
        let store_count = self.count(item);

        // Check if store does not have any of the item.
        if store_count == 0 {
            return 0;
        }

        // Take entire count if possible
        if store_count >= count {
            *self.items.get_mut(item).unwrap() -= count;
            return count;
        }

        // Else take only what store has to offer
        *self.items.get_mut(item).unwrap() -= store_count;
        store_count
    }
    ///
    /// Clear all items from the store.
    ///
    pub(super) fn clear(&mut self) {
        self.items.clear();
    }
    ///
    /// Get a list of all item kinds in the store.
    ///
    pub(super) fn get_item_kinds(&self) -> Vec<&String> {
        self.items.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let mut store = Store::new();
        store.add("Apple", 3);
        assert_eq!(store.count("Apple"), 3);
    }

    #[test]
    fn has() {
        let mut store = Store::new();
        store.add("Apple", 3);
        store.add("Orange", 1);
        store.add("Banana", 0);
        assert!(store.has("Apple"));
        assert!(store.has("Orange"));
        assert!(!store.has("Banana"));
        assert!(!store.has("Steak"));
    }

    #[test]
    fn add() {
        let mut store = Store::new();
        assert!(!store.has("Apple"));
        assert!(!store.has("Orange"));
        assert!(!store.has("Banana"));

        store.add("Apple", 3);
        store.add("Orange", 1);
        store.add("Banana", 0);

        assert!(store.has("Apple"));
        assert!(store.has("Orange"));
        assert!(!store.has("Banana"));
        assert!(!store.has("Steak"));
    }

    #[test]
    fn set() {
        let mut store = Store::new();
        assert!(!store.has("Apple"));
        assert!(!store.has("Orange"));
        assert!(!store.has("Banana"));

        store.add("Apple", 3);
        store.add("Orange", 1);
        store.add("Banana", 1);
        store.set("Apple", 0);
        store.set("Banana", 5);

        assert!(!store.has("Apple"));
        assert!(store.has("Orange"));
        assert!(store.has("Banana"));
        assert!(!store.has("Steak"));
    }

    #[test]
    fn take() {
        let mut store = Store::new();
        store.add("Apple", 3);
        store.add("Orange", 1);

        let taken_apples = store.take("Apple", 2);
        assert!(store.has("Apple"));
        assert_eq!(taken_apples, 2);
        assert_eq!(store.count("Apple"), 1);

        let taken_oranges = store.take("Orange", 3);
        assert!(!store.has("Orange"));
        assert_eq!(taken_oranges, 1);
        assert_eq!(store.count("Orange"), 0);

        let taken_grapes = store.take("Grape", 5);
        assert_eq!(taken_grapes, 0);
        assert!(!store.has("Grape"));
    }

    #[test]
    fn clear() {
        let mut store = Store::new();
        store.add("Apple", 3);
        store.add("Orange", 1);
        store.add("Banana", 0);
        assert!(store.has("Apple"));
        assert!(store.has("Orange"));
        store.clear();
        assert!(!store.has("Apple"));
        assert!(!store.has("Orange"));
    }

    #[test]
    fn get_item_kinds() {
        let mut store = Store::new();
        store.add("Apple", 3);
        store.add("Orange", 1);
        store.add("Banana", 0);
        let kinds = store.get_item_kinds();
        assert!(kinds.contains(&&"Apple".to_string()));
        assert!(kinds.contains(&&"Orange".to_string()));
        assert!(!kinds.contains(&&"Banana".to_string()));
    }
}
