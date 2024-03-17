#[cfg(target_os = "zkvm")]
use core::arch::asm;

#[no_mangle]
pub extern "C" fn syscall_sha256_extend(w: *mut u32) {
    #[cfg(target_os = "zkvm")]
    unsafe {
        asm!(
            "ecall",
            in("t0") crate::syscalls::SHA_EXTEND,
            in("a0") w
        );
    }

    #[cfg(not(target_os = "zkvm"))]
    unreachable!()
}
