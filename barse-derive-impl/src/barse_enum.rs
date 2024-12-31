//! Implementation of derive for enums.
#![allow(dead_code, unused)]

use ::proc_macro2::TokenStream;
use ::syn::ItemEnum;

use crate::{opt, result_aggregate::ResAggr};

opt::opt_parser! {
    /// Enum configuration.
    EnumConfig {
        /// Replace where clause of barse impl.
        where_clause: opt::CustomWhere,

        /// Replace path to barse crate/module.
        barse_path: opt::BarsePath,

        /// Set a ReadWith and WriteWith value.
        with: opt::With,

        /// Set a ReadWith value.
        read_with: opt::ReadWith,

        /// Set a WriteWith value.
        write_with: opt::WriteWith,

        /// Set prefix prepended to field names in expressions. (_ by default for tuple structs).
        field_prefix: opt::FieldPrefix,

        /// Set a fixed endian in use by struct (fields may overwrite to another fixed endian).
        endian: opt::Endian,

        /// Enum is read/written as discriminant then variant.
        discriminant: opt::EnumDiscriminant,
    },

    /// Enum variant configuration.
    VariantConfig {
        /// Set prefix prepended to field names in expressions. (_ by default for tuple structs).
        field_prefix: opt::FieldPrefix,

        /// Set a fixed endian in use by struct (fields may overwrite to another fixed endian).
        endian: opt::Endian,

        /// Condition for this variant.
        variant_if: opt::VariantIf,

        /// Discriminant of this variant.
        discriminant: opt::VariantDiscriminant,

        /// Field is ignored.
        ignore: opt::IgnoreField,

        /// Given expression is used instead of '()'.
        with: opt::FieldWith,

        /// Given expression is used instead of '()'.
        read_with: opt::FieldReadWith,

        /// Given expression is used instead of '()'.
        write_with: opt::FieldWriteWith,

        /// Bytes.
        bytes: opt::Bytes,

        /// Read bytes.
        read_bytes: opt::ReadBytes,

        /// Write bytes.
        write_bytes: opt::WriteBytes,

        /// Read using provided impl.
        read_as: opt::ReadAs,

        /// Write using provided impl.
        write_as: opt::WriteAs,

        /// Read/Write using provided impl.
        barse_as: opt::BarseAs,
    },
}

/// Derive barse for an enum.
///
/// # Errors
/// Should derive not be possible.
pub fn derive_barse_enum(item: ItemEnum) -> Result<TokenStream, ::syn::Error> {
    let EnumConfig {
        where_clause,
        barse_path,
        with,
        read_with,
        write_with,
        field_prefix,
        endian,
        discriminant,
    } = EnumConfig::default().parse_attrs(&item.attrs)?;

    let mut aggr = ResAggr::<()>::new();

    aggr.conflict(&read_with, &with)
        .conflict(&write_with, &with);

    todo!()
}
