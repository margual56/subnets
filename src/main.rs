use clap::Parser;
use subnets::ip::IP;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    /// IP address (with or without a mask)
    ip: String,

    /// Option: -u --url <URL>: Specifies an url to be played.
    #[clap(short, long, help = "Specifies an url to be played.")]
    url: Option<String>,

    /// Option: -s --station <station name>: Specifies the name of the station to be played
    #[clap(
        short,
        long,
        conflicts_with = "url",
        help = "Specifies the name of the station to be played."
    )]
    station: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // reads until a \n is encountered
    let Ok(ip) = IP::from_str(&args.ip) else {
        panic!("The IP is incorrect");
    };

    println!("IP: {}", ip);
    println!(
        "The mask is {} and can hold {} PCs (+1 gateway)",
        ip.mask,
        ip.get_hosts()
    );
    let range = ip.get_range();
    println!(
        "With this mask, the subnet IP range is from {} to {}",
        range.0, range.1
    );
    println!("IP in binary: {:?}", ip.ip);
    println!("Mask in binary: {:?}", ip.mask);
}
