# Tina - "The Last 10-Second" protocol-based EEW client


## Description

Tina is an EEW client based on "The Last 10-Second" protocol.
You can make your mini PC such as Intel NUC or Raspberry Pi into an EEW receiving terminal with this software.

**Currently this software is under heavy development.**

- EEW (Earthquake Early Warning):
Earthquake alerts issued by Japan Meteorological Agency immediately after an earthquake occurred
- "The Last 10-Second":
A GUI-based EEW client software for Windows released by Weathernews Inc. (WNI)


## Features

- Receive EEW information from WNI
- Post EEW information to some web services
    - Twitter
    - Slack
    - PushBullet (Not Implemented)
- Launch other programs in response to EEW information (Not Implemented)


## How to Build

### Rust Toolchain

This software is written in Rust. You should get the Rust toolchain from [www.rustup.rs](https://www.rustup.rs/).

### OpenSSL

Currently this software depends on OpenSSL. You should install related files by yourself.

On Ubuntu 16.04, running the following command is sufficient.

```sh
$ apt install libssl-dev
```

### Build

```sh
$ git clone [path_to_this_repository] tina
$ cd tina
$ cargo build --release
```


## Run

Some configurations are needed. The sample configuration file is at `config/tina.yaml.example`.

Once the configuration is finished, you can run the software with the following command.

```sh
$ ./tina [path_to_config_file]
```


## Configuration

(TODO)


## Miscellaneous

- Author: wafrelka
- License: MIT License
