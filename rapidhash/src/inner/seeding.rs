//! Internal module for seeding the hash functions.

pub(super) mod seed {
    use core::cell::Cell;
    use crate::inner::rapid_const::{rapidhash_seed, RAPID_SECRET};
    use crate::util::mix::rapid_mix;

    #[inline]
    pub(crate) fn get_seed() -> u64 {
        // this would all be so much easier if the rust std exposed how it does RandomState
        // we take the stack pointer as a rather poor but cheap source of entropy
        let mut seed = 0;
        let arbitrary = core::ptr::addr_of!(seed) as u64;

        // with std we avoid using global atomics
        #[cfg(feature = "std")] {
            thread_local! {
                static RANDOM_SEED: Cell<u64> = const {
                    Cell::new(0)
                }
            }

            seed = RANDOM_SEED.with(|cell| {
                let mut seed = cell.get();
                seed = rapid_mix::<false>(seed ^ RAPID_SECRET[1], arbitrary ^ RAPID_SECRET[0]);
                cell.set(seed);
                seed
            });
        }

        // without std we fall back to a global atomic and accept the chance of
        // race conditions, but don't consider this an issue
        #[cfg(not(feature = "std"))] {
            use core::sync::atomic::{AtomicUsize, Ordering};
            static RANDOM_SEED: AtomicUsize = AtomicUsize::new(0);

            seed = RANDOM_SEED.load(Ordering::Relaxed) as u64;
            seed = crate::util::mix::rapid_mix::<false>(seed ^ RAPID_SECRET[1], arbitrary ^ RAPID_SECRET[0]);
            RANDOM_SEED.store(seed as usize, Ordering::Relaxed);
        }

        rapidhash_seed(seed)
    }
}

#[cfg(not(target_has_atomic = "ptr"))]
pub(super) mod secrets {
    #[inline(always)]
    pub(crate) fn get_secrets() -> &'static [u64; 7] {
        // This is a no-op for platforms that do not support atomic pointers.
        // The secrets are not used, so we return an empty slice.
        &crate::inner::rapid_const::RAPID_SECRET.first_chunk().unwrap()
    }
}

#[cfg(target_has_atomic = "ptr")]
pub(super) mod secrets {
    use core::cell::UnsafeCell;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use crate::inner::rapid_const::RAPID_SECRET;
    use crate::util::mix::rapid_mix;

    /// A hacky sync-friendly, std-free, OnceCell that sadly needs unsafe inspired by foldhash's
    /// `seed.rs` which includes some similar bodges.
    struct SecretStorage {
        state: AtomicUsize,
        secrets: UnsafeCell<[u64; 7]>,
    }

    unsafe impl Sync for SecretStorage {}

    static SECRET_STORAGE: SecretStorage = SecretStorage {
        state: AtomicUsize::new(0),
        secrets: UnsafeCell::new([0; 7]),
    };

    enum SecretStorageStates {
        Uninitialized = 0,
        Initializing = 1,
        Initialized = 2,
    }

    #[inline]
    pub(crate) fn get_secrets() -> &'static [u64; 7] {
        if SECRET_STORAGE.state.load(Ordering::Acquire) != SecretStorageStates::Initialized as usize {
            initialize_secrets();
        }

        // SAFETY: The secrets are guaranteed to be initialized before being accessed
        unsafe {
            &*SECRET_STORAGE.secrets.get()
        }
    }

    fn initialize_secrets() {
        let secrets = create_secrets();
        const INITIALIZED: usize = SecretStorageStates::Initialized as usize;

        loop {
            match SECRET_STORAGE.state.compare_exchange_weak(
                SecretStorageStates::Uninitialized as usize,
                SecretStorageStates::Initializing as usize,
                Ordering::Acquire,
                Ordering::Acquire,
            ) {
                // This thread is the first to initialize, so we can safely set the secrets
                Ok(_) => {
                    unsafe {
                        *SECRET_STORAGE.secrets.get() = secrets;
                    }
                    SECRET_STORAGE.state.store(SecretStorageStates::Initialized as usize, Ordering::Release);
                    break;
                }

                // Another thread has initialized for us, so we're done.
                Err(INITIALIZED) => {
                    return;
                }

                // We are spinning here until the other thread is done initializing. This is a
                // one-time thing and creating secrets _shouldn't_ take long...
                _ => core::hint::spin_loop(),
            }
        }
    }

    fn create_secrets() -> [u64; 7] {
        let mut secrets = [0u64; 7];
        let seed = generate_random();

        // TODO: check quality of the generated secrets
        for i in 0..secrets.len() {
            secrets[i] = rapid_mix::<true>(seed, RAPID_SECRET[i]);
        }

        secrets
    }

    /// Generate a random number, trying our best to make this a good random number.
    ///
    /// To only be called sparingly as it's fairly slow.
    fn generate_random() -> u64 {
        #[cfg(feature = "rand")]
        {
            rand::random()
        }

        #[cfg(not(feature = "rand"))]
        {
            use crate::inner::rapid_const::{RAPID_SEED, RAPID_SECRET, RAPID_CONST};

            // trying out best to generate a good random number on all platforms
            let mut seed = RAPID_SEED;
            let stack_ptr = core::ptr::addr_of!(seed) as u64;
            let static_ptr = &RAPID_SECRET as *const _ as usize as u64;
            let function_ptr = generate_random as *const () as usize as u64;

            seed = rapid_mix::<true>(seed ^ RAPID_SECRET[4], stack_ptr ^ RAPID_SECRET[1]);
            seed = rapid_mix::<true>(seed ^ RAPID_SECRET[5], function_ptr ^ RAPID_SECRET[2]);
            seed = rapid_mix::<true>(seed ^ RAPID_SECRET[6], static_ptr ^ RAPID_SECRET[3]);

            #[cfg(feature = "std")]
            {
                // we can allocate to add extra noise
                let box_ptr = &*Box::new(1u64) as *const _ as usize as u64;
                seed = rapid_mix::<true>(seed ^ RAPID_SECRET[4], box_ptr ^ RAPID_SECRET[1]);
            }

            #[cfg(all(
                feature = "std",
                not(any(
                    miri,
                    all(target_family = "wasm", target_os = "unknown"),
                    target_os = "zkvm"
                ))
            ))]
            {
                // we can use the system time for extra noise
                seed = crate::rng::rapidrng_time(&mut seed);
            }

            // final avalanche step
            seed = rapid_mix::<true>(seed ^ RAPID_CONST, RAPID_SECRET[0]);
            seed
        }
    }
}
