//a Private module to allow sealing of traits
//mi private
/// This module is private to seal the TArch trait, which must be
/// implemented here only.
pub(crate) mod private {
    //tp Value
    pub(crate) trait Value: std::fmt::Debug + Default + Copy {
        fn since(self, last: Self) -> crate::Delta;
        fn since_and_update(&mut self, now: Self) -> crate::Delta;
    }
    impl Value for u64 {
        fn since(self, last: Self) -> crate::Delta {
            self.wrapping_sub(last).into()
        }
        fn since_and_update(&mut self, now: Self) -> crate::Delta {
            let delta = now.wrapping_sub(*self);
            *self = now;
            delta.into()
        }
    }

    //tp ArchDesc
    pub(crate) trait ArchDesc: Default {
        /// Value returned by the timer
        ///
        /// This is stored within timers but is not visible to users
        type Value: Value;

        //fp get_timer
        /// Get the current value of the timer
        fn get_timer() -> Self::Value;
    }

    //tt TraceValue
    pub(crate) trait TraceValue:
        Default + Copy + From<crate::Delta> + Into<crate::Delta>
    {
        fn sat_add(self, other: u64) -> Self;
    }
}

//a TraceCount
//tt TraceCount
/// A value that can be stored in a Trace; this is implemented for u8,
/// u16, u32, u64 and usize
pub trait TraceCount: Default + Copy {
    fn sat_inc(&mut self);
    fn as_usize(self) -> usize;
}

//ip TraceCount for ()
impl TraceCount for () {
    fn sat_inc(&mut self) {}
    fn as_usize(self) -> usize {
        0
    }
}

//ip TraceCount for u8/u16/u32/u64/u128/usize
macro_rules! trace_count {
    {$t:ty} => {
        impl TraceCount for $t {
            #[inline(always)]
            fn sat_inc(&mut self) {
                if *self != Self::MAX {*self = self.wrapping_add(1);}
            }
            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }
        }
    }
}
macro_rules! trace_float_count {
    {$t:ty} => {
        impl TraceCount for $t {
            #[inline(always)]
            fn sat_inc(&mut self) {
                *self += 1.0;
            }
            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }
        }
    }
}
trace_count!(u8);
trace_count!(u16);
trace_count!(u32);
trace_count!(u64);
trace_count!(u128);
trace_count!(usize);
trace_float_count!(f32);
trace_float_count!(f64);

//a TraceValue
//tt TraceValue
/// A value that can be stored in a Trace; this is implemented for u8,
/// u16, u32, u64 and usize
// Note that the type 'crate::Delta' is private
#[allow(private_bounds)]
pub trait TraceValue: private::TraceValue {}

//ip TraceValue for T: private::TraceValue
impl<T> TraceValue for T where T: private::TraceValue {}

//ip private::TraceValue for ()
impl private::TraceValue for () {
    fn sat_add(self, _other: u64) -> Self {}
}

//ip TraceValue for u8/u16/u32/u64/u128/usize
macro_rules! trace_value {
    {$t:ty} => {
        impl private::TraceValue for $t {
            fn sat_add(self, other:u64) -> Self {
                self.saturating_add(other as $t)
            }
        }
    }
}
macro_rules! trace_float_value {
    {$t:ty} => {
        impl private::TraceValue for $t {
            fn sat_add(self, other:u64) -> Self {
                self + (other as $t)
            }
        }
    }
}
trace_value!(u8);
trace_value!(u16);
trace_value!(u32);
trace_value!(u64);
trace_value!(u128);
trace_value!(usize);
trace_float_value!(f32);
trace_float_value!(f64);

//tt TArch
/// Trait provided for architecture-specific timers
///
/// This is supported by a single assembler timer and a standard
/// (std::time) timer
#[allow(private_bounds)]
pub trait TArch: private::ArchDesc {}

//ip TArch for T: private::ArchDesc
impl<T> TArch for T where T: private::ArchDesc {}
