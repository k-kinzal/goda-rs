use apollo_compiler::hir::{OperationDefinition, OperationType};
use std::sync::Arc;

mod query;

pub struct Operation {
    ast: Arc<OperationDefinition>,
}

impl Operation {
    pub fn new(ast: Arc<OperationDefinition>) -> Self {
        Self { ast }
    }

    pub fn query(&self) -> String {
        todo!()
    }

    pub fn to_string(&self) -> String {
        let operation_name = self.ast.operation_ty().to_string();
        self.ast.directives()
        todo!()
    }
}
