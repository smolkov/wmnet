extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};
use structopt::StructOpt;

use std::path::{Path, PathBuf};
// use std::time::Duration;

extern crate ssd1675;
use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};

// Graphics
extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

// Font
extern crate profont;
use profont::{ProFont10Point, ProFont12Point, ProFont18Point, ProFont9Point};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::process::Command;
use wqms::iface::*;
use wqms::network;
use wqms::Result;
// use std::thread::sleep;
// use std::{fs, io};

// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

const ROWS: u16 = 212;
const COLS: u8 = 104;
// pub fn layout(path: Path) -> Result<EPaperMenu, std::io::Error> {}

#[rustfmt::skip]
const LUT: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUTT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    64,   12,   32,   12,    6,   // 0 Flash
    16,   8,    4,    4,     6,   // 1 clear
    4,    8,    8,    16,    16,  // 2 bring in the black
    2,    2,    2,    64,    32,  // 3 time for red
    2,    2,    2,    2,     2,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];
///Edinburgh sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "🧰edinburgh sensor interface interface usage.")]
pub struct Args {
    ///🗁 sensor working directory
    #[structopt(short = "d", long = "dir", default_value = ".wqms")]
    dir: PathBuf,
    ///🔌 hardware connection address
    #[structopt(short = "s", long = "spi", default_value = "/dev/spidev0.0")]
    spi: PathBuf,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn directory(&self) -> &Path {
        &self.dir
    }
    /// Access the directory name.
    #[inline]
    pub fn spi(&self) -> &Path {
        &self.spi
    }
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let ws = wqms::ws::setup()?;
    let mut spi = Spidev::open(args.spi()).expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat
    // Configure Digital I/O Pins
    let cs = Pin::new(8); // BCM8
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(17); // BCM17
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(22); // BCM22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let reset = Pin::new(27); // BCM27
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset
        .set_direction(Direction::Out)
        .expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");
    println!("Pins configured");

    // Initialise display controller
    let mut delay = Delay {};

    let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

    let mut black_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let config = Builder::new()
        .dimensions(Dimensions {
            rows: ROWS,
            cols: COLS,
        })
        .rotation(Rotation::Rotate270)
        .lut(&LUT)
        .build()
        .expect("invalid configuration");
    let display = Display::new(controller, config);
    let mut display = GraphicDisplay::new(display, &mut black_buffer, &mut red_buffer);
    println!("Clear");
    const VAL: i32 = 35;
    const NAME: i32 = 15;
    const CORD: [(i32, i32); 8] = [
        (1, 1),
        (50, 1),
        (100, 1),
        (150, 1),
        (1, 40),
        (50, 40),
        (100, 40),
        (150, 40),
    ];

    let (tx, rx) = std::sync::mpsc::channel();
    // let (tx, rx) = crossbeam_channel::unbounded();
    // let mut watcher: RecommendedWatcher = Watcher::with_channel(tx, Duration::from_secs(2))?;

    // for event in rx.iter() {
    // ...
    // }
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(ws.rootdir(), RecursiveMode::Recursive)
        .unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event);
                display.reset(&mut delay).expect("error resetting display");
                // println!("Reset and initialised");
                display.clear(Color::White);
                let chs = ws.channels().list()?;
                for (index, ch) in chs.iter().enumerate() {
                    if index < 8 {
                        let info = ch.info();
                        display.draw(
                            ProFont12Point::render_str(info.value.as_str())
                                .with_stroke(Some(Color::Red))
                                .with_fill(Some(Color::White))
                                .translate(Coord::new(CORD[index].0, CORD[index].1))
                                .into_iter(),
                        );
                        display.draw(
                            ProFont10Point::render_str(info.label.as_str())
                                .with_stroke(Some(Color::Black))
                                .with_fill(Some(Color::White))
                                .translate(Coord::new(CORD[index].0 + VAL, CORD[index].1 + NAME))
                                .into_iter(),
                        );
                        display.draw(
                            ProFont10Point::render_str(info.label.as_str())
                                .with_stroke(Some(Color::Black))
                                .with_fill(Some(Color::White))
                                .translate(Coord::new(CORD[index].0 + VAL, CORD[index].1 + NAME))
                                .into_iter(),
                        );
                    }
                }
                display.draw(
                    ProFont9Point::render_str(network::hostname().as_str().trim())
                        .with_stroke(Some(Color::Black))
                        .with_fill(Some(Color::White))
                        .translate(Coord::new(1, 93))
                        .into_iter(),
                );
                display.draw(
                    ProFont9Point::render_str(format!("{}", network::state()).trim())
                        .with_stroke(Some(Color::Black))
                        .with_fill(Some(Color::White))
                        .translate(Coord::new(1, 84))
                        .into_iter(),
                );
                display.update(&mut delay).expect("error updating display");
                println!("Update...");
                println!("Finished - going to sleep");
                display.deep_sleep().unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
