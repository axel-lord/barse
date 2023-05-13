# Barse
A library for parsing binary data with procedural macros for easy composition.

## barse
Library to parse binary files.

## barse-derive
Proc macro crate for my binary file parsing libray barse.

## Planned
- [ ] Custom Errors when deriving
- [ ] Shorthand attributes for #[barse(as = "SizedVec<T, Q>")] and #[barse(as = "FlagConditional<T, C>")]
- [ ] FromByteReaderWith parsing with additional data specified by associated type
- [ ] Deriving FromByteReaderWith using attributes
- [ ] Dynamic dispatch based ByteRead wrapper 
- [ ] Remove flags

## Licensing
This project is licensed under MIT or Apache-2.0 use whichever you fancy.
