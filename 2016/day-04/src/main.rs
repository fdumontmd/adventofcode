use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct LetterCount (i32, char);

// return the sector if the room identifier is valid
fn get_room_sector(room: &str) -> Option<u32> {
    let mut iter = room.split("[");
    let code_sector = iter.next().unwrap();
    let checksum = iter.next().unwrap();
    let checksum = &checksum[..checksum.len()-1];

    if checksum.len() != 5 {
        return None;
    }

    let mut sector = 0;
    let mut letters = HashMap::new();

    for chunk in code_sector.split("-") {
        match u32::from_str(chunk) {
            Ok(num) => sector = num,
            Err(_) => {
                for c in chunk.chars() {
                    letters.entry(c).or_insert(LetterCount(0, c)).0 -= 1;
                }
            }
        }
    }

    let mut letters: Vec<LetterCount> = letters.values().cloned().collect();

    if letters.len() < 5 {
        return None;
    }

    letters.sort();

    for (lc, c) in letters.iter().zip(checksum.chars()) {
        if lc.1 != c {
            return None
        }
    }

    return Some(sector);
}

fn decrypt_room(room_name: &str, code: u32) -> String {
    let mut iter = room_name.split("[");
    let name = iter.next().unwrap();

    let mut real_name = String::new();

    for chunk in name.split("-") {
        if chunk == code.to_string() {
            continue;
        }

        for c in chunk.chars() {
            let c = c as u8 - 97;
            let c = (97 + (c as u32 + code ) % 26) as u8;
            real_name.push(c as u8 as char);
        }
        real_name.push(' ');
    }
    real_name
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let mut sector_sum = 0;

    for line in buffer.lines() {
        if let Some(sector) = get_room_sector(line) {
            sector_sum += sector;

            let real_name = decrypt_room(line, sector);
            if real_name.starts_with("north") {
                println!("real name: {} - {}", real_name, sector);
            }
        }
    }

    println!("Total sum of sector numbers: {}", sector_sum);
}

#[test]
fn basic_tests() {
    assert!(get_room_sector("aaaaa-bbb-z-y-x-123[abxyz]") == Some(123));
    assert!(get_room_sector("a-b-c-d-e-f-g-h-987[abcde]") == Some(987));
    assert!(get_room_sector("not-a-real-room-404[oarel]") == Some(404));
    assert!(get_room_sector("totally-real-room-200[decoy]") == None);
}
