# Netlink Wireless Interface

[![Crates.io](https://img.shields.io/crates/v/netlink_wi)](https://crates.io/crates/netlink_wi)
[![docs](https://img.shields.io/badge/docs-netlink__wi-blue)](https://docs.rs/netlink_wi/latest/netlink_wi/)
[![Crates.io](https://img.shields.io/crates/l/netlink_wi)](./LICENSE)
[![CI](https://github.com/uption/netlink_wi/workflows/CI/badge.svg)](https://github.com/uption/netlink_wi/actions?query=workflow%3ACI)

**netlink_wi** is a Rust library for interacting with wireless devices on Linux systems through the **nl80211** netlink interface.

It provides programmatic access to wireless configuration and status information, allowing developers to query and control Wi-Fi interfaces directly from Rust code.

With **netlink_wi**, you can:

- Enumerate and inspect wireless network interfaces
- Retrieve hardware capabilities and detailed device specifications
- Access information about connected stations and peers
- Query wireless regulatory domain settings
- Configure wireless interface parameters

## Installation

```sh
cargo add netlink_wi
```

### Features

The library supports both synchronous and asynchronous APIs:

- `sync` (default) - Synchronous API using `NlSocket`
- `async` - Asynchronous API using `AsyncNlSocket`

To use the async version:

```sh
cargo add netlink_wi --no-default-features --features async
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
