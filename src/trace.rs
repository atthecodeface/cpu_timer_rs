//a Imports
use crate::{BaseTimer, Delta, TArch, TDesc, TraceValue};

//a Trace
//tp Trace
/// A [Trace] can be used to trace the execution of some code, from an
/// start point through a series of intermediate points. The delta for
/// each step can be recorded.
///
/// The 'start' method is called first; at each completed step the
/// 'next' method is called. At the end (after no more than 'N'
/// steps!) the deltas for each step of the trace can be recovered
/// with the 'trace' method.
///
/// A Trace can be generated for any N, for T in u8, u16, u32, u64, u128 and usize
#[derive(Debug)]
pub struct Trace<const S: bool, T: TraceValue, const N: usize>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    index: usize,
    trace: [T; N],
}

//ip Default for Trace
impl<const S: bool, T, const N: usize> std::default::Default for Trace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    [T; N]: Default,
{
    fn default() -> Self {
        let base = BaseTimer::default();
        let index = 0;
        let trace = <[T; N]>::default();
        Self { base, index, trace }
    }
}

//ip Trace
impl<const S: bool, T, const N: usize> Trace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and trace
    pub fn clear(&mut self) {
        unsafe { *self = std::mem::zeroed() };
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    ///
    /// Up to *N* invocations of 'next' afterwards will store
    /// individual deltas in the trace
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
        self.index = 0;
    }

    //mp next
    /// Calculate the delta since the last 'start' or 'next', and
    /// store it in the next trace slot
    ///
    /// If this is invoked more than *N* times after a start then no
    /// work is done, as there is no space to store the time in the
    /// internal trace
    #[inline(always)]
    pub fn next(&mut self) {
        if self.index < N {
            let delta = self.base.elapsed_delta_and_update();
            self.trace[self.index] = delta.into();
            self.index += 1;
        }
    }

    //mp trace
    /// Return the current trace
    pub fn trace(&self) -> &[T; N] {
        &self.trace
    }
}

//a AccTrace
//tp AccTrace
#[derive(Debug)]
pub struct AccTrace<const S: bool, T: TraceValue, const N: usize>
where
    TDesc<S>: TArch,
{
    trace: Trace<S, T, N>,
    acc: [T; N],
}

//ip Default for AccTrace
impl<const S: bool, T, const N: usize> std::default::Default for AccTrace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    [T; N]: Default,
{
    fn default() -> Self {
        let trace = Trace::default();
        let acc = <[T; N]>::default();
        Self { trace, acc }
    }
}

//ip AccTrace
impl<const S: bool, T, const N: usize> AccTrace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        self.trace.clear();
        unsafe { self.acc = std::mem::zeroed() };
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    #[inline(always)]
    pub fn start(&mut self) {
        self.trace.start();
    }

    //mp next
    /// Record the ticks on exit from a region-to-time
    #[inline(always)]
    pub fn next(&mut self) {
        self.trace.next();
    }

    //mp acc
    /// Accumulate the current trace into the accumulated trace
    pub fn acc(&mut self) {
        for i in 0..N {
            let v: Delta = self.acc[i].into();
            let v = v.add(self.trace.trace[i].into());
            self.acc[i] = v.into();
        }
    }

    //mp last_trace
    /// Return the current trace
    pub fn last_trace(&self) -> &[T; N] {
        self.trace.trace()
    }

    //mp acc_trace
    /// Return the accumulated trace
    pub fn acc_trace(&self) -> &[T; N] {
        &self.acc
    }
}
