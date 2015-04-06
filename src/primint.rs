/// A trait for converting the endianness of primitive integers.
pub trait PrimInt: Sized + Copy {
    /// Convert an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(Self) -> Self;

    /// Convert an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(Self) -> Self;

    /// Convert `self` into little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;

    /// Convert `self` into big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;
}

macro_rules! primint_impl {
    ($($T:ty)*) => ($(
        impl PrimInt for $T {
            fn from_be(x: Self) -> Self { <$T>::from_be(x) }
            fn from_le(x: Self) -> Self { <$T>::from_le(x) }
            fn to_le(self) -> Self { <$T>::to_le(self) }
            fn to_be(self) -> Self { <$T>::to_be(self) }
        }
    )*)
}

primint_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

