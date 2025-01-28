//a Documentation
//! This library provides architecture/implementation specific CPU
//! counters for high precision timing, backed up by a std::time
//! implementation where an architecture has no explicit CPU support
//!
//! The timers are really CPU tick counters, and so are not resilient
//! to threads being descheduled or being moved between CPU cores; the
//! library is designed for precise timing of short code sections
//! where the constraints are understood. Furthermore, the timer
//! values are thus not in seconds but in other arbitrary units -
//! useful for comparing execution of different parts of code, but
//! requiring another mechanism to determine the mapping from ticks to
//! seconds
//!
//! # Precision
//!
//! For some architectures a real CPU ASM instruction is used to get
//! the tick count. For x86_64 this returns (in an unvirtualized
//! world) the real CPU tick counter, with a fine precision. For
//! Aarch64 on MacOs this is no better than using std::time, and has a
//! precision of about 40 ticks. However, the asm implementation has a
//! lower overhead on Aarch64 on MacOs, so it is still worth using.
//!
//! The library does not attempt to take into account any overheads of
//! using the timers; that is for the user. Normally the overheads
//! will be small compared to the times being measured.
//!
//! # CPU support (for non-experimental Rustc target architectures)
//!
//! For the stable Rustc-supported architectures, CPU implementations
//! are provided for:
//!
//! - [ ] x86    
//! - [x] x86_64
//! - [x] aarch64
//! - [ ] wasm32
//!
//! Nonsupported architectures resort to the [std::time::Instant]
//! 'now' method instead (which can be perfectly adequate)
//!
//! # Types
//!
//! The types in the library are all generic on *UseAsm* whether the CPU
//! architecture specific version (if provided) of the timer should be
//! used, or if std::time should be used instead. For architectures
//! without a CPU implementation, the std::time version is used
//! whatever the value of the generic.
//!
//! ## Timer
//!
//! The base type provided by this library is [Timer], which simply
//! has a `start` method and an `elapsed` method, to delver the ticks
//! (as a u64) since the last `state. It uses a generic *UseAsm* bool;
//! if true then the CPU specific timer implementation is used,
//! otherwise it uses std::time.
//!
//! There is an additional method `elapsed_and_update`, which restarts
//! the timer as well as returning the elapsed time, in a single
//! operation.
//!
//! ## DeltaTimer
//!
//! The [DeltaTimer] allows for *recording* the delta in CPU ticks
//! between the entry to a region of code and the exit from it. It
//! uses a generic *UseAsm* bool.
//!
//! ```
//! # use cpu_timer::DeltaTimer;
//! let mut t = DeltaTimer::<true>::default();
//! t.start();
//! // do something! - timed using CPU ticks
//! t.stop();
//! println!("That took {} cpu 'ticks'", t.value());
//!
//! let mut t = DeltaTimer::<false>::default();
//! t.start();
//! // do something! - timed using std::time
//! t.stop();
//! println!("That took {} nanoseconds", t.value());
//! ```
//!
//! ## AccTimer
//!
//! Frequently one will want to repeatedly time a piece of code, to
//! attain an average, or to just accumulate the time taken in some
//! code whenever it is called to determine if it is a 'hotspot'. The
//! [AccTimer] accumulates the time delta between start and stop.
//!
//! ```
//! # use cpu_timer::AccTimer;
//! let mut t = AccTimer::<true>::default();
//! for i in 0..100 {
//!     t.start();
//!     // do something!
//!     t.stop();
//!     println!("Iteration {i} took {} ticks", t.last_delta());
//! }
//! println!("That took an average of {} ticks", t.acc_value()/100);
//! ```
//!
//! ## AccArray
//!
//! An [AccArray] is used to accumulate timer values, storing not just
//! the times but also (optionally) the number of occurrences.
//!
//! It is used as `AccVec<A, T, C, N>`; A is a bool; T the time accumulator type; C the counter type; N the number of accumulators.
//!
//!  * A is true if the CPU-specific timer should be used, false if
//!    std::time should be used
//!
//!  * T is the type used for accumulating time deltas (u8, u16, u32,
//!    u64, u128, usize, f32, f64, or () to not accumulate times)
//!
//!  * C is the type used for counting occurrences (u8, u16, u32,
//!     u64, u128, usize, f32, f64, or () to not count occurrences)
//!
//!  * N can be any usize; the space for the occurrence accumulators
//!    and counters is statically held within the type, so *N* effects
//!    the size of the AccArray
//!
//! The array can be cleared - clearing the accumulators.
//!
//! A use is to first invoke `start` and then later `acc_n` with a
//! specific index which identifies the code just executed; the time
//! elapsed since the last start is accumulated and the occurrences
//! counted.
//!
//! ## AccVec
//!
//! An [AccVec] is a less static version of [AccArray], using an array
//! backed by a `Vec`. It has the same methods, and additional `push`
//! related methods.
//!
//! ## Trace
//!
//! The [Trace] type supports tracing the execution path through some
//! logic, getting deltas along the way
//!
//! ```
//! # use cpu_timer::Trace;
//! let mut t = Trace::<true, u32, 3>::default();
//! t.start();
//!   // do something!
//! t.next();
//!   // do something else!
//! t.next();
//!   // do something else!
//! t.next();
//! println!("The three steps took {:?} ticks", t.trace());
//! ```
//!
//! The trace will have three entries, which are the delta times for
//! the three operations.
//!
//! ## AccTrace
//!
//! The [AccTrace] accumulates a number of iterations of a Trace;
//!
//! ```
//! # use cpu_timer::AccTrace;
//! struct MyThing {
//!     // things ...
//!     /// For timing (perhaps only if #[cfg(debug_assertions)] )
//!     acc: AccTrace::<true, u32,4>,
//! }
//!
//! impl MyThing {
//!     fn do_something_complex(&mut self) {
//!         self.acc.start();
//!         // .. do first complex thing
//!         self.acc.next();
//!         // .. do second complex thing
//!         self.acc.next();
//!         // .. do third complex thing
//!         self.acc.next();
//!         // .. do fourth complex thing
//!         self.acc.next();
//!         self.acc.acc();
//!     }
//! }
//!
//! let mut t = MyThing { // ..
//!     acc: AccTrace::<true, u32, 4>::default()
//! };
//! for _ in 0..100 {
//!     t.do_something_complex();
//! }
//! println!("After 100 iterations the accumulated times for the four steps is {:?} ticks", t.acc.acc_trace());
//! t.acc.clear();
//! // ready to be complex all again
//! ```
//!
//! The trace will have four entries, which are the accumulated delta times for
//! the four complex things.
//!
//! # OS-specific notes
//!
//! These outputs are generated from tests/cpu_timer.rs, test_timer_values
//!
//! The tables will have a rough granularity of the precision of the
//! tick counter. Average time taken is calculated using the fastest
//! 95% of 10,000 calls, as beyond that the outliers should be ignored.
//!
//! ## MacOs aarch64 (MacBook Pro M4 Max Os15.1 rustc 1.84
//!
//! The granularity of the clock appears to be 41 or 42 ticks, and the
//! asm implementation seems to match the std time implementation for this precision.
//!
//! For asm, the average time taken for a call is 3 ticks in release, 9 ticks in debug
//!
//! For std::time, the average time taken for a call is 8 ticks in
//! release, 17 ticks in debug. So clearly there is an overhead for
//! using std::time
//!
//! | %age | arch release |   arch debug | std debug    | std release  |
//! |------|--------------|--------------|--------------|--------------|
//! | 10   |      0       |       0      |       41     |         0    |
//! | 25   |      0       |       0      |       42     |         0    |
//! | 50   |      0       |       0      |       42     |         0    |
//! | 75   |      0       |      41      |       83     |        41    |
//! | 90   |     42       |      41      |       83     |        41    |
//! | 95   |     42       |      41      |       83     |        41    |
//! | 99   |     42       |      42      |       84     |        42    |
//! | 100  |  27084       |    2498      |     2166     |      1125    |
//!
//! ### MacOs aarch64 std::time release
//!
//! Percentile distribution
//! 56, 0
//! 71, 41
//! 99, 42
//! 100, 1125
//!
//! average of up to 95 8
//!
//! ### MacOs aarch64 std::time debug
//!
//! Percentile distribution
//! 6, 41
//! 18, 42
//! 71, 83
//! 98, 84
//! 99, 125
//! 100, 2166
//!
//! average of up to 95 17
//!
//! ### MacOs aarch64 debug
//!
//! Percentile distribution
//! 52, 0
//! 68, 41
//! 99, 42
//! 100, 2958
//!
//! average of up to 95 9
//!
//! ### MacOs aarch64 release
//!
//! Percentile distribution
//! 77, 0
//! 85, 41
//! 99, 42
//! 100, 1500
//!
//! average of up to 95 3
//!
//! ## MacOs x86_64
//!
//! MacBook Pro 2018 Os 15.0 rustc 1.84 2.2GHz i7
//!
//! The granularity of the clock appears to be 2 ticks, and the
//! asm implementation is better than using the std::time implementation
//!
//! The average time taken for a call is 15 ticks in release, 78 (but
//! sometimes 66!) ticks in debug
//!
//! | %age | arch release |   arch debug | std debug    | std release  |
//! |------|--------------|--------------|--------------|--------------|
//! | 10   |     12       |      62      |       72     |        38    |
//! | 25   |     12       |      64      |       74     |        38    |
//! | 50   |     12       |      64      |       79     |        39    |
//! | 75   |     14       |      66      |       81     |        39    |
//! | 90   |     14       |      68      |       83     |        39    |
//! | 95   |     14       |      70      |       83     |        40    |
//! | 99   |     16       |      82      |      132     |        41    |
//! | 100  |  42918       |   65262      |    17101     |     24560    |
//!
//!
//! ### MacOs x86_64 release
//!
//! Percentile distribution
//! 5, 12
//! 73, 14
//! 99, 16
//! 100, 42918
//!
//! average of up to 95 15
//!
//! ### MacOs x86_64 debug
//!
//! Percentile distribution
//! 4, 62
//! 22, 64
//! 55, 66
//! 81, 68
//! 92, 70
//! 96, 72
//! 98, 74
//! 99, 82
//! 100, 65262    
//!
//! average of up to 95 78
//!
//! ### MacOs std::time debug
//!
//! Percentile distribution
//! 1, 70
//! 4, 71
//! 9, 72
//! 15, 73
//! 22, 74
//! 28, 75
//! 34, 76
//! 40, 77
//! 45, 78
//! 50, 79
//! 56, 80
//! 66, 81
//! 79, 82
//! 90, 83
//! 96, 84
//! 98, 85
//! 99, 132
//! 100, 17101
//!
//! ### MacOs std::time release
//!
//! Percentile distribution
//! 3, 37
//! 44, 38
//! 92, 39
//! 96, 40
//! 99, 41
//! 100, 24560

//a Imports
mod delta;
mod traits;

mod acc_vec;
mod arch;
mod base;
mod timers;
mod trace;

//a Export to the crate, but not outside
pub(crate) use base::BaseTimer;
pub(crate) use delta::Delta;
pub(crate) use traits::private;

//a Export to outside
pub use acc_vec::{AccArray, AccVec};
pub use arch::TDesc;
pub use timers::{AccTimer, DeltaTimer, Timer};
pub use trace::{AccTrace, Trace};
pub use traits::{TArch, TraceCount, TraceValue};
