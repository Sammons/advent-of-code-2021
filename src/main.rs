use std::{
    collections::{HashMap, HashSet},
    convert,
    iter::FromFn,
    ops::DerefMut,
};

// 3 bits version
// 3 bits type id

// when type id = 4
// 5 bit chunks, leading with 1 until last chunk which leads with 0

// otherwise type id is operator
// bit 1 is length type
// 0 = 15 bits are the length of all sub packets
// 1 = 11 bits are the number of sub

struct Blob {
    grid: Vec<bool>, // would be u4
}

#[derive(Debug)]
struct ContentPacketValue {
    kind: usize,
    version: usize,
    content: Vec<usize>,
}

fn num_to_bits(_n: u8) ->Vec<bool> {
    let mut n = _n;
    let mut bits: Vec<bool> = vec![];
    for _ in 0..4 {
        let v: bool = if (1 & n) == 1 { true } else { false };
        bits.push(v);
        n = n >> 1;
    }
    bits.reverse();
    bits
}



#[derive(Debug)]
struct OperationPacketValue {
    kind: usize,
    version: usize,
    packets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    ContentPacket(ContentPacketValue),
    OperationPacket(OperationPacketValue),
}

impl ContentPacketValue {
    fn get_value(&self) -> i64 {
        let mut v: i64 = 0;
        for d in &self.content {
            v *= 10;
            v += *d as i64;
        }
        v
    }
}

impl OperationPacketValue {
    fn evaluate(&self) -> i64 {
        let sum = || {
            let mut s = 0;
            for p in &self.packets {
                s += p.evaluate();
            }
            s
        };
        let product = || {
            let mut s = 1;
            for p in &self.packets {
                s *= p.evaluate();
            }
            s
        };
        let min = || {
            let mut min = self.packets[0].evaluate();
            for p in self.packets.iter().skip(1) {
                let e = p.evaluate();
                if min > e {
                    min = e;
                }
            }
            min
        };
        let max = || {
            let mut max = self.packets[0].evaluate();
            for p in self.packets.iter().skip(1) {
                let e = p.evaluate();
                if max < e {
                    max = e;
                }
            }
            max
        };
        let gt = || {
            let first = self.packets[0].evaluate();
            let second = self.packets[1].evaluate();
            if first > second {
                1
            } else {
                0
            }
        };
        let lt = || {
            let first = self.packets[0].evaluate();
            let second = self.packets[1].evaluate();
            if first < second {
                1
            } else {
                0
            }
        };
        let eq = || {
            let first = self.packets[0].evaluate();
            let second = self.packets[1].evaluate();
            if first == second {
                1
            } else {
                0
            }
        };
        match self.kind {
            0 => sum(),
            1 => product(),
            2 => min(),
            3 => max(),
            5 => gt(),
            6 => lt(),
            7 => eq(),
            _ => panic!("invalid kind")
        }
    }
}

impl Packet {
    fn evaluate(&self) -> i64 {
        match self {
            Packet::ContentPacket(value) => {
                value.get_value()
            },
            Packet::OperationPacket(value) => {
                value.evaluate()
            }
        }
    }
}

impl Blob {
    fn from_file(path: &str) -> Result<Blob, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        let decode_hex = |c: char| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            _ => panic!("invalid hex char"),
        };
        
        // println!("bits {:?}", num_to_bits(11));
        let grid = lines[0]
            .chars()
            .flat_map(|c| num_to_bits(decode_hex(c)))
            .collect();
        Ok(Blob { grid })
    }
    fn print(&self) {
        println!("");
    }

    // 3 bits version
    // 3 bits type id

    // when type id = 4
    // 5 bit chunks, leading with 1 until last chunk which leads with 0

    // otherwise type id is operator
    // bit 1 is length type
    // 0 = 15 bits are the length of all sub packets
    // 1 = 11 bits are the number of sub
    fn parse_packets(&self) -> (usize, Vec<Packet>) {
        let mut packets = vec![];
        let mut offset: usize = 0;
        let mut version_sum = 0;
        while self.grid.len() - offset > 11 {
            let (new_offset, vs, packet) = self.parse_one_packet(offset, self.grid.len());
            version_sum += vs;
            offset = new_offset;
            packets.push(packet);
        }
        (version_sum, packets)
    }
    fn convert_bits_into_num(bits: &Vec<bool>, start: usize, end: usize) -> usize {
        let mut num: usize = 0;
        for idx in start..end {
            let v: usize = if bits[idx] { 1 } else { 0 };
            num = num | v;
            num = num << 1
        }
        num >>= 1;
        // print!("-> {}\n", num);
        num
    }
    fn parse_one_packet(&self, start_idx: usize, boundary: usize) -> (usize, usize, Packet) {
        let version = Blob::convert_bits_into_num(&self.grid, start_idx, start_idx + 3);
        let kind = Blob::convert_bits_into_num(&self.grid, start_idx + 3, start_idx + 6);
        let mut offset = start_idx + 6;
        if kind == 4 {
            // content
            let mut content = vec![];
            let mut bit_segments: Vec<bool> = vec![];
            loop {
                let leading_bit = self.grid[offset];
                // content.push(Blob::convert_bits_into_num(offset + 1, offset + 5));
                for i in offset + 1 .. offset + 5 {
                    bit_segments.push(self.grid[i]);
                }
                offset += 5;
                if leading_bit == false {
                    break;
                }
            }
            content.push(Blob::convert_bits_into_num(&bit_segments, 0, bit_segments.len()));
            (
                offset,
                version,
                Packet::ContentPacket(ContentPacketValue {
                    content,
                    kind: 4,
                    version,
                }),
            )
        } else {
            // operator
            let length_type = self.grid[offset];
            offset += 1;
            let mut sub_packets = vec![];
            if length_type == true {
                // 11 bits represent number of packets
                let number_of_packets = Blob::convert_bits_into_num(&self.grid, offset, offset + 11);
                offset += 11;
                let mut version_sum = version;
                for _ in 0..number_of_packets {
                    let (new_offset, vs, packet) = self.parse_one_packet(offset, self.grid.len());
                    version_sum += vs;
                    sub_packets.push(packet);
                    offset = new_offset;
                }
                (
                    offset,
                    version_sum,
                    Packet::OperationPacket(OperationPacketValue {
                        kind,
                        version,
                        packets: sub_packets,
                    }),
                )
            } else {
                // 15 bits represent size of sub-packets
                let length_of_packets = Blob::convert_bits_into_num(&self.grid, offset, offset + 15);
                offset += 15;
                let boundary = offset + length_of_packets;
                let mut version_sum = version;
                while offset < boundary {
                    let (new_offset, vs, packet) = self.parse_one_packet(offset, boundary);
                    version_sum += vs;
                    if new_offset <= boundary {
                        sub_packets.push(packet);
                        offset = new_offset;
                    } else {
                        panic!("not clear, prev {} new {}, boundary {}", offset, new_offset, boundary);
                    }
                }
                (
                    offset,
                    version_sum,
                    Packet::OperationPacket(OperationPacketValue {
                        kind,
                        version,
                        packets: sub_packets,
                    }),
                )
            }
        }
    }
}

fn main() {
    let input_res = Blob::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            let (version_sum, packets) = input.parse_packets();
            let result = packets[0].evaluate();
            println!("{}", version_sum);
            // 8749379669
            println!("{}", result);
            // println!("{} top packets of kind {:?}", packets.len(), packets[0]);
        }
        Err(e) => println!("{}", e),
    }
}
