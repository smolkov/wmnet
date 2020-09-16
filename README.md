# `wqms`

🚧 _Work In Progress_ 🚧

![Minimum Rust Version][min-rust-badge]
![pipeline status](https://travis-ci.org/smolkov/iotnode.svg?branch=master)

## 🎙️ Commands `wqms-cli`

- ### 🔩 `setup`

- ### 🔩 `list`

- ### 🔬️ `get`
  show property
  - `prop`: property full name with path

* ### 🔧 `set`

      - `prop`: property patch
      - `value`: value to set

```
wqms set network/interface wlan0
```

⚙

- ### 🕵️‍♀️ `check`: run this command to confirm that your configuration is appropriately set up.

## Building

1. Install `cargo`:

   Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

   ```
   curl https://sh.rustup.rs -sSf | sh
   ```

   Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).

2. Install on linux:

- ### `i686`
  Step 1: Install the C cross toolchain
  ```sh
    sudo apt-get install -qq gcc-multilib-i686-linux-gnu
  ```
- ### `arm`

  Step 1: Install the C cross toolchain

  ```sh
    sudo apt-get install -qq gcc-arm-linux-gnueabihf
  ```

  Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).
  Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.

  To cross-compile for the PanelPC you will need an
  `i686-unknown-linux-gnu` GCC toolchain and Rust component installed. Add the Rust target
  with `rustup target add i686-unknown-linux-gnu`. Then you can
  cross-compile with `cargo`:

  ```
      cargo build --release --target i686-unknown-linux-gnu
  ```

  or arm:

  ```
      cargo build --release --target arm-unknown-linux-gnueabihf
  ```

To cross-compile for the Raspberry Pi you will need an
`arm-unknown-linux-gnueabihf` GCC toolchain and Rust component installed. On
Arch Linux I built [arm-linux-gnueabihf-gcc] from the AUR. Add the Rust target
with `rustup target add arm-unknown-linux-gnueabihf`. Then you can
cross-compile with `cargo`:

After it is built copy `target/arm-unknown-linux-gnueabihf/release/lca2019` to
the Raspberry Pi.

## Running

View the options with `./lca2019 -h`. By default it will try to bind the
webserver to port 80. You can give a regular user the permission to do this
with:

    sudo setcap cap_net_bind_service=ep lca2019

Alternatively use `-p` to set the port to a non-privileged one.

### Systemd Service

Copy `wqms.service` to `/etc/systemd/system/`.
Copy `ngrok.service` to `/etc/systemd/system/`.

    sudo systemctl daemon-reload
    sudo systemctl enable ngrok.service
    sudo systemctl enable --now wqms.service
    sudo systemctl enable wqmsbot.service
    sudo 
<!-- Badges -->

[issue]: https://img.shields.io/github/issues/smolkov/iotnode?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg
