use rfid_media_player::{
    convert_to_id, create_app, open, RfidMediaPlayer, CONSOLE_LOG_ARGUMENT_NAME,
    LOG_LEVEL_ARGUMENT_NAME, PRODUCT_ID_ARGUMENT_NAME, TIMEOUT_ARGUMENT_NAME,
    TRACKS_FILE_ARGUMENT_NAME, VENDOR_ID_ARGUMENT_NAME,
};

use rfid_reader::{ProductId, VendorId};

use flexi_logger::{Duplicate, LogTarget, Logger};
use log::info;

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;

use std::sync::Arc;
use std::thread;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_app().get_matches();

    // default is set, therefore unwrapping is safe.
    let log_spec = matches.value_of(LOG_LEVEL_ARGUMENT_NAME).unwrap();

    let mut logger = Logger::with_str(log_spec).log_target(LogTarget::File);

    let stdout = matches.is_present(CONSOLE_LOG_ARGUMENT_NAME);
    if stdout {
        logger = logger.duplicate_to_stdout(Duplicate::All);
    }
    logger.start()?;

    info!("Started rfid_media_player");

    let vendor_id = VendorId::from(convert_to_id(
        matches
            .value_of(VENDOR_ID_ARGUMENT_NAME)
            .unwrap()
            .to_string(),
    )?);
    let product_id = ProductId::from(convert_to_id(
        matches
            .value_of(PRODUCT_ID_ARGUMENT_NAME)
            .unwrap()
            .to_string(),
    )?);

    let timeout = Duration::from_millis(u64::from_str_radix(
        matches.value_of(TIMEOUT_ARGUMENT_NAME).unwrap(),
        10,
    )?);

    let yaml_string =
        std::fs::read_to_string(matches.value_of(TRACKS_FILE_ARGUMENT_NAME).unwrap())?;

    let mut rfid_media_player = open(vendor_id, product_id, timeout, &yaml_string)?;

    info!("Application opened successfully");

    // Shared atomic bool to signal that the program is aborted
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // all terminating signals
    let mut signals = Signals::new(TERM_SIGNALS)?;

    // spawn a thread to react to all terminating signals
    thread::spawn(move || {
        for sig in signals.forever() {
            info!("Received signal {:?}", sig);
            r.store(false, Ordering::SeqCst);
        }
    });

    while running.load(Ordering::SeqCst) {
        rfid_media_player.run()?;
    }
    rfid_media_player.shutdown()?;

    Ok(())
}
