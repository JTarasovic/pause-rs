#![no_main]
#![allow(unused_must_use)]

extern crate sc;
extern crate signal_hook_registry;

use std::process;

use signal_hook::consts::TERM_SIGNALS;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    for sig in TERM_SIGNALS {
        unsafe {
            signal_hook_registry::register(*sig, || {
                eprintln!("Exiting on signal: {}", *sig);
                process::exit(*sig);
            })
        };
    }

    loop {
        eprintln!("Pausing");
        unsafe {
            sc::syscall!(PAUSE);
        }
    }
}
