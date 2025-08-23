/// Wraps the `core::hint::likely` intrinsic if the `nightly` feature is enabled.
#[inline(always)]
pub(crate) const fn likely(x: bool) -> bool {
    #[cfg(feature = "nightly")] {
        core::hint::likely(x)
    }

    #[cfg(not(feature = "nightly"))] {
        x
    }
}

/// Wraps the `core::hint::unlikely` intrinsic if the `nightly` feature is enabled.
#[inline(always)]
pub(crate) const fn unlikely(x: bool) -> bool {
    #[cfg(feature = "nightly")] {
        core::hint::unlikely(x)
    }

    #[cfg(not(feature = "nightly"))] {
        x
    }
}


