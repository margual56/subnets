use std::{
    fmt,
    ops::{Add, BitAnd, Not},
};

use crate::ip::IPErr;

#[derive(Clone, Copy)]
pub struct Address {
    pub address: u32,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.address.to_be_bytes();

        write!(f, "{}.{}.{}.{}", a[0], a[1], a[2], a[3])
    }
}
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.address.to_be_bytes();

        write!(f, "{:08b}.{:08b}.{:08b}.{:08b}", a[0], a[1], a[2], a[3])
    }
}

impl BitAnd<Address> for Address {
    type Output = Address;

    fn bitand(self, rhs: Address) -> Self::Output {
        Self {
            address: self.address & rhs.address,
        }
    }
}

impl Not for Address {
    type Output = Address;

    fn not(self) -> Self::Output {
        Self {
            address: !self.address,
        }
    }
}

impl Add<u32> for Address {
    type Output = Address;

    fn add(self, rhs: u32) -> Self::Output {
        Address {
            address: self.address + rhs,
        }
    }
}

impl Address {
    pub fn from_str(s: &str) -> Result<Self, IPErr> {
        let ip = s.split(".").collect::<Vec<&str>>();

        if ip.len() < 2 {
            return Err(IPErr::InvalidFormat);
        }

        let i1: u32 = ip[0].parse::<u32>().unwrap() << 3 * 8;
        let i2: u32 = ip[1].parse::<u32>().unwrap() << 2 * 8;
        let i3: u32 = ip[2].parse::<u32>().unwrap() << 1 * 8;
        let i4: u32 = ip[3].parse::<u32>().unwrap() << 0;

        let address: u32 = i1 | i2 | i3 | i4;

        Ok(Address { address })
    }

    pub fn to_mask_repr(&self) -> u32 {
        32 - self.address.count_zeros()
    }

    pub fn from_mask_repr(m: u8) -> Self {
        let address: u32 = u32::MAX << (32 - m);

        Address { address }
    }
}
