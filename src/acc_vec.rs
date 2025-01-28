//a Imports
use crate::{BaseTimer, TArch, TDesc, TraceCount, TraceValue};

//a AccVec
//tp AccVec
/// An [AccVec] can be used to accumulate the times taken to execute
/// different branches of code, from a common start point. Each branch
/// is allocated a different index into the AccVec. It can also count
/// the entries.
///
/// The AccVec is generic on whether to use the CPU-specific
/// architcture timer implementation, the value to accumulate times in
/// (e.g. u64), the value to use to count occurrences (e.g. u32), and
/// the number of trace points in the vec
///
/// The 'start' method is called first; when a branch completed it
/// invokes the 'acc_n' method with its index, and the delta time since
/// the start is added to that start's accumulator.
///
/// Invoking the 'acc_n' method does not update the 'start' time, and it
/// is quite sensible to issue multiple 'acc' invocations (with
/// different index values) for a given 'start' invocation.
///
/// The 'acc_n_restart' method, though, performs the same accumulation
/// and it *does* update the 'start' time; this can be used to
/// accumulate elapsed time between stages.
///
/// An AccVec can be generated for any N, for T in u8, u16, u32, u64, u128 and usize
#[derive(Debug)]
pub struct AccVec<const S: bool, T: TraceValue, C: TraceCount, const N: usize>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    accs: [T; N],
    cnts: [C; N],
}

//ip Default for AccVec
impl<const S: bool, T, C, const N: usize> std::default::Default for AccVec<S, T, C, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
    [T; N]: Default,
    [C; N]: Default,
{
    fn default() -> Self {
        let base = BaseTimer::default();
        let accs = <[T; N]>::default();
        let cnts = <[C; N]>::default();
        Self { base, accs, cnts }
    }
}

//ip AccVec
impl<const S: bool, T, C, const N: usize> AccVec<S, T, C, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        unsafe { *self = std::mem::zeroed() };
    }

    //mp start
    /// Start the underlying timer
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp acc_n
    /// Add the ticks on exit to a specific region
    #[inline(always)]
    pub fn acc_n(&mut self, index: usize) {
        if index < N {
            let delta: u64 = self.base.elapsed();
            self.accs[index] = self.accs[index].sat_add(delta);
            self.cnts[index].sat_inc();
        }
    }

    //mp acc_n_restart
    /// Add the ticks on exit to a specific region
    #[inline(always)]
    pub fn acc_n_restart(&mut self, index: usize) {
        if index < N {
            let delta = self.base.elapsed_delta_and_update();
            let acc = delta.add(self.accs[index].into());
            self.accs[index] = acc.into();
            self.cnts[index].sat_inc();
        }
    }

    //mp accs
    /// Return the accumulated values
    pub fn accs(&self) -> &[T; N] {
        &self.accs
    }

    //mp cnts
    /// Return the accumulated counts
    pub fn cnts(&self) -> &[C; N] {
        &self.cnts
    }
}
