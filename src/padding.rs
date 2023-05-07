#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Padding<const SIZE: usize>;

impl<const SIZE: usize> std::fmt::Debug for Padding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Padding({})", SIZE * 8)
    }
}
