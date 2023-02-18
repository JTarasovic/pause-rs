extern crate sc;
extern crate signal_hook_registry;

use std::{io::Error, process};

use signal_hook::consts::TERM_SIGNALS;

fn main() -> std::result::Result<(), Error> {
    for sig in TERM_SIGNALS {
        unsafe {
            signal_hook_registry::register(*sig, || {
                eprintln!("Exiting on signal: {}", *sig);
                process::exit(*sig)
            })
        }?;
    }

    loop {
        eprintln!("Pausing");
        unsafe {
            sc::syscall!(PAUSE);
        }
    }
}
