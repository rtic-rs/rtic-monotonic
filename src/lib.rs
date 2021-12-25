//! Core abstractions of the Real-Time Interrupt-driven Concurrency (RTIC) Monotonic timers, used
//! internally for scheduling and users can use them for time.
//!
//! You can write generic *libraries* and HALs using the `Monotonic` trait in this crate. If you
//! want to write application code then you'll need an *implementation* of the RTIC framework for a
//! particular architecture. Currently, there are implementations for these architectures and OSes:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtic)

#![deny(missing_docs)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![no_std]
//deny_warnings_placeholder_for_ci

use core::ops::{Add, Sub};

/// # A monotonic clock / counter definition.
///
/// ## Correctness
///
/// The trait enforces that proper time-math is implemented between `Instant` and `Duration`. This
/// is a requirement on the time library that the user chooses to use.
pub trait Monotonic {
    /// This tells RTIC if it should disable the interrupt bound to the monotonic if there are no
    /// scheduled tasks. One may want to set this to `false` if one is using the `on_interrupt`
    /// method to perform housekeeping and need overflow interrupts to happen, such as when
    /// extending a 16 bit timer to 32/64 bits, even if there are no scheduled tasks.
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    /// The type for instant, defining an instant in time.
    ///
    /// **Note:** In all APIs in RTIC that use instants from this monotonic, this type will be used.
    type Instant: Ord
        + Copy
        + Add<Self::Duration, Output = Self::Instant>
        + Sub<Self::Duration, Output = Self::Instant>
        + Sub<Self::Instant, Output = Self::Duration>;

    /// The type for duration, defining an duration of time.
    ///
    /// **Note:** In all APIs in RTIC that use duration from this monotonic, this type will be used.
    type Duration;

    /// Get the current time.
    fn now(&mut self) -> Self::Instant;

    /// Set the compare value of the timer interrupt.
    ///
    /// **Note:** This method does not need to handle race conditions of the monotonic, the timer
    /// queue in RTIC checks this.
    fn set_compare(&mut self, instant: Self::Instant);

    /// Clear the compare interrupt flag.
    fn clear_compare_flag(&mut self);

    /// The time at time zero. Used by RTIC before the monotonic has been initialized.
    fn zero() -> Self::Instant;

    /// Optionally resets the counter to *zero* for a fixed baseline in a system.
    ///
    /// This method will be called *exactly once* by the RTIC runtime after `#[init]` returns and
    /// before tasks start.
    ///
    /// # Safety
    ///
    /// ## Correctness
    ///
    /// The user may not call this method.
    unsafe fn reset(&mut self);

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
