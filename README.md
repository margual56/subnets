[![Rust](https://github.com/margual56/subnets/actions/workflows/rust.yml/badge.svg)](https://github.com/margual56/subnets/actions/workflows/rust.yml)

# Subnets
A CLI tool to analyze and calculate IPv4 addresses and masks

# Install
This is a single executable, so you can head to [the releases page](https://github.com/margual56/subnets/releases) and download the executable (then rename it to `subnets`) and put it wherever you want to!

Otherwise, if you have rust installed, you can run `cargo install --git "https://github.com/margual56/subnets"`.

# Usage
`subnets -h` will show the help.

For analyzing a single ip/mask just run `subnets "<IP/mask>"` (for example, `subnets "192.168.0.0/24"`).
