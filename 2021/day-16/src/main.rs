use std::io;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Frame {
    state: u8,
    version: u32,
    opcode: u32,
    acc: u32,
    size: u32,
    packets: Vec<Frame>
}

fn main () {
    // let entry: Vec<char> = "D2FE2800000".chars().collect();
    let entry: Vec<char> = "38006F45291200".chars().collect();
    // let entry: Vec<char> = "EE00D40C823060".chars().collect();
    // let entry: Vec<char> = "8A004A801A8002F478".chars().collect();

    let mut tape: u32 = 0;
    let mut carry: u32 = 0;

    // Run read loop

    let mut iterator = entry.iter();
    
    // while ingest(&mut iterator, &mut tape, &mut carry) {
    //     step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    // }

    read_packet(&mut iterator, &mut tape, &mut carry);

    // while carry > 0 {
    //     println!("{:?}", frame);
    //     println!("{}", carry);

    //     step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    // }

    // println!("{:?}", stack);
    // println!("{:?}", packets);

}

fn ingest<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u32, carry: &mut u32) -> bool {
    match it.next() {
        Some(c) => {
            // Each read operation adds 4 bits to tape.
            *tape <<= 4;
            *tape |= decode(*c);
            *carry += 4;
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn read_packet<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u32, carry: &mut u32) -> Frame {

    let mut frame = Frame{ state: 0, version: 0, opcode: 0, acc: 0, size: 0, packets: vec![]};

    let mut finished = false;

    while ! finished {
        match step(it, tape, carry, &mut frame) {
            Some(p) => {
                println!("{:?}", p);
                finished = true;
                break;
            },
            _ => { 
                match ingest(it, tape, carry) {
                    true => {},
                    false => {
                        // If already finished ingesting keep adding padding.
                        *tape <<= 1;
                        *carry += 1;
                    }
                }
            }
        }
    }

    return frame;
}

fn step<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u32, carry: &mut u32, frame: &mut Frame) -> Option<Frame> {
    match frame.state {
        0 => {
            // read version
            match read(3, tape, carry, frame) {
                Some(v) => {
                    frame.version = v;
                    frame.state = 1;
                },
                _ => (),
            }
        },
        1 => {
            // read opcode
            match read(3, tape, carry, frame) {
                Some(op) => {
                    frame.opcode = op;
                    frame.state = 2;
                },
                _ => (),
            }
        },
        2 => {
            match frame.opcode {
                4 => {
                    if *carry >= 5 {
                        let block_end = read(1, tape, carry, frame).unwrap();
                        
                        frame.acc <<= 4;
                        frame.acc |= read(4, tape, carry, frame).unwrap();

                        if block_end == 0 {
                            // finished reading literal
                            frame.state = 3;
                        }
                    }
                },

                _ => {

                    if *carry >= 16 {

                        let length_type_id = read(1, tape, carry, frame).unwrap();

                        if length_type_id == 1 {
                            // number of sub-packets immediately contained
                            let number_of_packets = read(11, tape, carry, frame).unwrap();

                            // Read X packets into this one
                            for nr_p in 0..number_of_packets {
                                frame.packets.push(read_packet(it, tape, carry));
                            }

                            // finish this packet
                            frame.state = 3;

                        } else {
                            // total length in bits
                            let total_packet_length = read(15, tape, carry, frame).unwrap() as i32;

                            // Read packets until size is hit
                            let mut packets_size = 0;
                            while packets_size < total_packet_length as u32 {
                                let p = read_packet(it, tape, carry);
                                packets_size += p.size;
                                frame.packets.push(p);
                            }
                            frame.size += packets_size;
                            // finish this packet
                            frame.state = 3;
                        }
                    }

                },
                
            }
        },
        3 => {
            // Finished packet
            println!("Packet:");
            println!("  version: {:b}",frame.version);
            println!("  opcode: {}",frame.opcode);

            match frame.opcode {
                4 => {
                    println!("  literal value: {}",frame.acc);
                }
                _ => {}
            }

            return Some(frame.clone());
        }
        _ => {}
    }

    return None;
}

fn read(nr_bytes: u32, tape: &mut u32, carry: &mut u32, frame: &mut Frame) -> Option<u32> {

    if nr_bytes <= *carry {

        frame.size += nr_bytes;

        let result: u32 = *tape >> *carry - nr_bytes;
        
        let mask: u32 = u32::MAX << *carry - nr_bytes;
        
        *tape &= !mask;
        *carry -= nr_bytes;

        return Some(result);
    }

    return None;
}

fn decode(c: char) -> u32 {

    match c {
        '0' => 0b0000,
        '1' => 0b0001,
        '2' => 0b0010,
        '3' => 0b0011,
        '4' => 0b0100,
        '5' => 0b0101,
        '6' => 0b0110,
        '7' => 0b0111,
        '8' => 0b1000,
        '9' => 0b1001,
        'A' => 0b1010,
        'B' => 0b1011,
        'C' => 0b1100,
        'D' => 0b1101,
        'E' => 0b1110,
        'F' => 0b1111,
        _ => panic!("Can't decode character: {}.", c)
    }
    
}
