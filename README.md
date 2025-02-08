# ESP32 + I2C + SSD1306

The project is to connect the esp 32 to the ssd1306 OLED display via i2c and output a gif image.

<div style="display: flex; justify-content: center;">
<img src="./assets/preview.gif" width=30% height=30%/>
</div>

---

## Build and Run

1) First, make sure the following are installed:

   [Rust](https://www.rust-lang.org/tools/install) 🦀

4) Clone the github repo:

  ```sh
  git clone https://github.com/killpop3770/esp32_ssd1306_i2c.git
  cd esp32_ssd1306_i2c
  ```

3) Follow the book:

   [ESP Book](https://docs.esp-rs.org/book/) 📙

4) Install the proper toolchain with the rust-src component.

   For **no_std** (bare-metal) applications, you can use:

```sh
rustup toolchain install stable --component rust-src
```

5) Install espup:

```sh
cargo install espup
```

6) Install Necessary Toolchains

```sh
espup install
```

7) Assemble the board and peripherals and then run:

```sh
cargo run
```


## Acknowledgements

- [TinyGif](https://github.com/andelf/tinygif)
- [ESP32-hal](https://github.com/esp-rs/esp-hal)
- [SSD1306](https://github.com/rust-embedded-community/ssd1306)
