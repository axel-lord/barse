/// Enum describing endianess.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Endian {
    /// Little endian, 258 as a u16 in hex is 02 01 (2 + 256).
    Little,
    /// Big endian, 258 as a u16 in hex is 01 02 (256 + 2).
    Big,
}
