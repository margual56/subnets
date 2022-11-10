[![Rust](https://github.com/margual56/subnets/actions/workflows/rust.yml/badge.svg)](https://github.com/margual56/subnets/actions/workflows/rust.yml)

# Subnets
A CLI tool to analyze and calculate IPv4 addresses and masks

## Install
This is a single executable, so you can head to [the releases page](https://github.com/margual56/subnets/releases) and download the executable (then rename it to `subnets`) and put it wherever you want to!

Otherwise, if you have rust installed, you can run `cargo install --git "https://github.com/margual56/subnets"`.

## Run example
_(Here the colors are missing)_
```
$ subnets -h
Usage: subnets [OPTIONS] <IP>

Arguments:
  <IP>  IP address (with or without a mask)

Options:
  -n, --hosts <HOSTS>      Specifies the number of hosts.
  -s, --subnets <SUBNETS>  Specify a config file.
  -c, --csv                Output in csv format.
  -h, --help               Print help information (use `--help` for more detail)
  -V, --version            Print version information

$ subnets "192.168.0.0/26"
IP: 192.168.0.0/26
The mask is 255.255.255.192 and can hold 62 PCs (or 61 PCs and 1 gateway)
With this mask, the subnet IP range is from 192.168.0.0 to 192.168.0.63
IP in binary:   11000000.10101000.00000000.00000000
Mask in binary: 11111111.11111111.11111111.11000000

$ subnets "192.168.0.0" --subnets test.json
╭───────────────────┬──────────────────────┬───────────────────────┬─────────────────┬───────────────────────────────────╮
│ Name              │ Requested # of hosts │ Mask                  │ Max. # of hosts │ Range                             │
├───────────────────┼──────────────────────┼───────────────────────┼─────────────────┼───────────────────────────────────┤
│ Sala B2           │ 241 PCs              │ (/24) 255.255.255.0   │ 254             │ 192.168.128.0   - 192.168.128.255 │
│ Sala B1           │ 131 PCs              │ (/24) 255.255.255.0   │ 254             │ 192.168.129.0   - 192.168.129.255 │
│ Sala A2           │ 120 PCs              │ (/25) 255.255.255.128 │ 126             │ 192.168.130.0   - 192.168.130.127 │
│ Sala A1           │ 96 PCs               │ (/25) 255.255.255.128 │ 126             │ 192.168.130.128 - 192.168.130.255 │
│ Sala C1           │ 61 PCs               │ (/26) 255.255.255.192 │ 62              │ 192.168.131.0   - 192.168.131.63  │
│ Sala C2           │ 25 PCs               │ (/27) 255.255.255.224 │ 30              │ 192.168.131.64  - 192.168.131.95  │
│ Sala D1           │ 15 PCs               │ (/27) 255.255.255.224 │ 30              │ 192.168.131.96  - 192.168.131.127 │
│ Sala D2           │ 10 PCs               │ (/28) 255.255.255.240 │ 14              │ 192.168.131.128 - 192.168.131.143 │
│ Router4 - Router3 │ 2 PCs                │ (/30) 255.255.255.252 │ 2               │ 192.168.131.144 - 192.168.131.147 │
│ Router1 - Router5 │ 2 PCs                │ (/30) 255.255.255.252 │ 2               │ 192.168.131.148 - 192.168.131.151 │
│ Router2 - Router1 │ 2 PCs                │ (/30) 255.255.255.252 │ 2               │ 192.168.131.152 - 192.168.131.155 │
│ Router5 - Router4 │ 2 PCs                │ (/30) 255.255.255.252 │ 2               │ 192.168.131.156 - 192.168.131.159 │
╰───────────────────┴──────────────────────┴───────────────────────┴─────────────────┴───────────────────────────────────╯
	Note: Rember to reserve 1 address for the gateway!
```

## Usage
`subnets -h` will show the help.

For analyzing a single ip/mask just run `subnets "<IP/mask>"` (for example, `subnets "192.168.0.0/24"`).
