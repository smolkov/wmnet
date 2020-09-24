extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};
extern crate ssd1675;
use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};
use wqms::channel::ShortInfo;
// Graphics
#[macro_use]
extern crate embedded_graphics;
use embedded_graphics::prelude::*;

// Font
extern crate profont;
use profont::{ProFont14Point,ProFont18Point,ProFont10Point, ProFont24Point, ProFont9Point};

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
// use std::{fs, io};
use wqms::ws::Workspace;
use wqms::iface::Class;
// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

const ROWS: u16 = 212;
const COLS: u8 = 104;

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

pub struct State {
    ws: wqms::ws::Workspace,
    chan: Vec<ShortInfo>,
    online: String,
    status: String,
    host: String,
    error: String,
    changed: bool,
    last: std::time::Instant,
}

pub fn new_state(ws:Workspace) -> State {
    State{
        ws:ws,
        chan: vec!{
                ShortInfo::new("TOX","NIL"),
                ShortInfo::new("DOS","NIL"),
                ShortInfo::new("ph","NIL"),
                ShortInfo::new("ec","NIL"),
                ShortInfo::new("orp","NIL"),
                ShortInfo::new("temp","NIL"),
                ShortInfo::new("tur","NIL"),
            },
        online:"offline".to_owned(),
        status:"-------".to_owned(),
        host: "none".to_owned(),
        error: "".to_owned(),
        changed: false,
        last : std::time::Instant::now(),
    }
}
impl State {
    pub fn changed(&mut self) {
       self.changed = true;
       self.last = std::time::Instant::now();
    }
    pub fn is_changed(&self) -> bool {
        self.changed
    }
    pub fn update(&mut self) -> wqms::Result<()> {
        self.error = "".to_owned();
        self.changed = false;
        let online = wqms::network::status();
        let host   = wqms::network::hostname();
        if self.online != online {
            self.online = online;
            self.changed();
        }
        if self.host != host {
            self.host= host;
            self.changed();
        }
        let status = self.ws.channels()?.status();
        if self.status != status {
            self.status = status;
            self.changed();
        }
        let list = self.ws.channels()?.infos()?;
        let count = list.iter().zip(&self.chan).filter(|&(a, b)| a.value != b.value).filter(|&(a,b)| a.label != b.label).count();
        if  count > 0  {
            self.chan = list.iter().cloned().collect();
            self.changed();
        }
        if !self.is_changed() {
            if self.last.elapsed().as_secs() > 600 {
                self.changed();
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    let ws = wqms::ws::default();
    wqms::logger::debug();
    let mut state = new_state(wqms::ws::default());
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat
    // Configure Digital I/O Pins
    let cs = Pin::new(24); // BCM8
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
    const VAL: i32 = 20;
    const CORD: [(i32, i32); 8] = [
        (1, 1),
        (54, 1),
        (107, 1),
        (160, 1),
        (1, 40),
        (54, 40),
        (107, 40),
        (160, 40),
    ];
    // Main loop. Displays CPU temperature, uname, and uptime every minute with a red Raspberry Pi
    // header.
    loop {
        let wait = Duration::from_secs(10);
        if let Err(e) = state.update() {
            log::error!("inky update state failed {}",e);
            state.error= "update failed".to_owned();
            state.changed();
        }
        if state.is_changed(){
            display.reset(&mut delay).expect("error resetting display");
            display.clear(Color::White);
            let chv = ws.channels().unwrap();
            let mut index = 0;
            for  ch in chv.list.iter() {
                if index < 8 {
                    let mut value = ch.value();
                    value.truncate(5);
                    // log::info!("{} {}[{}]",ch.value,ch.label,ch.unit); 
                    egtext!(
                        text = &format!("{}",value.trim()),
                        top_left = (CORD[index].0, CORD[index].1),
                        style = text_style!(
                            font = ProFont14Point,
                            background_color = Color::White,
                            text_color = Color::Red,
                        )
                    )
                    .draw(&mut display)
                    .expect("error drawing text");
                    egtext!(
                        text = &format!("{} {}", ch.label().as_str().trim(),ch.status().trim()),
                        top_left = (CORD[index].0, CORD[index].1+ VAL),
                        style = text_style!(
                            font = ProFont10Point,
                            background_color = Color::White,
                            text_color = Color::Black,
                        )
                    )
                    .draw(&mut display)
                    .expect("error drawing text");
                    index+=1;
                    // display.draw(
                    //     ProFont18Point::render_str(ch.value().as_str())
                    //         .with_stroke(Some(Color::Red))
                    //         .with_fill(Some(Color::White))
                    //         .translate(Coord::new(CORD[index].0, CORD[index].1))
                    //         .into_iter(),
                    // );
                    // display.draw(
                    //     ProFont10Point::render_str(ch.label().as_str())
                    //         .with_stroke(Some(Color::Black))
                    //         .with_fill(Some(Color::White))
                    //         .translate(Coord::new(CORD[index].0, CORD[index].1 + VAL))
                    //         .into_iter(),
                    // );
                }
            }
            egtext!(
                text = &format!("status: {}",chv.status().trim()),
                top_left = (1, 73),
                style = text_style!(
                    font = ProFont9Point,
                    background_color = Color::White,
                    text_color = Color::Red,
                )
            )
            .draw(&mut display)
            .expect("error drawing text");
            egtext!(
                text = &format!("{}", state.online.trim()),
                top_left = (10, 83),
                style = text_style!(
                    font = ProFont9Point,
                    background_color = Color::White,
                    text_color = Color::Red,
                )
            )
            .draw(&mut display)
            .expect("error drawing text");

            egtext!(
                text = &format!("ip: {}",state.host.trim()),
                top_left = (10, 93),
                style = text_style!(
                    font = ProFont9Point,
                    background_color = Color::White,
                    text_color = Color::Black,
                )
            )
            .draw(&mut display)
            .expect("error drawing text");
           
            if let Some(uptime) = read_uptime() {
                egtext!(
                    text = uptime.trim(),
                    top_left = (120, 83),
                    style = text_style!(
                        font = ProFont9Point,
                        background_color = Color::White,
                        text_color = Color::Black,
                    )
                )
                .draw(&mut display)
                .expect("error drawing text");
            }


            display.update(&mut delay).expect("error updating display");
            println!("Update...");

            println!("Finished - going to sleep");
            display.deep_sleep()?;
        }
        sleep(wait);
    }
}

// fn read_cpu_temp() -> Result<f64, io::Error> {
//     fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
//         .trim()
//         .parse::<i32>()
//         .map(|temp| temp as f64 / 1000.)
//         .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
// }

fn read_uptime() -> Option<String> {
    Command::new("uptime")
        .arg("-p")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
}

// fn read_uname() -> Option<String> {
//     Command::new("uname")
//         .arg("-smr")
//         .output()
//         .ok()
//         .and_then(|output| {
//             if output.status.success() {
//                 String::from_utf8(output.stdout).ok()
//             } else {
//                 None
//             }
//         })
// }
