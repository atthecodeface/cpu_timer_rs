//a Imports
use crate::{BaseTimer, Delta, TArch, TDesc};

//a Timer
//tp Timer
/// A basic timer that supports start, elapsed, and elapsed_and_update
#[derive(Default, Debug, Clone, Copy)]
pub struct Timer<const S: bool>
where
    BaseTimer<S>: Default,
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
}

impl<const S: bool> Timer<S>
where
    BaseTimer<S>: Default,
    TDesc<S>: TArch,
{
    //mp start
    /// Record the time now
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start()
    }

    //ap elapsed
    /// Return the time elapsed as a u64
    #[inline(always)]
    pub fn elapsed(&self) -> u64 {
        self.base.elapsed()
    }

    //mp elapsed_and_update
    /// Return the time elapsed as a u64, and update the timer
    #[inline(always)]
    pub fn elapsed_and_update(&mut self) -> u64 {
        self.base.elapsed_and_update()
    }
}

//a DeltaTimer
//tp DeltaTimer
/// A timer that uses the underlying CPU clock ticks to generate
/// precise timings for short-term execution
///
/// This should *not* be expected to be correct in all cases; if a
/// thread sleeps or is interrupted, for example by the kernel, for
/// any reason, then the CPU timer value may not be useful; if the
/// thread migrates to a different CPU core it may become invalid; etc
///
/// The usage model is to capture a 'start' time and a 'stop' time;
/// the *value* method can then be used to retrieve the CPU ticks
/// between the start and stop
///
/// ```
/// # use cpu_timer::DeltaTimer;
/// let mut t = DeltaTimer::<true>::default();
/// t.start();
/// // do something!
/// t.stop();
/// println!("That took {} ticks", t.value());
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct DeltaTimer<const S: bool>
where
    BaseTimer<S>: Default,
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    delta: Delta,
}

//ip DeltaTimer
impl<const S: bool> DeltaTimer<S>
where
    TDesc<S>: TArch,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp start
    /// Record the ticks at the start of the timer
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp delta
    /// Return (without updating) the delta since start
    #[inline(always)]
    pub fn delta(&mut self) -> u64 {
        self.base.elapsed_delta().into()
    }

    //mp stop
    /// Record the delta time since the last start
    #[inline(always)]
    pub fn stop(&mut self) {
        self.delta = self.base.elapsed_delta();
    }

    //mp value
    /// Return the delta time in ticks
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.delta.into()
    }
}

//a AccTimer
//tp AccTimer
/// An timer that accumulates the value for multiple timer start-stops
///
#[derive(Default, Debug, Clone, Copy)]
pub struct AccTimer<const S: bool>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    delta: Delta,
    acc: Delta,
}

//ip AccTimer
impl<const S: bool> AccTimer<S>
where
    TDesc<S>: TArch,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp stop
    /// Record the ticks on stop from a region-to-time, and update the accimulator
    #[inline(always)]
    pub fn stop(&mut self) {
        self.delta = self.base.elapsed_delta();
        self.acc = self.acc.sat_add(self.delta);
    }

    //mp last_delta
    /// Return the last ticks between start and stop
    #[inline(always)]
    pub fn last_delta(&self) -> u64 {
        self.delta.into()
    }

    //mp acc_value
    /// Read the accumulator value
    #[inline(always)]
    pub fn acc_value(&self) -> u64 {
        self.acc.into()
    }
}
