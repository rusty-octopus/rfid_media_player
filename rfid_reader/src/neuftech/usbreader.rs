use crate::usbreader::{UsbReader, UsbReaderError};

pub(crate) struct NeuftechUsbReader;

/*
  libusbutils holds all convenience functions
  NeuftechUsbReader or rawusbreader holds Context
  on each new read, the device handle is opened again and made available again
  Endpoint etc. is also stored
  maybe on drop or on each read the kernel is attached again
  Advanced option: Use thread and send data via channel, then on spawning the thread context,
  device handle etc. can be kept alive in thread
*/

impl UsbReader for NeuftechUsbReader {
    fn read(&self) -> Box<[u8]> {
        let data = (0..10).collect::<Vec<u8>>().into_boxed_slice();
        data
    }
}

#[derive(Debug, PartialEq)]
enum RawDataInterpretation {
    Value(u8),
    Repeated,
    Enter,
}

impl RawDataInterpretation {
    fn from(data: &[u8]) -> Result<RawDataInterpretation, NeuftechError> {
        if data.len() >= 3 {
            let value = data[2];
            let return_value = match value {
                0 => Ok(Self::Repeated),
                30..=39 => Ok(Self::Value(value)),
                40 => Ok(Self::Enter),
                _ => Err(NeuftechError::InvalidData),
            };
            return return_value;
        }
        Err(NeuftechError::TooFewReceivedData)
    }
}

#[derive(Debug, PartialEq)]
enum NeuftechError {
    TooFewReceivedData,
    InvalidData,
}

struct RawDataInterpreter {
    finished: bool,
    index: usize,
    data: [u8; 10],
    last: Option<RawDataInterpretation>,
}

impl Default for RawDataInterpreter {
    fn default() -> Self {
        RawDataInterpreter {
            finished: false,
            index: 0,
            data: [0; 10],
            last: None,
        }
    }
}

impl RawDataInterpreter {
    fn process(&mut self, raw_data: &[u8]) -> Result<(), NeuftechError> {
        let raw_data_interpretation = RawDataInterpretation::from(raw_data)?;
        match raw_data_interpretation {
            RawDataInterpretation::Value(value) => {
                self.data[self.index] = value;
                self.last = Some(raw_data_interpretation);
                self.index += 1;
            }
            RawDataInterpretation::Enter => {
                self.last = Some(raw_data_interpretation);
            }
            RawDataInterpretation::Repeated => {
                if self.index == 10 && self.last == Some(RawDataInterpretation::Enter) {
                    self.finished = true;
                }
            }
        }
        Ok(())
    }
    fn finished_processing(&self) -> bool {
        self.finished
    }
    fn reset(&mut self) {
        self.finished = false;
        self.last = None;
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_data_interpretation() {
        let data: [u8; 1] = [0];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Err(NeuftechError::TooFewReceivedData), result);

        let data: [u8; 3] = [1, 0, 39];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Value(39)), result);

        let data: [u8; 3] = [1, 0, 40];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Enter), result);
    }

    #[test]
    fn test_raw_data_interpreter() {
        let mut interpreter = RawDataInterpreter::default();
        let test_data = [1, 0, 39];
        for _ in 0..=9 {
            assert_eq!(Ok(()), interpreter.process(&test_data));
            assert!(!interpreter.finished_processing());
        }
        let enter_data = [1, 0, 40];
        interpreter.process(&enter_data);
        assert!(!interpreter.finished_processing());

        let ignore_data = [1, 0, 0];
        interpreter.process(&ignore_data);
        assert!(interpreter.finished_processing());
    }
}
