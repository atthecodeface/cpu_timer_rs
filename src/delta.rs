//a Delta
//ti Delta
/// A private type that is returned by get_timer, and which can be
/// used for all the timer calculations
///
/// This is used to abstract the internals from the public API
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Delta(u64);

//ip Delta
impl Delta {
    //cp add
    /// Accmulate another delta into this value
    #[inline(always)]
    #[must_use]
    pub fn add(self, other: Self) -> Self {
        self.0.wrapping_add(other.0).into()
    }

    //cp sat_add
    /// Accmulate another delta into this value
    #[inline(always)]
    #[must_use]
    pub fn sat_add(self, other: Self) -> Self {
        self.0.saturating_add(other.0).into()
    }
}

//ip From<()> for Delta
impl From<()> for Delta {
    #[inline(always)]
    fn from(_t: ()) -> Self {
        Self(0)
    }
}

//ip From<Delta> for ()
impl From<Delta> for () {
    #[inline(always)]
    fn from(_v: Delta) -> Self {}
}

//ip From<uN> for Delta, and the reverse
macro_rules! to_from_value {
    {$t:ty} => {
        impl From<Delta> for $t {
            #[inline(always)]
            fn from(v: Delta) -> Self {
                v.0 as $t
            }
        }
        impl From<$t> for Delta {
            #[inline(always)]
            fn from(t: $t) -> Self {
                Delta(t as u64)
            }
        }
    }
}
to_from_value!(u8);
to_from_value!(u16);
to_from_value!(u32);
to_from_value!(u64);
to_from_value!(u128);
to_from_value!(usize);
