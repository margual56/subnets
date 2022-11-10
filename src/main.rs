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

    /// Option: -n --hosts <number of hosts>: Specifies a number of hosts to calculate a mask.
    #[clap(short = 'n', long, help = "Specifies the number of hosts.")]
    hosts: Option<u32>,

    /// Option: -s --station <station name>: Specifies the name of the station to be played
    #[clap(short, long, help = "Specifies the name of the station to be played.")]
    station: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let ip: IP = match args.hosts {
        None => IP::from_str(&args.ip),
        Some(hosts) => IP::from_hosts(&args.ip, hosts),
    };

    println!("{}", ip.summary());

    // reads until a \n is encountered
}
