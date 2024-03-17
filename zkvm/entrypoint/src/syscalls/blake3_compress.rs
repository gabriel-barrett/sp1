#[cfg(target_os = "zkvm")]
use core::arch::asm;

/// Blake3 compress operation.
///
/// The result is written over the input state.
#[no_mangle]
pub extern "C" fn syscall_blake3_compress_inner(state: *mut u32, message: *mut u32) {
    #[cfg(target_os = "zkvm")]
    unsafe {
        asm!(
            "ecall",
            in("t0") crate::syscalls::BLAKE3_COMPRESS_INNER,
            in("a0") state,
            in("a1") message
        );
    }

    #[cfg(not(target_os = "zkvm"))]
    unreachable!()
}
