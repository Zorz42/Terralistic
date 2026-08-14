#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::libraries::registry::{NamedEntry, Registry, RegistryEntry, RegistryId};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct TestId {
        id: i8,
    }

    impl TestId {
        const fn undefined() -> Self {
            Self { id: -1 }
        }
    }

    impl RegistryId for TestId {
        const KIND: &'static str = "test type";

        fn from_index(index: usize) -> Self {
            Self { id: index as i8 }
        }

        fn index(self) -> Option<usize> {
            usize::try_from(self.id).ok()
        }
    }

    #[derive(Debug)]
    struct TestEntry {
        name: String,
        id: TestId,
    }

    impl TestEntry {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                id: TestId::undefined(),
            }
        }
    }

    impl RegistryEntry<TestId> for TestEntry {
        fn set_id(&mut self, id: TestId) {
            self.id = id;
        }
    }

    impl NamedEntry for TestEntry {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    fn registry_of(names: &[&str]) -> Registry<TestId, TestEntry> {
        let mut registry = Registry::new();
        for name in names {
            registry.register(TestEntry::new(name));
        }
        registry
    }

    #[test]
    fn test_new_registry_is_empty() {
        let registry = Registry::<TestId, TestEntry>::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(registry.ids().is_empty());
        registry.get(TestId::from_index(0)).unwrap_err();
    }

    #[test]
    fn test_register_hands_out_ids_in_order() {
        let mut registry = Registry::new();

        let first = registry.register(TestEntry::new("first"));
        let second = registry.register(TestEntry::new("second"));

        assert_ne!(first, second);
        assert_eq!(registry.get(first).unwrap().name, "first");
        assert_eq!(registry.get(second).unwrap().name, "second");
        assert_eq!(registry.len(), 2);
    }

    /// An entry is handed around away from the registry that owns it, so it has to be told
    /// its own handle when it is registered.
    #[test]
    fn test_register_stamps_the_id_into_the_entry() {
        let mut registry = Registry::new();

        let id = registry.register(TestEntry::new("stamped"));

        assert_eq!(registry.get(id).unwrap().id, id);
    }

    #[test]
    fn test_get_mut() {
        let mut registry = registry_of(&["one"]);
        let id = TestId::from_index(0);

        "renamed".clone_into(&mut registry.get_mut(id).unwrap().name);

        assert_eq!(registry.get(id).unwrap().name, "renamed");
    }

    #[test]
    fn test_unknown_id_is_an_error() {
        let registry = registry_of(&["one"]);

        registry.get(TestId::from_index(1)).unwrap_err();
        registry.get(TestId::from_index(99)).unwrap_err();
    }

    /// Every one of these handle types has an "undefined" value, and it must resolve to
    /// nothing rather than to whatever entry a negative index casts to.
    #[test]
    fn test_undefined_id_resolves_to_nothing() {
        let mut registry = registry_of(&["one", "two"]);

        registry.get(TestId::undefined()).unwrap_err();
        registry.get_mut(TestId::undefined()).unwrap_err();
    }

    #[test]
    fn test_lookup_by_name() {
        let registry = registry_of(&["dirt", "stone"]);

        assert_eq!(registry.get_id_by_name("stone").unwrap(), TestId::from_index(1));
        assert_eq!(registry.get_by_name("dirt").unwrap().name, "dirt");
    }

    #[test]
    fn test_lookup_by_unknown_name_is_an_error() {
        let registry = registry_of(&["dirt"]);

        registry.get_id_by_name("granite").unwrap_err();
        registry.get_by_name("granite").unwrap_err();
    }

    /// The error names what kind of thing was being looked for, which is the whole reason
    /// `RegistryId::KIND` exists - one generic registry must not produce one generic
    /// message for five different registries.
    #[test]
    fn test_errors_name_the_kind() {
        let registry = registry_of(&["dirt"]);

        let by_id = registry.get(TestId::from_index(7)).unwrap_err().to_string();
        let by_name = registry.get_id_by_name("granite").unwrap_err().to_string();

        assert!(by_id.contains("test type"), "id error should name the kind, got {by_id:?}");
        assert!(by_name.contains("test type"), "name error should name the kind, got {by_name:?}");
        assert!(by_name.contains("granite"), "name error should name what was asked for, got {by_name:?}");
    }

    #[test]
    fn test_ids_returns_every_handle_in_order() {
        let registry = registry_of(&["a", "b", "c"]);

        let ids = registry.ids();

        assert_eq!(ids.len(), 3);
        for (index, id) in ids.iter().enumerate() {
            assert_eq!(registry.get(*id).unwrap().id, *id, "id {index} does not resolve to its own entry");
        }
    }

    #[test]
    fn test_iter_visits_every_entry_in_registration_order() {
        let registry = registry_of(&["a", "b", "c"]);

        let names: Vec<&str> = registry.iter().map(|entry| entry.name.as_str()).collect();

        assert_eq!(names, vec!["a", "b", "c"]);
    }

    /// Two names that are the same is not an error - the first one registered wins the
    /// lookup, and both keep their own handle.
    #[test]
    fn test_duplicate_names_keep_distinct_ids() {
        let registry = registry_of(&["same", "same"]);

        assert_eq!(registry.get_id_by_name("same").unwrap(), TestId::from_index(0));
        assert_eq!(registry.len(), 2);
    }
}
