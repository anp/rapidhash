/// Provides a stable `assume` function that uses `core::hint::assert_unchecked`.
#[rustversion::since(1.81)]
#[inline(always)]
pub(crate) const unsafe fn assume(cond: bool) {
    core::hint::assert_unchecked(cond);
}

/// Provides a stable `assume` function that uses `core::hint::assert_unchecked`.
#[rustversion::before(1.81)]
#[inline(always)]
pub(crate) const unsafe fn assume(_cond: bool) { }
