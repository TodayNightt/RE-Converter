pub mod extensions;

pub mod view_helper;

pub trait EnumToArray<const N: usize> {
    type T;

    fn all() -> [Self::T; N];
}
