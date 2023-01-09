use apollo_parser::ast::{AstNode, Value};
use quote::__private::TokenStream;
use quote::{format_ident, quote};

pub struct SchemaDefinition(apollo_parser::ast::SchemaDefinition);

impl SchemaDefinition {
    pub fn new(def: apollo_parser::ast::SchemaDefinition) -> Self {
        Self(def)
    }

    pub fn name(&self) -> &str {
        todo!()
    }
}

impl ToString for SchemaDefinition {
    fn to_string(&self) -> String {
        let description_comment = self
            .0
            .description()
            .and_then(|description| {
                description.string_value().map(|v| {
                    v.source_string()
                        .split('\n')
                        .filter(|s| s.trim() != r#"""""#)
                        .filter(|s| s.trim() != "")
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                })
            })
            .map(|s| {
                quote! {
                    #[doc = #s]
                }
            });
        let directives_macro = self
            .0
            .directives()
            .map(|v| {
                v.directives()
                    .map(|directive| {
                        let name = directive
                            .name()
                            .map(|v| format_ident!("{}", v.source_string()));
                        let arguments = directive
                            .arguments()
                            .map(|v| {
                                v.arguments()
                                    .map(|argument| {
                                        let name = argument
                                            .name()
                                            .map(|v| format_ident!("{}", v.text().to_string()));
                                        let value = argument.value().map(|v| match v {
                                            Value::Variable(v) => v.text().to_string(),
                                            Value::StringValue(v) => v.syntax().to_string(),
                                            Value::FloatValue(v) => v.source_string(),
                                            Value::IntValue(v) => v.source_string(),
                                            Value::BooleanValue(v) => v.source_string(),
                                            Value::NullValue(v) => v.source_string(),
                                            Value::EnumValue(v) => v.source_string(),
                                            Value::ListValue(v) => v.source_string(),
                                            Value::ObjectValue(v) => v.source_string(),
                                        });
                                        quote! {
                                            #name = #value
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        quote! {
                            #name(#(#arguments),*)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .map(|v| {
                quote! {
                    #[directives(#(#v),*)]
                }
            })
            .unwrap_or_else(|| quote! {});
        let fields = self
            .0
            .root_operation_type_definitions()
            .map(|v| {
                let name = v
                    .operation_type()
                    .map(|v| format_ident!("{}", v.source_string()));
                let ty = v
                    .named_type()
                    .and_then(|v| v.name().map(|v| format_ident!("{}", v.text().to_string())));
                quote! {
                    #name: #ty,
                }
            })
            .collect::<Vec<TokenStream>>();
        let token = quote! {
            #description_comment
            #[derive(GraphQL)]
            #directives_macro
            struct Schema {
                #(#fields)*
            }
        };
        token.to_string()
    }
}
