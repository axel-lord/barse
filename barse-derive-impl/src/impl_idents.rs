//! [ImplIdents] impl.

use ::quote::format_ident;
use ::syn::Token;

use crate::{
    opt::{WithPat, WithPatPat},
    unit_ty,
};

/// Collection of generated identifiers used for impls.
#[derive(Debug, Clone)]
pub struct ImplIdents {
    /// Random number used.
    pub _r: u32,

    /// Ident of endian generic param.
    pub endian_ident: ::syn::Ident,

    /// Ident of byte source/sink generic param.
    pub byte_ident: ::syn::Ident,

    /// Ident of with param.
    pub with_ident: ::syn::Ident,

    /// Ident of to param.
    pub to_ident: ::syn::Ident,

    /// Ident of from param.
    pub from_ident: ::syn::Ident,

    /// Ident of discriminant.
    pub discriminant_ident: ::syn::Ident,
}

impl ImplIdents {
    /// Generate identifiers.
    pub fn new() -> Self {
        let r = ::rand::random::<u32>();

        Self {
            _r: r,
            endian_ident: format_ident!("__E_{r:X}"),
            byte_ident: format_ident!("__B_{r:X}"),
            with_ident: format_ident!("__with_{r:x}"),
            to_ident: format_ident!("__to_{r:x}"),
            from_ident: format_ident!("__from_{r:x}"),
            discriminant_ident: format_ident!("__dsicriminant_{r:x}"),
        }
    }

    /// Get default with value.
    pub fn default_with(&self) -> WithPat {
        WithPat {
            pat: Some(WithPatPat {
                pat: self.with_ident.clone(),
                colon_token: <Token![:]>::default(),
            }),
            ty: unit_ty(),
        }
    }
}
