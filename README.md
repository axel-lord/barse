# Barse
A library for parsing binary data with procedural macros for easy composition.

## barse
Library to parse binary files.

## barse-derive
Proc macro crate for my binary file parsing libray barse.

## Planned
- [x] Custom Errors when deriving
- [ ] ~~Shorthand attributes for #[barse(as = "SizedVec<T, Q>")] and #[barse(as = "FlagConditional<T, C>")]~~
- [x] FromByteReaderWith parsing with additional data specified by associated type
- [x] Deriving FromByteReaderWith using attributes
- [x] ~~Dynamic dispatch based ByteRead wrapper~~
- [x] Remove flags
- [x] Replace Cursor impl with custom type using usize
- [x] Implement FromByteReaderWith for u8 slices
- [ ] Replace Slice error with more malleable one that can indicate reason
- [ ] Add assertion error
- [ ] Add assertions to macro for both types and fields
- [x] Replace const endian reader with little big variants
- [x] Add specialized way of getting a mut reference to a reader and remove the auto mut ref impl
- [ ] Add with impls for endianess
- [ ] Replace ByteRead trait with struct

## Licensing
This project is licensed under MIT or Apache-2.0 use whichever you fancy.
