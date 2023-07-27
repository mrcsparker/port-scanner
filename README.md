# Port Scanner

The **Port Scanner** is a simple command-line tool written in Rust that allows you to scan for open ports on a target host. It uses the [Clap](https://crates.io/crates/clap) library for parsing command-line arguments and the [Rayon](https://crates.io/crates/rayon) library for parallelizing the port scanning process, making it efficient and fast.

## Table of Contents

- [Port Scanner](#port-scanner)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Command-line Options](#command-line-options)
  - [Example](#example)
  - [License](#license)

## Installation

To use the **Port Scanner**, you need to have [Rust](https://www.rust-lang.org/) installed on your system. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/learn/get-started) to install it.

Once you have Rust installed, you can clone this repository and build the project using Cargo, the package manager and build tool for Rust:

```bash
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner
cargo build --release
```

After building the project, you will find the executable binary in the `target/release` directory.

## Usage

To scan for open ports on a target host, simply run the executable binary followed by the target host name or IP address:

```bash
./port-scanner --target 192.168.0.1
```

By default, the scanner will check for open ports in the range of 1 to 1024. You can customize the port range by modifying the `PortScanner` struct in the code.

## Command-line Options

The **Port Scanner** supports the following command-line options:

- `-t, --target <HOST>`: Specifies the host name or IP address to scan for open ports. If not provided, the default value is `127.0.0.1`.

## Example

```bash
./port-scanner --target example.com
```

Output:
```
Open Ports: [80, 443]
```

In this example, the tool scanned for open ports on the host `example.com`, and it found that ports 80 (HTTP) and 443 (HTTPS) were open.

## License

This project is licensed under the [MIT License](LICENSE).

---

Happy port scanning! If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit a pull request. Contributions are welcome!
