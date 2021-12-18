use std::io;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Frame {
    state: u8,
    version: u64,
    opcode: u64,
    acc: u64,
    size: u64,
    packets: Vec<Frame>
}

fn main () {
    // let entry: Vec<char> = "D2FE2800000".chars().collect();
    // let entry: Vec<char> = "38006F45291200".chars().collect();
    // let entry: Vec<char> = "EE00D40C823060".chars().collect();
    // let entry: Vec<char> = "8A004A801A8002F478".chars().collect();
    let entry: Vec<char> = "8A004A801A8002F478".chars().collect(); // 16
    let entry: Vec<char> = "620080001611562C8802118E34".chars().collect(); // 12
    let entry: Vec<char> = "C0015000016115A2E0802F182340".chars().collect(); // 23
    let entry: Vec<char> = "A0016C880162017C3686B18A3D4780".chars().collect(); // 31
    let entry: Vec<char> = "E20D7880532D4E551A5791BD7B8C964C1548CB3EC1FCA41CC00C6D50024400C202A65C00C20257C008AF70024C00810039C00C3002D400A300258040F200D6040093002CC0084003FA52DB8134DE620EC01DECC4C8A5B55E204B6610189F87BDD3B30052C01493E2DC9F1724B3C1F8DC801E249E8D66C564715589BCCF08B23CA1A00039D35FD6AC5727801500260B8801F253D467BFF99C40182004223B4458D2600E42C82D07CC01D83F0521C180273D5C8EE802B29F7C9DA1DCACD1D802469FF57558D6A65372113005E4DB25CF8C0209B329D0D996C92605009A637D299AEF06622CE4F1D7560141A52BC6D91C73CD732153BF862F39BA49E6BA8C438C010E009AA6B75EF7EE53BBAC244933A48600B025AD7C074FEB901599A49808008398142013426BD06FA00D540010C87F0CA29880370E21D42294A6E3BCF0A080324A006824E3FCBE4A782E7F356A5006A587A56D3699CF2F4FD6DF60862600BF802F25B4E96BDD26049802333EB7DDB401795FC36BD26A860094E176006A0200FC4B8790B4001098A50A61748D2DEDDF4C6200F4B6FE1F1665BED44015ACC055802B23BD87C8EF61E600B4D6BAD5800AA4E5C8672E4E401D0CC89F802D298F6A317894C7B518BE4772013C2803710004261EC318B800084C7288509E56FD6430052482340128FB37286F9194EE3D31FA43BACAF2802B12A7B83E4017E4E755E801A2942A9FCE757093005A6D1F803561007A17C3B8EE0008442085D1E8C0109E3BC00CDE4BFED737A90DC97FDAE6F521B97B4619BE17CC01D94489E1C9623000F924A7C8C77EA61E6679F7398159DE7D84C015A0040670765D5A52D060200C92801CA8A531194E98DA3CCF8C8C017C00416703665A2141008CF34EF8019A080390962841C1007217C5587E60164F81C9A5CE0E4AA549223002E32BDCEA36B2E100A160008747D8B705C001098DB13A388803F1AE304600".chars().collect(); // 979

    let mut tape: u64 = 0;
    let mut carry: u64 = 0;

    // Run read loop

    let mut iterator = entry.iter();
    
    // while ingest(&mut iterator, &mut tape, &mut carry) {
    //     step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    // }

    let p = read_packet(&mut iterator, &mut tape, &mut carry);

    // while carry > 0 {
    //     println!("{:?}", frame);
    //     println!("{}", carry);

    //     step(&mut tape, &mut carry, &mut frame, &mut stack, &mut packets);
    // }

    // println!("{:?}", stack);
    // println!("{:?}", packets);

    println!("Version Checksum: {}", version_checksum(p));
}

fn version_checksum(frame: Frame) -> u64 {
    let mut acc = frame.version;

    for p in frame.packets {
        acc += version_checksum(p)
    }

    return acc;
}

fn ingest<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u64, carry: &mut u64) -> bool {
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

fn read_packet<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u64, carry: &mut u64) -> Frame {

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

fn step<'a>(it: &mut impl Iterator<Item = &'a char>, tape: &mut u64, carry: &mut u64, frame: &mut Frame) -> Option<Frame> {
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
                            while packets_size < total_packet_length as u64 {
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
            println!("  version: {} = {:b}",frame.version, frame.version);
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

fn read(nr_bytes: u64, tape: &mut u64, carry: &mut u64, frame: &mut Frame) -> Option<u64> {

    if nr_bytes <= *carry {

        frame.size += nr_bytes;

        let result: u64 = *tape >> *carry - nr_bytes;
        
        let mask: u64 = u64::MAX << *carry - nr_bytes;
        
        *tape &= !mask;
        *carry -= nr_bytes;

        return Some(result);
    }

    return None;
}

fn decode(c: char) -> u64 {

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
