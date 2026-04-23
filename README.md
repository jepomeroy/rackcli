# rackcli

A CLI tool for managing PoE switches via SNMP and Wake-on-LAN devices.

## Features

- Enable, disable, and query the status of individual PoE switch ports
- SNMP v2c and v3 support
- SNMP v3 authentication: MD5, SHA1, SHA224, SHA256, SHA384, SHA512
- SNMP v3 encryption: None, DES, AES128, AES192, AES256
- System keystore integration (macOS Keychain, GNOME Keyring / KWallet) for secure credential storage
- Send Wake-on-LAN magic packets to registered devices
- Per-device configuration stored in a local TOML file

## Supported Switch Brands

Aruba, Cisco, Dell, Juniper, Netgear, TP-Link, Ubiquiti

All brands use the standard IEEE 802.3af PoE MIB OID (`pethPsePortAdminEnable`).

## Prerequisites

OpenSSL with the legacy provider enabled (required for DES encryption). On most systems this is available by default. On Linux you may need to install `libssl-dev` or equivalent.

## Installation

```
cargo install --path .
```

## Usage

```
rackcli <COMMAND>

Commands:
  switch  Manage PoE switches
  wol     Manage Wake-on-LAN devices
  list    List all configured devices
```

### Switch Commands

```
rackcli switch <COMMAND>

Commands:
  add     Add a new switch
  delete  Delete a switch
  list    List all switches
  update  Update a switch
  enable  Enable ports on a switch
  disable Disable ports on a switch
  status  Get port status for a switch
```

#### Examples

```bash
# Add a new switch (interactive)
rackcli switch add

# Check port status
rackcli switch status

# Enable ports
rackcli switch enable

# Disable ports
rackcli switch disable
```

All commands that require selecting a switch or specifying a port range are interactive. The port range prompt accepts single ports (`4`), ranges (`1-8`), and combinations (`1-6,8,10-12`).

### Wake-on-LAN Commands

```
rackcli wol <COMMAND>

Commands:
  add     Add a new WoL device
  delete  Delete a WoL device
  list    List all WoL devices
  update  Update a WoL device
  enable  Send a magic packet to a WoL device
```

## Configuration

The configuration file is stored at:

| Platform | Path |
|---|---|
| Linux | `~/.config/com.epomeroy.rackcli/config.toml` |
| macOS | `~/Library/Application Support/com.epomeroy.rackcli/config.toml` |

The file is created automatically on first run. It is written with `0600` permissions (owner read/write only).

Credentials are **never** stored in the config file. They are either stored in the system keystore or prompted at runtime.

## Credential Storage

When adding or updating a switch you choose one of two credential modes:

**System keystore (recommended)** — passwords are stored in the OS keystore (macOS Keychain on macOS, GNOME Keyring or KWallet on Linux). Credentials are loaded automatically at runtime with no prompt.

**Prompt at runtime** — no credentials are stored anywhere. You are prompted for the community string (v2c) or auth/encryption passwords (v3) each time you run a command against that switch.

On headless Linux systems without a keyring daemon, the keystore mode will fall back to prompting with a warning if the credentials cannot be retrieved.
