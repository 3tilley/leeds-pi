use fmt::LowerHex;
use std::{fs, fmt};
use std::fs::OpenOptions;
use std::io::Write;
use cc1101::{Cc1101, RadioMode};

// use embedded_hal::digital::v2::OutputPin;
// use hal::prelude::*;
// use hal::spi::Spi;
// use hal::stm32;

use cc1101_rust::{
    config::{Modulation, TXConfig},
    CC1101Error, CC1101,
};
use cc1101_rust::config::RXConfig;

// type Packet = Vec<u8>;

const ONE: [u8; 16] = [
    0x55, 0x55, 0x55, 0x53, 0x33, 0x54, 0xd5, 0x35, 0x55, 0x54, 0xac, 0xad, 0x00, 0x00, 0x00, 0x00,
];

const THREE: [u8; 16] = [
    0x55, 0x55, 0x55, 0x53, 0x33, 0x54, 0xd5, 0x35, 0x55, 0x4c, 0xca, 0xb5, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug)]
struct AppArgs {
    packet_size: u32,
    baud: f32,
    gpio: u8,
}

// impl LowerHex for Packet {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if f.alternate() {
//             write!(f, "0x")?;
//         }
//         for &byte in self {
//             write!(f, "{:0>2x}", byte)?;
//         }
//         Ok(())
//     }
// }

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = AppArgs {
        // Parses a required value that implements `FromStr`.
        // Returns an error if not present.
        packet_size: pargs.opt_value_from_str("--packet-size")?.unwrap_or(1024),
        baud: pargs.opt_value_from_str("--baud")?.unwrap_or(9.6),
        gpio: pargs.opt_value_from_str("--gpio")?.unwrap_or(24),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn main() -> Result<(), CC1101Error> {
    let args = parse_args().unwrap();
    // let tx_config = TXConfig::new(
    //     433.92,
    //     Modulation::OOK,
    //     8.192,
    //     0.6,
    //     Some(20.629883),
    //     // Some(0xAAAB),
    //     None,
    // ).unwrap();
    let rx_config = RXConfig::new(433.92, Modulation::OOK, args.baud, args.packet_size, Some(20.629883), None, None, None, None, None, None).unwrap();

    let cc1101 = CC1101::new("/dev/cc1101.0.0", Some(rx_config), true).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data_rust.txt")
        .unwrap();

    loop {
        let packets = cc1101.receive().unwrap();
        let packets_text: String = packets.iter().map(|packet| {
            let mut text = String::new();
            for &p in packet {
                text.push_str(&format!("{:0>2x}", p));
            }
            text
        }).collect::<Vec<_>>().join("\n");
        println!("Received packet: {:?}", packets_text);

        if packets_text.len() > 0 {
            writeln!(file, "{}", packets_text).unwrap();
        }
    }

    // cc1101.transmit(&tx_config, &THREE)?;

    Ok(())
}
