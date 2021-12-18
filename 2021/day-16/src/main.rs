use std::io;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Frame {
    state: u8,
    version: u32,
    opcode: u32,
    acc: u32,
    limit: i32,
    limit_packets: i32,
    size: u32,
    packets: Vec<Frame>
}

fn main () {
    // let entry: Vec<char> = "D2FE2800000".chars().collect();
    let entry: Vec<char> = "38006F4529120000000000000000000000000000000000000000000".chars().collect();
    // let entry: Vec<char> = "EE00D40C823060".chars().collect();
    // let entry: Vec<char> = "8A004A801A8002F478".chars().collect();

    let mut tape: u32 = 0;
    let mut carry: u32 = 0;

    let mut packets: Vec<Frame> = Vec::new();
    let mut stack: VecDeque<Frame> = VecDeque::new();

    let mut frame = Frame{ state: 0, version: 0, opcode: 0, acc: 0, size: 0, limit: -1, limit_packets: -1, packets: vec![]};

    // Run read loop

    let mut iterator = entry.iter();
    
    while ingest(&mut iterator, &mut tape, &mut carry) {
        step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    }

    // while carry > 0 {
    //     println!("{:?}", frame);
    //     println!("{}", carry);

    //     step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    // }

    println!("{:?}", stack);
    println!("{:?}", packets);

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

fn step(tape: &mut u32, carry: &mut u32, frame: &mut Frame, stack: &mut VecDeque<Frame>, packets: &mut Vec<Frame>) {
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
                        frame.acc |= read(4, tape, carry, frame).unwrap();
                        if block_end == 0 {
                            // finished reading literal
                            frame.state = 3;
                        } else {
                            frame.acc <<= 4;
                        }
                    }
                },

                _ => {

                    if *carry >= 16 {
                        let length_type_id = read(1, tape, carry, frame).unwrap();
                        if length_type_id == 1 {
                            // number of sub-packets immediately contained
                            let number_of_packets = read(11, tape, carry, frame).unwrap();
                            frame.limit_packets = number_of_packets as i32;

                            stack.push_back(frame.clone());
                            *frame = Frame{ state: 0, version: 0, opcode: 0, acc: 0, size: 0, limit: -1, limit_packets: -1, packets: vec![]};
                        } else {
                            // total length in bits
                            let total_packet_length = read(15, tape, carry, frame).unwrap() as i32;
                            frame.limit = total_packet_length;

                            stack.push_back(frame.clone());
                            *frame = Frame{ state: 0, version: 0, opcode: 0, acc: 0, size: 0, limit: -1, limit_packets: -1, packets: vec![]};
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
            
            // Finished packet.
            if ! stack.is_empty() { 
                let back = stack.back_mut().unwrap();
                let size = frame.size;
                back.packets.push(frame.clone());

                back.limit -= size as i32;

                if back.limit == 0 {
                    packets.push(stack.pop_back().unwrap());
                } else {
                    if back.limit_packets > 0 && back.limit_packets == back.packets.len() as i32  {
                        packets.push(stack.pop_back().unwrap()); 
                    }
                }

            } else {
                packets.push(frame.clone());
            }

            // New frame
            *frame = Frame{ state: 0, version: 0, opcode: 0, acc: 0, size: 0, limit: -1, limit_packets: -1, packets: vec![]};
        }
        _ => {}
    }
}

fn read(nr_bytes: u32, tape: &mut u32, carry: &mut u32, frame: &mut Frame) -> Option<u32> {

    if nr_bytes <= *carry {

        if frame.limit != -1 {
            if frame.limit >= nr_bytes as i32 {
                frame.limit -= nr_bytes as i32;
            } else {
                return None
            }
        }

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
