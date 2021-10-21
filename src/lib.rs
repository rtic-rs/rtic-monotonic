//! Core abstractions of the Real-Time Interrupt-driven Concurrency (RTIC) Monotonic timers, used
//! internally for scheduling and users can use them for time.
//!
//! You can write generic *libraries* and HALs using the `Monotonic` trait in this crate. If you
//! want to write application code then you'll need an *implementation* of the RTIC framework for a
//! particular architecture. Currently, there are implementations for these architectures and OSes:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtic)

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

// use core::cmp::Ordering;

/// # A monotonic clock / counter definition.
///
/// RTIC assumes a MonoInit implementation for setup in the #[init] task.
/// The API is very limited.
pub trait MonoInit {
    /// The type for instant
    type Instant: Ord + Eq + Copy;

    /// The type for duration
    type Duration;

    /// The type for the monotonic
    type Mono;

    /// Get the current time instant.
    /// Should return zero if `reset` is implemented for the corresponding Mono type;
    fn now(&mut self) -> Self::Instant;

    /// Called by RTIC after #[init] but before any tasks are executed.
    /// The MonoInit implementer is turned into a Mono type (implementing the Monotonic trait)
    fn into_mono(self) -> Self::Mono;
}
/// ## Correctness
///
/// When implementing this trait it is important to decide if one want to have a fixed baseline and
/// utilize the `reset` method. If not, one can implement `reset` as an empty method. If
/// `reset` is **not empty**, it is **not allowed** for `now()` to return
/// nonsensical values if called before `reset` is invoked by the runtime. Therefore implementation
/// authors should have methods in place for making sure of this, for example a flag in the timer
/// which tracks if the `reset` method has been called yet, and if not always return `0`.
///

pub trait Monotonic {
    /// This tells RTIC if it should disable the interrupt bound to the monotonic if there are no
    /// scheduled tasks. One may want to set this to `false` if one is using the `on_interrupt`
    /// method to perform housekeeping and need overflow interrupts to happen, such as when
    /// extending a 16 bit timer to 32/64 bits, even if there are no scheduled tasks.
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    /// The type for instant
    type Instant: Ord + Eq + Copy;

    /// The type for duration
    type Duration;

    /// Optionally resets the counter to *zero* for a fixed baseline in a system.
    ///
    /// This method will be called *exactly once* by the RTIC runtime after `#[init]` returns and
    /// before tasks can start.
    fn reset(&mut self) {}

    /// Get the current time instant
    fn now(&mut self) -> Self::Instant;

    /// Set the compare value of the timer interrupt.
    fn set_compare(&mut self, instant: &Self::Instant);

    /// Clear the compare interrupt flag.
    fn clear_compare_flag(&mut self);

    /// Optional. Commonly used for performing housekeeping of a timer when it has been extended,
    /// e.g. a 16 bit timer extended to 32/64 bits. This will be called at the end of the interrupt
    /// handler after all other operations have finished.
    fn on_interrupt(&mut self) {}

    /// Optional. This is used to save power, this is called when the Monotonic interrupt is
    /// enabled.
    fn enable_timer(&mut self) {}

    /// Optional. This is used to save power, this is called when the Monotonic interrupt is
    /// disabled.
    fn disable_timer(&mut self) {}
}
