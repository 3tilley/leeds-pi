use std::io::Write;
use std::ops::Rem;
use std::thread::sleep;
use std::time::Duration;
// use tokio::time::sleep;

struct JunkPacketRange {
    start: u64,
    end: Option<u64>,
}

impl Iterator for JunkPacketRange {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.start += 1;
        if let Some(e) = self.end {
            return None
        }
        let mod_ = self.start.rem(16) as u8;
        if (0..10).contains(&mod_) {
            Some(0)
        } else {
            Some(mod_)
        }
    }
}

pub struct JunkData {
    pub(crate) start: u64,
    pub(crate) end: Option<u64>,
    pub(crate) packet_size: u32,
}

impl DataProducer for JunkData {
    fn receive_packets(&self) -> Option<Vec<Vec<u8>>> {
        let mut generator = JunkPacketRange{ start: self.start, end: self.end };
        let mut vec = Vec::new();
        for d in generator {
            if vec.len() < self.packet_size as usize {
                vec.push(d)
            } else {
                break;
            }
            sleep(Duration::from_millis(100))

        }
        Some(vec![vec])
    }
}

fn junk_data(sleep_ms: u64) {
    for d in (JunkPacketRange { start: 0, end: Some(100) }) {
        sleep(Duration::from_millis(sleep_ms));

    }
}

pub trait DataProducer {
    fn receive_packets(&self) -> Option<Vec<Vec<u8>>>;
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
impl DataProducer for CC1101 {
    fn receive_packets(&self) -> Option<Vec<Vec<u8>>> {
        let data = self.receive().unwrap();
        if data.len() > 0 {
            Some(data)
        } else {
            None
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn init_cc1101(baud: f32, packet_size: u32) -> CC1101 {
    let rx_config = RXConfig::new(433.92, Modulation::OOK, baud, packet_size, Some(20.629883), None, None, None, None, None, None).unwrap();

    CC1101::new("/dev/cc1101.0.0", Some(rx_config), true).unwrap()
}

pub fn listen_and_record(producer: Box<dyn DataProducer>, file: &mut impl Write) {

    loop {
        let packets_opt = producer.receive_packets();
        if let Some(packets) = packets_opt {
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
    }
}
