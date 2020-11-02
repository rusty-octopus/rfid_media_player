use crate::usbreader::{UsbReader, UsbReaderError};

pub(crate) struct NeuftechUsbReader;

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
}
