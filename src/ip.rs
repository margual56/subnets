use super::address::Address;
use colored::*;
use std::fmt;
use std::fmt::Write;

#[derive(Clone, Copy)]
pub struct IP {
    pub ip: Address,
    pub mask: Address,
}

#[derive(Debug)]
pub enum IPErr {
    InvalidFormat,
    Other,
}

impl fmt::Display for IPErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IPErr::InvalidFormat => write!(f, "IP format is invalid"),
            IPErr::Other => write!(f, "Unknown error!"),
        }
    }
}

impl fmt::Display for IP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask.to_mask_repr())
    }
}
impl fmt::Debug for IP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            format!("{:?}", self.ip).bold(),
            format!("{:?}", self.mask.to_mask_repr()).red()
        )
    }
}

impl IP {
    pub fn from_str(txt: &str) -> Self {
        let spl = txt.split("/").collect::<Vec<&str>>();

        if spl.len() < 2 {
            panic!("The IP did not contain a mask");
        }

        IP {
            ip: Address::from_str(spl[0]).unwrap(),
            mask: Address::from_mask_repr(spl[1].parse::<u8>().unwrap()),
        }
    }

    pub fn from_hosts(txt: &str, hosts: u32) -> Self {
        let ip = Address::from_str(txt).unwrap();

        let repr = 32 - (hosts as f32 + 2f32).log2().ceil() as u8;

        let mask: Address = Address::from_mask_repr(repr);

        IP { ip, mask }
    }

    pub fn get_range(&self) -> (Address, Address) {
        let min_ip = self.ip & self.mask;
        let max_ip = min_ip + (u32::pow(2, 32 - self.mask.to_mask_repr()) - 1);

        (min_ip, max_ip)
    }

    pub fn get_hosts(&self) -> u32 {
        u32::pow(2, 32 - self.mask.to_mask_repr()) - 2
    }

    pub fn summary(&self) -> String {
        let mut f = String::new();

        writeln!(
            &mut f,
            "IP: {}/{}",
            format!("{}", self.ip).cyan(),
            format!("{}", self.mask.to_mask_repr()).red()
        )
        .unwrap();

        writeln!(
            f,
            "The mask is {} and can hold {} PCs (or {} PCs and {} gateway)",
            format!("{}", self.mask).red(),
            format!("{}", self.get_hosts()).green(),
            format!("{}", self.get_hosts() - 1).yellow(),
            "1".yellow()
        )
        .unwrap();

        let range = self.get_range();

        writeln!(
            f,
            "With this mask, the subnet IP range is from {} to {}",
            format!("{}", range.0).bold().cyan(),
            format!("{}", range.1).bold().yellow()
        )
        .unwrap();

        writeln!(f, "IP in binary:   {}", format!("{:?}", self.ip).bold()).unwrap();
        writeln!(f, "Mask in binary: {}", format!("{:?}", self.mask).bold()).unwrap();

        return f;
    }
}
