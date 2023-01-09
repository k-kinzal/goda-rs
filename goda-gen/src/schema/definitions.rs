use crate::schema::definitions::schema::SchemaDefinition;

pub mod schema;

pub enum Definition {
    Schema(SchemaDefinition),
}

impl Definition {
    pub fn name(&self) -> &str {
        match self {
            Definition::Schema(def) => def.name(),
        }
    }
}
