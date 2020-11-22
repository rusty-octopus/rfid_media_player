#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

mod rusbhumbleusbdevice;
mod utils;

pub(crate) use rusbhumbleusbdevice::open;
