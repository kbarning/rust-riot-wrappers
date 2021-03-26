#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
    use crate::thread;

    use crate::stdio;
    use core::fmt::Write;

    if unsafe { riot_sys::irq_is_in() } != 0 {
        // Touch luck. Jumping into an endless loop right away seems to be the only reliable way to
        // keep the interupt from ever entering again.
        loop {
            // Primarily for its side effect of making the behavior not undefined, but also because
            // any power saving would be good until the watchdog kicks in (you do have a watchdog,
            // right?)
            core::hint::spin_loop();
        }
    }

    // I *guess* it's OK for a panic to simply make a thread into a zombie -- this does allow other
    // threads (including spawned Rust threads) to continue, but my layman's understanding of
    // panicking is that that's OK because whatever we were just mutating can simply never be used
    // by someone else ever again.
    let mut stdout = stdio::Stdio {};
    let me = thread::get_pid();

    // Ignoring any errors -- there's not much we can do any more.
    let _ = writeln!(
        stdout,
        "Error in thread {:?} ({}):",
        me,
        me.get_name().unwrap_or("unnamed")
    );
    let _ = writeln!(stdout, "{}", info);

    // Not trying any unwinding -- this thread is just dead, won't be re-claimed, any mutexes it
    // holds are just held indefinitely rather than throwing poison errors.
    loop {
        thread::sleep();
    }
}

// This is only needed to build the i686 version
#[lang = "eh_personality"]
fn rust_eh_personality() {
    loop {}
}
