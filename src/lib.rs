#![crate_name = "small_derive_deref"]
//! ## Example
//! 
//! ```rust
//! use std::ops::DerefMut;
//! use small_derive_deref::{Deref, DerefMut};
//! 
//! #[derive(Deref, DerefMut)]
//! struct WrapperStructDifferentTargetsMultipleGenerics<'a, T> {
//!     #[DerefTarget]
//!     field: &'a str,
//!     #[DerefMutTarget]
//!     field_mut: &'a str,
//!     foo: T
//! }
//! 
//! let mut w = WrapperStructDifferentTargetsMultipleGenerics { field: "not rust", field_mut: "rust", foo: "foo"};
//! assert_eq!(*w, "not rust");
//! *w = "rUst";
//! assert_eq!(*w.deref_mut(), "rUst");
//! 
//! #[derive(Deref, DerefMut)]
//! struct WrapperTuple(i32, i32);
//! 
//! let mut w = WrapperTuple(1, 3);
//! *w *= 2;
//! assert_eq!(*w, 2);
//! assert_eq!(*w.deref_mut(), 2);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Defines Deref either on a struct or a tuple.<br>
/// No attribute needed for structs with one field or tuples.<br>
/// ## Example
/// 
/// ```rust
/// use small_derive_deref::Deref;
/// 
/// #[derive(Deref)]
/// struct WrapperStruct {
///     field: i32,
/// }
/// 
/// let w = WrapperStruct { field: 1 };
/// assert_eq!(*w, 1);
/// 
/// #[derive(Deref)]
/// struct WrapperMultipleStruct {
///     #[DerefTarget]
///     field1: i32,
///     field2: i32,
/// }
/// 
/// let w = WrapperMultipleStruct { field1: 1, field2: 2 };
/// 
/// assert_eq!(*w, 1);
/// 
/// #[derive(Deref)]
/// struct WrapperTuple(i32);
/// 
/// let w = WrapperTuple(1);
/// assert_eq!(*w, 1);
/// ```
#[proc_macro_derive(Deref, attributes(DerefTarget))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named) => {
                    let field = match named.named.len() {
                        1 => { named.named.first() }
                        _ => named.named.iter()
                            .find_map(|field| {
                                for attr in &field.attrs {
                                    if attr.path().is_ident("DerefTarget") {
                                        return Some(field);
                                    }
                                }
                                None
                            })
                    };

                    if let Some(field) = field {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        quote! {
                            impl #impl_generics std::ops::Deref for #name #type_generics #where_clause {
                                type Target = #field_type;

                                fn deref(&self) -> &Self::Target {
                                    &self.#field_name
                                }
                            }
                        }
                    } else {
                        panic!("No field with #[DerefTarget]")
                    }
                },
                Fields::Unnamed(unnamed) => {
                    let field_type = &unnamed.unnamed[0].ty;

                    quote! {
                        impl #impl_generics std::ops::Deref for #name #type_generics #where_clause {
                            type Target = #field_type;

                            fn deref(&self) -> &Self::Target {
                                &self.0
                            }
                        }
                    }
                    
                },
                Fields::Unit => panic!("Deref not implemented for unit structs")
            }
        },
        _ => panic!("Deref only implemented for structs")
    };

    expanded.into()
}


/// Defines DerefMut either on a struct or a tuple.<br>
/// No attribute needed for structs with one field or tuples.<br>
/// # Examples
/// ## Struct
/// ```rust
/// use std::ops::DerefMut;
/// use small_derive_deref::{Deref, DerefMut};
/// 
/// #[derive(DerefMut, Deref)]
/// struct WrapperStruct {
///     field: i32,
/// }
///
/// let mut w = WrapperStruct { field: 1 };
/// *w *= 2;
/// assert_eq!(*w.deref_mut(), 2);
/// 
/// #[derive(DerefMut, Deref)]
/// struct WrapperMultipleStruct {
///     #[DerefTarget]
///     field1: i32,
///     #[DerefMutTarget]
///     field2: i32,
/// }
/// 
/// let mut w = WrapperMultipleStruct { field1: 1, field2: 3 };
/// *w *= 2;
/// assert_eq!(*w.deref_mut(), 6);
/// assert_eq!(*w, 1);
/// 
/// #[derive(DerefMut, Deref)]
/// struct WrapperTuple(i32);
/// 
/// let mut w = WrapperTuple(1);
/// *w *= 2;
/// assert_eq!(*w.deref_mut(), 2);
/// ```
#[proc_macro_derive(DerefMut, attributes(DerefMutTarget))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match input.data {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named) => {
                    let field = match named.named.len() {
                        1 => { named.named.first() }
                        _ => named.named.iter()
                        .find_map(|field| {
                            for attr in &field.attrs {
                                if attr.path().is_ident("DerefMutTarget") {
                                    return Some(field);
                                }
                            }
                            None
                        })
                    };

                    if let Some(field) = field {
                        let field_name = &field.ident;
                        quote! {
                            impl #impl_generics std::ops::DerefMut for #struct_name #type_generics #where_clause {
                                fn deref_mut(&mut self) -> &mut Self::Target {
                                    &mut self.#field_name
                                }
                            }
                        }
                    } else {
                        panic!("No field marked with #[DerefMutTarget]")
                    }
                },
                Fields::Unnamed(_) => {
                    quote! {
                        impl #impl_generics std::ops::DerefMut for #struct_name #type_generics #where_clause {
                            fn deref_mut(&mut self) -> &mut Self::Target {
                                &mut self.0
                            }
                        }
                    }
                },
                Fields::Unit => panic!("DerefMut not implemented for unit structs"),
            }
        },
        _ => panic!("DerefMut only implemented for structs"),
    };

    expanded.into()
}