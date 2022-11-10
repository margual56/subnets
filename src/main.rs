use clap::Parser;
use colored::Colorize;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
use subnets::{address::Address, ip::IP};
use tabled::{builder::Builder, object::Rows, Alignment, ModifyObject, Style};

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

    /// Option: -s --subnets: Specify a config file with multiple subnets.
    #[clap(long, short, conflicts_with = "hosts", help = "Specify a config file.")]
    subnets: Option<PathBuf>,

    /// Option: -c --csv: Output in csv format.
    #[clap(long, short, default_value = "false", help = "Output in csv format.")]
    csv: bool,
}

fn main() {
    let args = Cli::parse();

    match args.subnets {
        None => {
            let ip: IP = match args.hosts {
                None => IP::from_str(&args.ip),
                Some(hosts) => IP::from_hosts(&args.ip, hosts),
            };

            if args.csv {
                println!("{}", ip.summary_csv());
            } else {
                println!("{}", ip.summary());
            }
        }
        Some(file) => {
            let Ok(mut subnets_file) = File::open(file.to_path_buf()) else {
                panic!("File '{}' does not exist", file.display());
            };

            let mut buf = String::new();
            let Ok(_) = subnets_file.read_to_string(&mut buf) else {
                panic!("Could not read config file");
            };

            let Ok(subnets) = serde_json::from_str::<HashMap<String, u32>>(&buf) else {
                panic!("Could not parse config file");
            };

            if args.csv {
                print_csv_table(&args.ip, subnets);
            } else {
                print_table(&args.ip, subnets);
            }
        }
    };

    // reads until a \n is encountered
}

fn print_table(orig_ip: &str, subnets: HashMap<String, u32>) {
    let mut v = Vec::from_iter(subnets);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let mut ip: Option<Address> = None;
    let mut builder = Builder::default();

    v.into_iter().for_each(|v| {
        let tmp_ip = match ip {
            None => IP::from_hosts(&orig_ip, v.1),
            Some(i) => IP::from_hosts(&format!("{}", i + 1), v.1),
        };

        let range = tmp_ip.get_range();

        builder.add_record(vec![
            v.0,
            format!("{} PCs", v.1),
            format!("(/{:<2}) {:<16}", tmp_ip.mask.to_mask_repr(), tmp_ip.mask),
            format!("{}", tmp_ip.get_hosts()),
            format!("{:<15} - {}", format!("{}", range.0), range.1),
        ]);

        ip = Some(range.1.clone());
    });

    builder.set_columns(vec![
        "Name",
        "Requested # of hosts",
        "Mask",
        "Max. # of hosts",
        "Range",
    ]);

    println!(
        "{}\r\n\t{}",
        builder
            .build()
            .with(Style::rounded())
            .with(Rows::new(1..).modify().with(Alignment::left()))
            .to_string(),
        "Note: Rember to reserve 1 address for the gateway!"
            .italic()
            .red()
            .dimmed()
    );
}

fn print_csv_table(orig_ip: &str, subnets: HashMap<String, u32>) {
    println!("Name,IP,IP_bin,Mask,Mask_long,Mask_bin,Hosts,Range_low,Range_high");

    let mut v = Vec::from_iter(subnets);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let mut ip: Option<Address> = None;

    v.into_iter().for_each(|v| {
        let tmp_ip = match ip {
            None => IP::from_hosts(&orig_ip, v.1),
            Some(i) => IP::from_hosts(&format!("{}", i + 1), v.1),
        };

        let range = tmp_ip.get_range();

        println!(
            "{},{},{:?},{},{},{:?},{},{},{}",
            v.0,
            tmp_ip.ip,
            tmp_ip.ip,
            tmp_ip.mask.to_mask_repr(),
            tmp_ip.mask,
            tmp_ip.mask,
            tmp_ip.get_hosts(),
            range.0,
            range.1
        );

        ip = Some(range.1.clone());
    });
}
