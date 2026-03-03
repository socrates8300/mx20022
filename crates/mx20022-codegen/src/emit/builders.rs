//! Emit builder pattern code for [`StructDef`] types.
//!
//! For each struct, we generate:
//!
//! - A `{Name}Builder` struct where every field is stored as `Option<T>` for
//!   required/optional fields, or `Vec<T>` for vec fields.
//! - A `Default` impl initialising all fields to `None` / empty `Vec`.
//! - Setter methods that consume and return `self` for chaining.
//!   Vec fields additionally get an `add_{field}` method.
//! - A `build(self) -> Result<{Name}, crate::common::BuilderError>` method
//!   that validates all required fields are set.
//! - An associated `{Name}::builder() -> {Name}Builder` function on the
//!   original struct.

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{Cardinality, FieldDef, StructDef, TypeRef};

use super::util::{make_ident, type_ref_tokens};

/// Emit the builder struct, its `Default` impl, setter methods, `build()`
/// method, and the `{Name}::builder()` associated function.
///
/// `choice_types` is the set of xs:choice enum names in the schema.  Fields
/// whose type is in this set are wrapped in
/// `crate::common::ChoiceWrapper<T>` to match the generated struct exactly.
pub fn emit_builder(def: &StructDef, choice_types: &HashSet<String>) -> TokenStream {
    let struct_name = make_ident(&def.name);
    let builder_name = make_ident(&format!("{}Builder", def.name));

    // ── Builder struct fields ─────────────────────────────────────────────────
    // For Required/Optional fields   → Option<BaseType>
    // For Vec/BoundedVec fields       → Vec<BaseType>
    let builder_fields: TokenStream = def
        .fields
        .iter()
        .map(|f| emit_builder_field(f, choice_types))
        .collect();

    // ── Setter methods ────────────────────────────────────────────────────────
    let setters: TokenStream = def
        .fields
        .iter()
        .map(|f| emit_setter(f, choice_types, &builder_name))
        .collect();

    // ── build() method ────────────────────────────────────────────────────────
    let has_required = def
        .fields
        .iter()
        .any(|f| matches!(&f.cardinality, Cardinality::Required));

    let missing_checks: TokenStream = def
        .fields
        .iter()
        .filter_map(|f| {
            if matches!(&f.cardinality, Cardinality::Required) {
                let rust_name = make_ident(&f.rust_name);
                let field_name_str = &f.rust_name;
                Some(quote! {
                    if self.#rust_name.is_none() {
                        missing.push(#field_name_str.to_owned());
                    }
                })
            } else {
                None
            }
        })
        .collect();

    let field_assignments: TokenStream = def.fields.iter().map(emit_build_assignment).collect();

    let struct_name_str = &def.name;

    // Generate the build() body.  When there are no required fields, skip the
    // `let mut missing` declaration to avoid an `unused_mut` lint.
    let build_body: TokenStream = if has_required {
        quote! {
            let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
            #missing_checks
            if !missing.is_empty() {
                return ::std::result::Result::Err(crate::common::BuilderError {
                    type_name: #struct_name_str.to_owned(),
                    missing_fields: missing,
                });
            }
            ::std::result::Result::Ok(#struct_name {
                #field_assignments
            })
        }
    } else {
        quote! {
            ::std::result::Result::Ok(#struct_name {
                #field_assignments
            })
        }
    };

    let builder_doc = format!(
        " Builder for [`{struct_name_str}`]. Construct via [`{struct_name_str}::builder()`].",
    );

    quote! {
        #[doc = #builder_doc]
        #[allow(clippy::struct_field_names)]
        #[derive(Default)]
        pub struct #builder_name {
            #builder_fields
        }

        impl #builder_name {
            #setters

            /// Validate required fields and construct the type.
            ///
            /// # Errors
            ///
            /// Returns [`crate::common::BuilderError`] listing the names of any
            /// required fields that were not set.
            ///
            /// # Panics
            ///
            /// Does not panic — all `.unwrap()` calls are guarded by the
            /// missing-field check above.
            pub fn build(self) -> ::std::result::Result<#struct_name, crate::common::BuilderError> {
                #build_body
            }
        }

        impl #struct_name {
            /// Return a new builder for this type.
            #[must_use]
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
    }
}

// ── Field helpers ─────────────────────────────────────────────────────────────

/// Resolve the "base" type token stream for a field, applying `ChoiceWrapper` if
/// the type is an xs:choice enum.
fn base_type(field: &FieldDef, choice_types: &HashSet<String>) -> TokenStream {
    let is_choice = match &field.type_ref {
        TypeRef::Named(n) => choice_types.contains(n),
        TypeRef::Builtin(_) => false,
    };
    if is_choice {
        let inner = type_ref_tokens(&field.type_ref);
        quote! { crate::common::ChoiceWrapper<#inner> }
    } else {
        type_ref_tokens(&field.type_ref)
    }
}

/// Emit one field declaration inside the builder struct.
fn emit_builder_field(field: &FieldDef, choice_types: &HashSet<String>) -> TokenStream {
    let rust_name = make_ident(&field.rust_name);
    let bt = base_type(field, choice_types);
    match &field.cardinality {
        // Both Required and Optional fields are stored as Option<T> inside the builder.
        Cardinality::Required | Cardinality::Optional => quote! {
            #rust_name: ::std::option::Option<#bt>,
        },
        Cardinality::Vec | Cardinality::BoundedVec(_) => quote! {
            #rust_name: ::std::vec::Vec<#bt>,
        },
    }
}

/// Emit setter method(s) for one field on the builder.
///
/// - Required/Optional → one setter: `fn {name}(mut self, value: T) -> Self`
/// - Vec/BoundedVec    → two setters:
///   - `fn {name}(mut self, value: Vec<T>) -> Self`
///   - `fn add_{name}(mut self, value: T) -> Self`
fn emit_setter(
    field: &FieldDef,
    choice_types: &HashSet<String>,
    builder_name: &proc_macro2::Ident,
) -> TokenStream {
    let rust_name = make_ident(&field.rust_name);
    let bt = base_type(field, choice_types);
    let set_doc = format!(" Set the `{}` field.", field.rust_name);

    match &field.cardinality {
        Cardinality::Required | Cardinality::Optional => {
            quote! {
                #[doc = #set_doc]
                #[must_use]
                pub fn #rust_name(mut self, value: #bt) -> #builder_name {
                    self.#rust_name = ::std::option::Option::Some(value);
                    self
                }
            }
        }
        Cardinality::Vec | Cardinality::BoundedVec(_) => {
            let add_name = make_ident(&format!("add_{}", field.rust_name));
            let set_doc_vec = format!(
                " Set the `{}` field (replaces any previously added items).",
                field.rust_name
            );
            let add_doc = format!(" Append one item to the `{}` field.", field.rust_name);
            quote! {
                #[doc = #set_doc_vec]
                #[must_use]
                pub fn #rust_name(mut self, value: ::std::vec::Vec<#bt>) -> #builder_name {
                    self.#rust_name = value;
                    self
                }

                #[doc = #add_doc]
                #[must_use]
                pub fn #add_name(mut self, value: #bt) -> #builder_name {
                    self.#rust_name.push(value);
                    self
                }
            }
        }
    }
}

/// Emit one field assignment in the `build()` method's struct literal.
fn emit_build_assignment(field: &FieldDef) -> TokenStream {
    let rust_name = make_ident(&field.rust_name);
    match &field.cardinality {
        // Required fields: we've already validated they're Some; unwrap is safe.
        Cardinality::Required => quote! {
            #rust_name: self.#rust_name.unwrap(),
        },
        // Optional / Vec fields: pass through as-is.
        Cardinality::Optional | Cardinality::Vec | Cardinality::BoundedVec(_) => quote! {
            #rust_name: self.#rust_name,
        },
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Cardinality, FieldDef, RustType, StructDef, TypeRef};

    fn field(
        xml_name: &str,
        rust_name: &str,
        type_ref: TypeRef,
        cardinality: Cardinality,
    ) -> FieldDef {
        FieldDef {
            xml_name: xml_name.to_owned(),
            rust_name: rust_name.to_owned(),
            type_ref,
            cardinality,
        }
    }

    fn no_choices() -> HashSet<String> {
        HashSet::new()
    }

    fn choice_set(names: &[&str]) -> HashSet<String> {
        names.iter().map(|&s| s.to_owned()).collect()
    }

    // Helper: emit and format to a string for assertion.
    fn emit_str(def: &StructDef, choices: &HashSet<String>) -> String {
        let ts = emit_builder(def, choices);
        let file_str = ts.to_string();
        let parsed = syn::parse_file(&file_str)
            .unwrap_or_else(|e| panic!("builder emit produced invalid Rust: {e}\n{file_str}"));
        prettyplease::unparse(&parsed)
    }

    #[test]
    fn builder_for_required_field_is_valid_rust() {
        let def = StructDef {
            name: "MyMsg".to_owned(),
            fields: vec![field(
                "Id",
                "id",
                TypeRef::Builtin(RustType::String),
                Cardinality::Required,
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("MyMsgBuilder"), "builder struct: {src}");
        assert!(src.contains("pub struct MyMsgBuilder"), "{src}");
        // Required field stored as Option inside builder
        assert!(
            src.contains("Option<String>") || src.contains("Option < String >"),
            "{src}"
        );
    }

    #[test]
    fn builder_for_optional_field() {
        let def = StructDef {
            name: "Foo".to_owned(),
            fields: vec![field(
                "Val",
                "val",
                TypeRef::Builtin(RustType::String),
                Cardinality::Optional,
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("FooBuilder"), "{src}");
        // Setter exists
        assert!(src.contains("pub fn val"), "{src}");
    }

    #[test]
    fn builder_for_vec_field_has_add_method() {
        let def = StructDef {
            name: "Container".to_owned(),
            fields: vec![field(
                "Items",
                "items",
                TypeRef::Named("ItemType".to_owned()),
                Cardinality::Vec,
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("pub fn items"), "{src}");
        assert!(src.contains("pub fn add_items"), "{src}");
    }

    #[test]
    fn builder_for_bounded_vec_field_has_add_method() {
        let def = StructDef {
            name: "Container".to_owned(),
            fields: vec![field(
                "Items",
                "items",
                TypeRef::Named("ItemType".to_owned()),
                Cardinality::BoundedVec(10),
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("pub fn items"), "{src}");
        assert!(src.contains("pub fn add_items"), "{src}");
    }

    #[test]
    fn builder_choice_field_uses_choice_wrapper() {
        let def = StructDef {
            name: "Hdr".to_owned(),
            fields: vec![field(
                "Fr",
                "fr",
                TypeRef::Named("Party51Choice".to_owned()),
                Cardinality::Required,
            )],
        };
        let choices = choice_set(&["Party51Choice"]);
        let src = emit_str(&def, &choices);
        assert!(
            src.contains("ChoiceWrapper<Party51Choice>")
                || src.contains("ChoiceWrapper < Party51Choice >"),
            "choice field must use ChoiceWrapper: {src}"
        );
    }

    #[test]
    fn builder_build_method_present() {
        let def = StructDef {
            name: "Thing".to_owned(),
            fields: vec![
                field(
                    "A",
                    "a",
                    TypeRef::Builtin(RustType::String),
                    Cardinality::Required,
                ),
                field(
                    "B",
                    "b",
                    TypeRef::Builtin(RustType::String),
                    Cardinality::Optional,
                ),
            ],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("pub fn build"), "{src}");
        assert!(src.contains("BuilderError"), "{src}");
    }

    #[test]
    fn struct_builder_associated_fn_present() {
        let def = StructDef {
            name: "Thing".to_owned(),
            fields: vec![field(
                "X",
                "x",
                TypeRef::Builtin(RustType::Bool),
                Cardinality::Required,
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("impl Thing"), "{src}");
        assert!(src.contains("pub fn builder"), "{src}");
    }

    #[test]
    fn empty_struct_builder_is_valid_rust() {
        let def = StructDef {
            name: "Empty".to_owned(),
            fields: vec![],
        };
        let src = emit_str(&def, &no_choices());
        syn::parse_file(&src).expect("must be valid Rust");
    }

    #[test]
    fn keyword_field_name_in_builder() {
        // "type" is a Rust keyword — must be emitted as r#type
        let def = StructDef {
            name: "Foo".to_owned(),
            fields: vec![field(
                "Type",
                "type",
                TypeRef::Builtin(RustType::String),
                Cardinality::Required,
            )],
        };
        let src = emit_str(&def, &no_choices());
        assert!(src.contains("r#type"), "keyword field not escaped: {src}");
    }

    #[test]
    fn full_struct_with_all_cardinalities_is_valid_rust() {
        let def = StructDef {
            name: "Full".to_owned(),
            fields: vec![
                field(
                    "Req",
                    "req",
                    TypeRef::Builtin(RustType::String),
                    Cardinality::Required,
                ),
                field(
                    "Opt",
                    "opt",
                    TypeRef::Builtin(RustType::Bool),
                    Cardinality::Optional,
                ),
                field(
                    "Many",
                    "many",
                    TypeRef::Named("ItemT".to_owned()),
                    Cardinality::Vec,
                ),
                field(
                    "Bounded",
                    "bounded",
                    TypeRef::Named("ItemT".to_owned()),
                    Cardinality::BoundedVec(5),
                ),
            ],
        };
        let src = emit_str(&def, &no_choices());
        syn::parse_file(&src).expect("must be valid Rust");
        assert!(src.contains("pub fn req"), "{src}");
        assert!(src.contains("pub fn opt"), "{src}");
        assert!(src.contains("pub fn many"), "{src}");
        assert!(src.contains("pub fn add_many"), "{src}");
        assert!(src.contains("pub fn bounded"), "{src}");
        assert!(src.contains("pub fn add_bounded"), "{src}");
    }
}
