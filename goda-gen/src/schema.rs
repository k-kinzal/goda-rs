use crate::operation::Operation;
use crate::schema::database::Database;
use crate::schema::definitions::schema::SchemaDefinition;
use crate::schema::definitions::Definition;
use apollo_compiler::{hir, ApolloCompiler, ApolloDiagnostic, HirDatabase};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

mod database;
mod definitions;

pub struct CompileError {
    diagnostics: Vec<ApolloDiagnostic>,
}

impl Debug for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CompileError {}

pub struct Schema {
    compiler: ApolloCompiler,
}

impl Schema {
    pub fn parse(input: &str) -> Result<Self, CompileError> {
        let mut compiler = ApolloCompiler::new();
        compiler.create_schema(input, "schema.graphql");
        let diagnostics = compiler.validate();
        if diagnostics.len() > 0 {
            Err(CompileError { diagnostics })
        } else {
            Ok(Self { compiler })
        }
    }

    pub fn create_operation(
        &mut self,
        input: &str,
        filename: Option<&str>,
    ) -> Result<Operation, CompileError> {
        let query_id = self
            .compiler
            .create_executable(input, filename.unwrap_or_else(|| "operation.graphql"));

        let diagnostics = self.compiler.validate();
        if !diagnostics.is_empty() {
            Err(CompileError { diagnostics })
        } else {
            let operation = self
                .compiler
                .db
                .find_operation_by_name(query_id, String::from("getProduct"))
                .expect("unreachable");
            Ok(Operation::new(operation))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let schema = r#"
            """
            My Schema
            nigyou me
            """
            schema @deprecated(reason: "なんとなく") {
              query: Query
              mutation: Mutation
            }
        "#;
        let _ = Schema::parse(schema);
    }
}
