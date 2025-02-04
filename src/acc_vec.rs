//a Imports
use crate::{BaseTimer, TArch, TDesc, TraceCount, TraceValue};

//a AccArray
//tp AccArray
/// An [AccArray] can be used to accumulate the times taken to execute
/// different branches of code, from a common start point. Each branch
/// is allocated a different index into the AccArray. It can also count
/// the entries.
///
/// The AccArray is generic on whether to use the CPU-specific
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
/// An AccArray can be generated for any N, for an accumulator value
/// of (), u8, u16, u32, u64, u128 and usize, and for a counter value
/// of (), u8, u16, u32, u64, usize. If a value of () is used then the
/// count or delta accumulator are effectively always 0.
#[derive(Debug, Clone, Copy)]
pub struct AccArray<const S: bool, T: TraceValue, C: TraceCount, const N: usize>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    accs: [T; N],
    cnts: [C; N],
}

//ip Default for AccArray
impl<const S: bool, T, C, const N: usize> std::default::Default for AccArray<S, T, C, N>
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

//ip Display for AccArray
impl<const S: bool, T, C, const N: usize> std::fmt::Display for AccArray<S, T, C, N>
where
    TDesc<S>: TArch,
    T: TraceValue + std::fmt::Display + std::ops::Div<C>,
    <T as std::ops::Div<C>>::Output: std::fmt::Display,
    C: TraceCount + std::fmt::Display + PartialEq<C>,
    [T; N]: Default,
    [C; N]: Default,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let def_c: C = C::default();
        write! {fmt, "["}?;
        for i in 0..N {
            if i != 0 {
                write! {fmt, ", "}?;
            }
            if self.cnts[i] == def_c {
                write!(fmt, "({}, {}, -)", self.accs[i], self.cnts[i])?;
            } else {
                write!(
                    fmt,
                    "({}, {}, {})",
                    self.accs[i],
                    self.cnts[i],
                    self.accs[i] / self.cnts[i]
                )?;
            }
        }
        write! {fmt, "]"}
    }
}

//ip AccArray
impl<const S: bool, T, C, const N: usize> AccArray<S, T, C, N>
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
            let delta = self.base.elapsed_and_update();
            self.accs[index] = self.accs[index].sat_add(delta);
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

//a AccVec
//tp AccVec
/// An [AccVec] can be used to count and accumulate the times taken to
/// execute different branches of code, from a common start point. It
/// uses a `Vec` internally, and can be dynamically sized; it is
/// otherwise very similar in operation to an [AccArray].
///
/// The AccVec is generic on whether to use the CPU-specific
/// architcture timer implementation, the value to accumulate times in
/// (e.g. u64), the value to use to count occurrences (e.g. u32).
///
/// An AccVec can be created with a specific capacity - and memory is
/// allocated at this time for that capacity; when it is cleared, the
/// AccVec is emptied, but the contents are not freed.
///
/// In addition to the `acc_n` and `acc_n_restart` methods of the
/// AccArray, the AccVec adds `acc_push` and `acc_push_restart`, which
/// append to the current vector (and return the index used).
///
/// # Example accumulated times
///
/// An AccVec can be used to simply generate an arbitrarily long trace
/// of accumulated elapsed times (using `acc_push`):
///
/// ```
/// # use cpu_timer::AccVec;
/// let mut t = AccVec::<true, u32,()>::default();
/// t.start();
/// // do something!
/// t.acc_push();
/// // do something more!
/// t.acc_push();
/// // do something even more!
/// t.acc_push();
///
/// println!("The first step took {} ticks", t.acc_cnts()[0].0);
/// println!("The first and second steps took {} ticks", t.acc_cnts()[1].0);
/// println!("The first thru third steps took {} ticks", t.acc_cnts()[2].0);
///
/// ```
///
/// # Example individual times
///
/// An AccVec can be used to simply generate an arbitrarily long trace
/// of elapsed times (using `acc_push_restart`):
///
/// ```
/// # use cpu_timer::AccVec;
/// let mut t = AccVec::<true, u32, u8>::default();
/// t.start();
/// // do something!
/// t.acc_push_restart();
/// // do something more!
/// t.acc_push_restart();
/// // do something even more!
/// t.acc_push_restart();
///
/// println!("The first step took {} ticks", t.acc_cnts()[0].0);
/// println!("The second step took {} ticks", t.acc_cnts()[1].0);
/// println!("The third step took {} ticks", t.acc_cnts()[2].0);
///
/// for ac in t.acc_cnts() {
///     assert_eq!(ac.1, 1, "Each step occurred precisely once");
/// }
/// ```
///
/// # Example accumulate times over many executions
///
/// An AccVec can be used to accumulate an arbitrarily long trace
/// of elapsed times (using `acc_push_restart`):
///
/// ```
/// # use cpu_timer::AccVec;
/// let mut t = AccVec::<true, u32, u32>::default();
///
/// for i in 0..1000 {
///   t.start();
///   if i%3 == 0 {
///      // do something!
///   }
///   t.acc_push_restart();
///   if i%5 == 0 {
///      // do something!
///   }
///   t.acc_push_restart();
///   if i%2 == 0 {
///      // do something!
///      t.acc_push_restart();
///   }
/// }
///
/// let ac = t.all_acc_cnts();
/// assert_eq!(ac.len(), 3, "There are three accumulated values");
/// println!("The first step took an average of {} ticks", ac[0].0 / ac[0].1);
/// println!("The second step took an average of {} ticks", ac[1].0 / ac[1].1);
/// println!("The third step took an average of {} ticks", ac[2].0 / ac[2].1);
///
/// assert_eq!(ac[0].1, 1000, "First step occurred 1000 times");
/// assert_eq!(ac[1].1, 1000, "Second step occurred 1000 times");
/// assert_eq!(ac[2].1, 500, "*Last* step occurred 500 times");
///
/// assert_eq!(t.acc_cnts().len(), 2, "The last iteration did not do the `i%2` push!");
/// ```
/// # Example accumulate times for different operations over many executions
///
/// ```
/// # use cpu_timer::AccVec;
/// let mut t = AccVec::<true, u32, u32>::with_capacity(6);
///
/// for i in 0..1000 {
///   t.start();
///   for j in &["a", "", "bb", "ccc", "dddd", "eeeee", "bb", "ccc"] {
///      let k = j.chars().count();
///      t.acc_n_restart(k);
///   }
/// }
///
/// let ac = t.all_acc_cnts();
/// assert_eq!(ac.len(), 6, "There are five accumulated values, the given capacity");
/// for i in 0..6 {
///     let avg = ac[i].0 / ac[i].1;
///     println!("Counting {i} characters took an average of {} ticks", avg);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AccVec<const S: bool, T: TraceValue, C: TraceCount>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    index: usize,
    acc_cnts: Vec<(T, C)>,
}

//ip Default for AccVec
impl<const S: bool, T, C> std::default::Default for AccVec<S, T, C>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
{
    fn default() -> Self {
        let base = BaseTimer::default();
        let acc_cnts = vec![];
        let index = 0;
        Self {
            base,
            index,
            acc_cnts,
        }
    }
}

//ip Display for AccVec
impl<const S: bool, T, C> std::fmt::Display for AccVec<S, T, C>
where
    TDesc<S>: TArch,
    T: TraceValue + std::fmt::Display + std::ops::Div<C>,
    <T as std::ops::Div<C>>::Output: std::fmt::Display,
    C: TraceCount + std::fmt::Display + PartialEq<C>,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let def_c: C = C::default();
        write! {fmt, "["}?;
        for (i, ac) in self.acc_cnts.iter().enumerate() {
            if i != 0 {
                write! {fmt, ", "}?;
            }
            if ac.1 == def_c {
                write!(fmt, "({}, {}, -)", ac.0, ac.1)?;
            } else {
                write!(fmt, "({}, {}, {})", ac.0, ac.1, ac.0 / ac.1)?;
            }
        }
        write! {fmt, "]"}
    }
}

//ip AccVec
impl<const S: bool, T, C> AccVec<S, T, C>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
{
    //mp with_capacity
    /// Create a new AccVec of a certain size
    pub fn with_capacity(n: usize) -> Self {
        let mut s = Self::default();
        s.acc_cnts = vec![(T::default(), C::default()); n];
        s
    }

    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        self.index = 0;
        self.acc_cnts.clear();
    }

    //mp start
    /// Start the underlying timer
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
        self.index = 0;
    }

    //mp acc_n
    /// Calculate the ticks elapsed, and accumulate that in the specified
    /// entry in the store
    ///
    /// If the entry is beyond the capacity of the store, then this
    /// does nothing
    #[inline(always)]
    pub fn acc_n(&mut self, index: usize) {
        if let Some(ac) = self.acc_cnts.get_mut(index) {
            let delta: u64 = self.base.elapsed();
            ac.0 = ac.0.sat_add(delta);
            ac.1.sat_inc();
        }
    }

    //mp acc_n_restart
    /// Calculate the ticks elapsed and restart the timer, and
    /// accumulate that in the specified entry in the store
    ///
    /// If the entry is beyond the capacity of the store, then this
    /// just restarts the timer
    #[inline(always)]
    pub fn acc_n_restart(&mut self, index: usize) {
        if let Some(ac) = self.acc_cnts.get_mut(index) {
            let delta = self.base.elapsed_and_update();
            ac.0 = ac.0.sat_add(delta);
            ac.1.sat_inc();
        } else {
            self.base.start();
        }
    }

    //mp acc_push
    /// Calculate the ticks elapsed, and accumulate that in the next
    /// entry in the store
    ///
    /// This will extend the underlying store if required
    #[inline(always)]
    pub fn acc_push(&mut self) -> usize {
        let n = self.acc_cnts.len();
        if n > self.index {
            self.acc_n(self.index);
            let n = self.index;
            self.index += 1;
            n
        } else {
            let delta: u64 = self.base.elapsed();
            let delta = T::default().sat_add(delta);
            let mut cnt = C::default();
            cnt.sat_inc();
            self.acc_cnts.push((delta, cnt));
            self.index = n + 1;
            n
        }
    }

    //mp acc_push_restart
    /// Calculate the ticks elapsed and restart the timer, and
    /// accumulate that in the next entry in the store
    ///
    /// This will extend the underlying store if required
    #[inline(always)]
    pub fn acc_push_restart(&mut self) -> usize {
        let n = self.acc_cnts.len();
        if n > self.index {
            self.acc_n_restart(self.index);
            let n = self.index;
            self.index += 1;
            n
        } else {
            let delta = self.base.elapsed_and_update();
            let delta = T::default().sat_add(delta);
            let mut cnt = C::default();
            cnt.sat_inc();
            self.acc_cnts.push((delta, cnt));
            self.index = n + 1;
            n
        }
    }

    //mp all_acc_cnts
    /// Return *all* the accumulated values and counts
    ///
    /// This should be used if the `acc_n` methods are used, but not
    /// `acc_push`
    pub fn all_acc_cnts(&self) -> &[(T, C)] {
        &self.acc_cnts
    }

    //mp acc_cnts
    /// Return the accumulated values and counts, up to the last
    /// pushed
    ///
    /// The underlying store may have *more* values, perhaps from
    /// previous executions. This only returns up to the last value
    /// `pushed` since the last start.
    pub fn acc_cnts(&self) -> &[(T, C)] {
        &self.acc_cnts[0..self.index]
    }
}
