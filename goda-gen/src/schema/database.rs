use crate::schema::definitions::Definition;
use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, Definition>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, def: Definition) {
        if self.data.get(def.name()).is_none() {
            self.data.insert(def.name().to_string(), def);
        } else {
            panic!("Duplicate definition: {}", def.name());
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&Definition> {
        self.data.get(name)
    }
}
