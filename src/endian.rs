/// An Enum for handling byte order.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Endian {
    Big,
    Little,
}

#[cfg(target_endian = "big")]
const NATIVE_ENDIAN: Endian = Endian::Big;
#[cfg(target_endian = "little")]
const NATIVE_ENDIAN: Endian = Endian::Little;

impl Endian {
    /// The byte order of the current target.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_endian::Endian;
    /// const NE: Endian = Endian::NATIVE;
    /// ```
    pub const NATIVE: Self = NATIVE_ENDIAN;

    /// Tests for `self` and `other` values to be equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_endian::Endian;
    /// const BE: Endian = Endian::Big;
    /// const IS_EQUAL: bool = BE.is(Endian::Big);
    /// assert!(IS_EQUAL);
    /// ```
    ///
    /// You can also use `==` out Constant Expressions:
    ///
    /// ```
    /// use lib_endian::Endian;
    /// let be = Endian::Big;
    /// let is_equal = be == Endian::Big;
    /// assert!(is_equal);
    /// ```
    pub const fn is(self, other: Self) -> bool {
        self as u8 == other as u8
    }

    /// Returns a new `Endian` value which is different from `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_endian::Endian;
    /// const BE: Endian = Endian::Little.reverse();
    /// const IS_EQUAL: bool = BE.reverse().is(Endian::Little);
    /// assert!(IS_EQUAL);
    /// ```
    ///
    /// You can also use `!` out Constant Expressions:
    ///
    /// ```
    /// use lib_endian::Endian;
    /// let be = !Endian::Little;
    /// let is_equal = !be == Endian::Little;
    /// assert!(is_equal);
    /// ```
    pub const fn reverse(self) -> Self {
        match self {
            Self::Big => Self::Little,
            Self::Little => Self::Big,
        }
    }

    /// Returns `Endian::native()` if `reverse` is true,  
    /// otherwise returns `Endian::native().reverse()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_endian::Endian;
    ///
    /// const REVERSE: bool = true;
    ///
    /// const NE_REV: Endian = Endian::native_or_reversed(REVERSE);
    /// assert_eq!(NE_REV, Endian::NATIVE.reverse());
    ///
    /// const NE: Endian = Endian::native_or_reversed(!REVERSE);
    /// assert_eq!(NE, Endian::NATIVE);
    /// ```
    pub const fn native_or_reversed(reverse: bool) -> Self {
        if reverse {
            Self::NATIVE.reverse()
        } else {
            Self::NATIVE
        }
    }
}

impl std::ops::Not for Endian {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.reverse()
    }
}
