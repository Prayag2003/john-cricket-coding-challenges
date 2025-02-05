use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn setup_signal_handlers() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nmyshell> ");
    })
    .expect("Error setting Ctrl-C handler");
}
