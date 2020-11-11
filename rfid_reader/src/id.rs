use std::fmt;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VendorId(u16);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ProductId(u16);

pub trait Id {
    fn new(id: u16) -> Self;
}

macro_rules! implement_from_trait {
    ($id_type:tt) => {
        impl From<u16> for $id_type {
            fn from(id: u16) -> Self {
                $id_type(id)
            }
        }
    };
}

macro_rules! implement_into_trait {
    ($id_type:tt) => {
        impl Into<u16> for $id_type {
            fn into(self) -> u16 {
                self.0
            }
        }
    };
}

macro_rules! implement_display_trait {
    ($id_type:tt) => {
        impl fmt::Display for $id_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:#06x}", self.0)
            }
        }
    };
}

implement_from_trait!(VendorId);
implement_from_trait!(ProductId);
implement_into_trait!(VendorId);
implement_into_trait!(ProductId);
implement_display_trait!(VendorId);
implement_display_trait!(ProductId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_id_from() {
        let vendor_id = VendorId::from(16);
        assert_eq!(16, vendor_id.0);
    }

    #[test]
    fn product_id_from() {
        let product_id = ProductId::from(16);
        assert_eq!(16, product_id.0);
    }

    #[test]
    fn vendor_id_display() {
        let vendor_id = VendorId::from(16);
        assert_eq!("0x0010", format!("{}", vendor_id));
    }

    #[test]
    fn product_id_display() {
        let product = ProductId::from(16);
        assert_eq!("0x0010", format!("{}", product));
    }
}
