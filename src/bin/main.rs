#![no_std]
#![no_main]

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use esp_hal::gpio::Level;
use esp_hal::i2c::master::Config;
use esp_hal::{clock::CpuClock, gpio::Output};
use esp_hal::main;
use esp_println::println;
use fugit::HertzU32;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use embedded_graphics::Drawable;
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::draw_target::DrawTargetExt;
use ssd1306::mode::DisplayConfig;
use {defmt_rtt as _, esp_backtrace as _};

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let pin_d2 = Output::new(peripherals.GPIO2, Level::Low);
    let pin_d4 = Output::new(peripherals.GPIO4, Level::Low);

    let i2c_config = Config::default().with_frequency(HertzU32::kHz(400));
    let i2c = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config).unwrap()
    .with_scl(pin_d2)
    .with_sda(pin_d4);
    
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let delay = esp_hal::delay::Delay::new();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../../assets/rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();
    display.flush().unwrap();
    
    delay.delay(esp_hal::time::Duration::millis(2000));
    display.clear_buffer();

    let image = tinygif::Gif::<BinaryColor>::from_slice(include_bytes!("../../assets/badapple_full.gif")).unwrap();
    loop {
        for frame in image.frames() {
            frame.draw(&mut display.translated(Point::new(20, 0))).unwrap();
            delay.delay(esp_hal::time::Duration::millis(100));
            println!("blink!");
            display.flush().unwrap();
        }
    }
}