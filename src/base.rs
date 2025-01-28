//a Imports
use crate::private;
use crate::traits::private::Value;
use crate::{Delta, TArch, TDesc};

//a BaseTimer
//tp BaseTimer
/// A basic timer that just contains the timer value
///
/// This is used internally for all the timer implementations
#[derive(Default, Debug)]
pub struct BaseTimer<const S: bool>
where
    TDesc<S>: TArch,
{
    start: <TDesc<S> as private::ArchDesc>::Value,
}

//ip BaseTimer
impl<const S: bool> BaseTimer<S>
where
    TDesc<S>: TArch,
{
    //mi now
    #[inline(always)]
    fn now() -> <TDesc<S> as private::ArchDesc>::Value {
        <TDesc<S> as private::ArchDesc>::get_timer()
    }

    //mp start
    /// Record the time now
    #[inline(always)]
    pub fn start(&mut self) {
        self.start = Self::now();
    }

    //mp elapsed_delta
    /// Return the Delta between now and self.start
    #[inline(always)]
    pub(crate) fn elapsed_delta(&self) -> Delta {
        Self::now().since(self.start)
    }

    //mp elapsed_delta_and_update
    /// Record the delta time since the last start
    #[inline(always)]
    pub(crate) fn elapsed_delta_and_update(&mut self) -> Delta {
        self.start.since_and_update(Self::now())
    }

    //ap elapsed
    /// Return the time elapsed as a u64
    #[inline(always)]
    pub fn elapsed(&self) -> u64 {
        self.elapsed_delta().into()
    }

    //mp elapsed_and_update
    /// Return the time elapsed as a u64, and update the timer
    #[inline(always)]
    pub fn elapsed_and_update(&mut self) -> u64 {
        self.elapsed_delta_and_update().into()
    }
}
