pub mod address;
pub mod ip;

#[cfg(test)]
mod tests {
    use crate::ip::IP;
    const IP_MASK_STR: &str = "192.168.0.0/20";
    const IP_STR: &str = "192.168.0.0";

    #[test]
    fn test_ip() {
        let i = IP::from_str(IP_MASK_STR);
        println!("\t{}", i);
        assert_eq!(format!("{}", i), "192.168.0.0/20");
    }

    #[test]
    fn test_bin() {
        let i = IP::from_str(IP_MASK_STR);
        println!("\t{:?}", i);
        assert_eq!(
            format!("{:?}", i),
            "11000000.10101000.00000000.00000000/11111111.11111111.11110000.00000000"
        );
    }

    #[test]
    fn test_range() {
        let i = IP::from_str(IP_MASK_STR);
        let range = i.get_range();

        println!(
            "\tWith mask /{}: {} -> {}",
            i.mask.to_mask_repr(),
            range.0,
            range.1
        );
        assert_eq!(range.0.address, 3232235520);
        assert_eq!(range.1.address, 3232239615);
    }

    #[test]
    fn test_mask_from_hosts() {
        let ip = IP::from_hosts(IP_STR, 2048);

        println!("\t{} for {} hosts -> {}", IP_STR, 2048, ip);

        assert_eq!(format!("{}", ip), "192.168.0.0/20");
        assert_eq!(ip.get_hosts(), 4094);
    }

    #[test]
    fn real_example() {
        let ip = IP::from_hosts("192.168.131.64", 25);
        let range = ip.get_range();
        println!("\t{} -- {}", range.0, range.1);
        println!("\tThere is space for {} PCs", ip.get_hosts());

        assert_eq!(format!("{}", range.0), "192.168.131.64");
        assert_eq!(format!("{}", range.1), "192.168.131.95");
        assert_eq!(ip.get_hosts(), 30);
    }
}
