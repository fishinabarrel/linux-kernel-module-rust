use core::convert::TryInto;

use crate::{bindings, c_types, error};

/// Fills `dest` with random bytes generated from the kernel's CSPRNG. Ensures
/// that the CSPRNG has been seeded before generating any random bytes, and
/// will block until it's ready.
pub fn getrandom(dest: &mut [u8]) -> error::KernelResult<()> {
    let res = unsafe { bindings::wait_for_random_bytes() };
    if res != 0 {
        return Err(error::Error::from_kernel_errno(res));
    }

    unsafe {
        bindings::get_random_bytes(
            dest.as_mut_ptr() as *mut c_types::c_void,
            dest.len().try_into()?,
        );
    }
    Ok(())
}

/// Fills `dest` with random bytes generated from the kernel's CSPRNG. If the
/// CSPRNG is not yet seeded, returns an `Err(EAGAIN)` immediately. Only
/// available on 4.19 and later kernels.
#[cfg(kernel_4_19_0_or_greater)]
pub fn getrandom_nonblock(dest: &mut [u8]) -> error::KernelResult<()> {
    if !unsafe { bindings::rng_is_initialized() } {
        return Err(error::Error::EAGAIN);
    }
    getrandom(dest)
}

/// Contributes the contents of `data` to the kernel's entropy pool. Does _not_
/// credit the kernel entropy counter though.
pub fn add_randomness(data: &[u8]) {
    unsafe {
        bindings::add_device_randomness(
            data.as_ptr() as *const c_types::c_void,
            data.len().try_into().unwrap(),
        );
    }
}
