//a Imports
use crate::private;

//a Architecture-specific and standard get_timer functions
//tp TDesc
/// Marker type generic on a bool, which has the 'TArch' trait
/// implemented for it for (true) an assembler architecture specific
/// timer implementation, and (false) for a std::time implementation
#[derive(Default)]
pub struct TDesc<const B: bool>();

//tp Asm
/// Marker type for which TDesc is implemented for both true and false
#[derive(Default)]
pub struct Asm(());

//ip TArch for TDesc<true>
// Assembler specific implementation of a
// timer architecture
//
// If the architecture does not have an assembler implementation then
// this will actually be the std::time implementation
impl private::ArchDesc for TDesc<true> {
    type Value = arch::Value;
    #[inline(always)]
    fn get_timer() -> Self::Value {
        arch::get_timer()
    }
}

//ip TArch for TDesc<false>
// std::time implementation of a
// timer architecture
impl private::ArchDesc for TDesc<false> {
    type Value = arch_std::Value;
    #[inline(always)]
    fn get_timer() -> Self::Value {
        arch_std::get_timer()
    }
}

//a Architecture specific and standard timer implementation modules
//mi Standard architecture implementation of a timer
mod arch_std {
    #[derive(Debug, Clone, Copy)]
    pub struct Value(std::time::Instant);
    impl super::private::Value for Value {
        fn since(self, last: Self) -> crate::Delta {
            (self.0 - last.0).as_nanos().into()
        }
        fn since_and_update(&mut self, now: Self) -> crate::Delta {
            let delta = (now.0 - self.0).as_nanos().into();
            *self = now;
            delta
        }
    }
    impl std::default::Default for Value {
        fn default() -> Self {
            Self(std::time::Instant::now())
        }
    }
    #[inline(always)]
    pub fn get_timer() -> Value {
        Value(std::time::Instant::now())
    }
}

//mi get_timer for OTHER architectures
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64",)))]
use arch_std as arch;

//fi get_timer for Aarch64
/// Known to work on Apple M4 (MacbookPro 2024)
#[cfg(target_arch = "aarch64")]
mod arch {
    use std::arch::asm;
    pub type Value = u64;
    #[inline(always)]
    pub fn get_timer() -> u64 {
        let timer: u64;
        unsafe {
            asm!(
                "isb
                mrs {timer}, cntvct_el0",
                timer = out(reg) timer,
            );
        }
        timer
    }
}

//fi get_timer for x86_64
/// Not tested yet
#[cfg(target_arch = "x86_64")]
mod arch {
    use std::arch::asm;
    pub type Value = u64;
    #[inline(always)]
    pub fn get_timer() -> Value {
        let lo: u64;
        let hi: u64;
        unsafe {
            asm!(
                "ldfence
                rdtsc",
                lateout("eax") lo,
                lateout("edx") hi,
              options(nomem, nostack)
            );
        }
        hi << 32 | lo
    }
}
