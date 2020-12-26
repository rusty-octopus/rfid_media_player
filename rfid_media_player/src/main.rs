use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Shared atomic bool to signal that the program is aborted
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // all terminating signals
    let mut signals = Signals::new(TERM_SIGNALS)?;

    // spawn a thread to react to all terminating signals
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            r.store(false, Ordering::SeqCst);
        }
    });

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");

    Ok(())
}
