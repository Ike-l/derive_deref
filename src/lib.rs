#![crate_name = "derive_deref"]
//! ## Example
//! 
//! ```rust
//! use std::ops::DerefMut;
//! use derive_deref::{Deref, DerefMut};
//! 
//! #[derive(Deref, DerefMut)]
//! struct WrapperStructDifferentTargets {
//!     #[DerefTarget]
//!     field: i32,
//!     #[DerefMutTarget]
//!     field_mut: i32,
//! }
//! 
//! fn struct_deref_mut_different_targets() {
//!     let mut w = WrapperStructDifferentTargets { field: 1, field_mut: 2};
//!     *w *= 2;
//!     assert_eq!(*w, 1);
//!     assert_eq!(*w.deref_mut(), 4);
//! }
//! 
//! #[derive(Deref, DerefMut)]
//! struct WrapperTuple(i32, i32);
//! 
//! fn tuple_deref_mut() {
//!     let mut w = WrapperTuple(1, 3);
//!     *w *= 2;
//!     assert_eq!(*w, 2);
//!     assert_eq!(*w.deref_mut(), 2);
//! }
//! ```


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Defines Deref either on a struct or a tuple.<br>
/// ## Example
/// 
/// ```rust
/// use derive_deref::Deref;
/// 
/// #[derive(Deref)]
/// struct WrapperStruct {
///     #[DerefTarget]
///     field: i32,
/// }
/// 
/// fn struct_deref() {
///     let w = WrapperStruct { field: 1 };
///     assert_eq!(*w, 1);
/// }
/// 
/// #[derive(Deref)]
/// struct WrapperTuple(i32);
/// 
/// fn tuple_deref() {
///     let w = WrapperTuple(1);
///    assert_eq!(*w, 1);
/// }
/// ```
#[proc_macro_derive(Deref, attributes(DerefTarget))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named) => {
                    let field = named.named.iter()
                        .find_map(|field| {
                            for attr in &field.attrs {
                                if attr.path().is_ident("DerefTarget") {
                                    return Some(field);
                                }
                            }
                            None
                        });
                    if let Some(field) = field {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        quote! {
                            impl std::ops::Deref for #name {
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
                        impl std::ops::Deref for #name {
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
/// # Examples
/// ## Struct
/// ```rust
/// use std::ops::DerefMut;
/// use derive_deref::{Deref, DerefMut};
/// 
/// #[derive(DerefMut, Deref)]
/// struct WrapperStruct {
///     #[DerefMutTarget]
///     #[DerefTarget]
///     field: i32,
/// }
///
/// fn struct_deref_mut() {
///     let mut w = WrapperStruct { field: 1 };
///     *w *= 2;
///     assert_eq!(*w.deref_mut(), 2);
/// }
/// 
/// #[derive(DerefMut, Deref)]
/// struct WrapperTuple(i32);
/// 
/// fn tuple_deref() {
///     let mut w = WrapperTuple(1);
///     *w *= 2;
///     assert_eq!(*w.deref_mut(), 2);
/// }
/// ```
#[proc_macro_derive(DerefMut, attributes(DerefMutTarget))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named) => {
                    let field = named.named.iter().find_map(|field| {
                        for attr in &field.attrs {
                            if attr.path().is_ident("DerefMutTarget") {
                                return Some(field);
                            }
                        }
                        None
                    });

                    if let Some(field) = field {
                        let field_name = &field.ident;
                        quote! {
                            impl std::ops::DerefMut for #struct_name {
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
                        impl std::ops::DerefMut for #struct_name {
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