use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
/// Set the panicking behavior to halt
///
/// An implementation of `panic_fmt` that simply halt in an infinite loop.
///
/// # Usage
///
/// ```
/// #![no_std]
///
/// fn main() {
///     panic!("argument is ignored");
/// }
/// ```
///
/// # Breakable symbols
///
/// With the panic handler being `#[inline(never)]` the symbol `rust_begin_unwind` will be
/// available to place a breakpoint on to halt when a panic is happening.
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}