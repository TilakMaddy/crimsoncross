#[macro_export]
/*
 * - Add the `Span` field to the first struct in AST Node
 * - The first struct is made `pub`
 * - Keep the rest of the token tree can be anything
 */
macro_rules! ast_node {
    (
       $(#[$struct_meta:meta])*
       struct $name:ident {
           $(
               $(#[$field_meta:meta])*
               $field_ident:ident: $field_type:ty
           ,)*
       }

       $($dependents:tt)*
    ) => {
       $(#[$struct_meta])*
       #[derive(Debug)]
       pub struct $name {
           $(
               $(#[$field_meta])*
               pub $field_ident: $field_type
           ,)*
           pub span: $crate::span::Span,
       }

       $(
           $dependents
       )*
    };
}

// References:
//
// 1. Little book of Rust macros
// - The only page you must read to create *any* declarative macro your heart desires.
// - https://veykril.github.io/tlborm/decl-macros/macros-methodical.html.
