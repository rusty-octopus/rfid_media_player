use rfid_media_player::{open, RfidMediaPlayer};

use rfid_reader::{ProductId, RfidReader, VendorId};

use flexi_logger::{Duplicate, LogTarget, Logger};
use log::{error, info};

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;

use std::sync::Arc;
use std::thread;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

fn read_rfid(rfid_reader: &impl RfidReader) -> Option<String> {
    let read_result = rfid_reader.read();
    match read_result {
        Ok(rfid_value) => {
            info!("Received RFID value: {}", rfid_value);
            Some(rfid_value)
        }
        Err(error) => {
            error!("Reading RFID resolved in error: {}", error);
            None
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Use clap to get all parameters

    let log_spec = "info";

    let mut logger = Logger::with_str(log_spec).log_target(LogTarget::File);

    let stdout = true;
    if stdout {
        logger = logger.duplicate_to_stdout(Duplicate::All);
    }
    logger.start()?;

    // neuftech device
    let vendor_id = VendorId::from(0x16c0);
    let product_id = ProductId::from(0x27db);

    // bluetooth device
    //let vendor_id = VendorId::from(0x0cf3);
    //let product_id = ProductId::from(0x3005);

    let timeout = Duration::from_secs(60);
    // TODO: add actual id here
    let yaml_string = "0006641642: ../media_player/tests/rand1.wav";

    if let Some(mut rfid_media_player) = open(vendor_id, product_id, timeout, yaml_string) {
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

        //while running.load(Ordering::SeqCst) {
        let rfid_reader = rfid_reader::open(vendor_id, product_id, timeout)?;
        loop {
            //let value = rfid_reader.read()?;
            //info!("Read value: {}", value);
            rfid_media_player.run();
            //let option_rfid_value = read_rfid(&rfid_reader);
            //info!("Optional value: {:?}", option_rfid_value);
        }
        rfid_media_player.shutdown();
    }

    Ok(())
}
