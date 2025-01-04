//! Macros for generating Option structs.

/// Generate option structs.
macro_rules! opt {
    ($(
        $(#[doc = $sdoc:expr])*
        $([parser $par:path])?
        $nm:ident {$(
            $(#[doc = $fdoc:expr])*
            $([attr $attr:ident])?
            $([call $p:path])?
            $([opt $e:expr])?
            $f_nm:ident: $f_ty:ty,
        )*}
    ),* $(,)?) => {$(
        #[derive(Debug, Clone)]
        $(#[doc = $sdoc])*
        pub struct $nm {$(
            $(#[doc = $fdoc])*
            pub $f_nm: $f_ty,
        )*}

        $crate::opt::opt_macro::impl_opt_trait!($nm $(, $f_nm: $f_ty)*);
        $crate::opt::opt_macro::impl_deref_into!($nm $(,$([attr $attr])? $f_nm: $f_ty)*);
        $crate::opt::opt_macro::impl_parse!(
            $(@parser $par,)*
            $nm,
            $(
            $f_nm,
            {
                $(opt: $e,)*
                $(call: $p,)*
            },
            )*
        );

        impl ToTokens for $nm {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                $(self.$f_nm.to_tokens(tokens);)*
            }
        }
    )*};
}

/// Implement parse for type.
macro_rules! impl_parse {
    (@parser $p:path, $nm:ident, $($_tt:tt)*) => {
        impl Parse for $nm {
            fn parse(input: ParseStream) -> ::syn::Result<Self> {
                $p(input)
            }
        }
    };
    (
        $nm:ident,
        $($f_nm:ident, {
            $(opt: $e:expr,)?
            $(call: $p:path,)?
        },)*
    ) => {
        impl Parse for $nm {
            fn parse(input: ParseStream) -> ::syn::Result<Self> {
                Ok(Self {$(
                    $f_nm: $crate::opt::opt_macro::parse_field!(input $(, opt = $e)* $(, call = $p)*),
                )*})
            }
        }
    };
}

/// Parse for field.
macro_rules! parse_field {
    ($input:expr, opt = $e:expr $(, call = $p:path)?) => {
        if $input.peek($e) {
            Some($crate::opt::opt_macro::parse_field!($input $(, call = $p )*))
        } else {
            None
        }
    };
    ($input:expr, call = $p:path) => {
        $input.call($p)?
    };
    ($input:expr) => {
        $input.parse()?
    };
}

/// Implement Opt trait if first field is named kw.
macro_rules! impl_opt_trait {
    ($nm:ident, kw: $kw:path $(,$_f_nm:ident: $_f:ty)*) => {
        impl Opt for $nm {
            fn peek(lookahead: &::syn::parse::Lookahead1) -> bool {
                lookahead.peek($kw)
            }

            fn name() -> impl ::core::fmt::Display {
                <$kw>::default().into_token_stream()
            }

            fn kw_span(&self) -> Span {
                ::syn::spanned::Spanned::span(&self.kw)
            }
        }
    };
    ($_nm:ident $(,$_f_nm:ident: $_f:ty)*) => {};
}

/// Implement deref to last field.
macro_rules! impl_deref_into {
    ($nm:ident, [attr deref] $f_nm:ident: $f_ty:ty, $($tt:tt)* ) => {
        $crate::opt::opt_macro::impl_deref_into!($nm, $f_nm: $f_ty);
    };
    (
        $nm:ident,
        $([attr $($_tt:tt)*])?
        $_f_nm:ident: $_f_ty:ty
        $(,
            $([attr $($att:tt)*])?
            $f_nm:ident:$f_ty:ty
        )+) => {
        $crate::opt::opt_macro::impl_deref_into!($nm $(,$([attr $($att)*])* $f_nm: $f_ty)*);
    };
    ($nm:ident, $([attr $($tt:tt)*])? $f_nm:ident: $f_ty:ty) => {
        impl ::core::ops::Deref for $nm {
            type Target = $f_ty;

            fn deref(&self) -> &Self::Target {
                &self.$f_nm
            }
        }

        impl ::core::convert::From<$nm> for $f_ty {
            fn from(value: $nm) -> Self {
                value.$f_nm
            }
        }
    };
}

pub(super) use impl_deref_into;
pub(super) use impl_opt_trait;
pub(super) use impl_parse;
pub(super) use opt;
pub(super) use parse_field;
