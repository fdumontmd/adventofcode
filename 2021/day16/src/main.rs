use anyhow::{bail, Error};

static INPUT: &str = include_str!("input.txt");

fn decode_to_binary(input: &str) -> impl Iterator<Item = u64> + '_ {
    let input = input.trim();
    input
        .bytes()
        .map(|b| {
            if b.is_ascii_digit() {
                (b - b'0') as u64
            } else {
                assert!((b'A'..=b'F').contains(&b));
                (b - b'A') as u64 + 10
            }
        })
        .flat_map(|d| {
            [
                (d & (1 << 3)) >> 3,
                (d & (1 << 2)) >> 2,
                (d & 2) >> 1,
                d & 1,
            ]
            .into_iter()
        })
}

fn decode_n_bits<I: Iterator<Item = u64>>(i: &mut I, n: usize) -> Result<u64, Error> {
    let mut res = 0;

    for _ in 0..n {
        let Some(d) = i.next() else { bail!("not enough bits")};
        res <<= 1;
        res += d;
    }
    Ok(res)
}

fn decode_literal_value<I: Iterator<Item = u64>>(i: &mut I) -> Result<u64, Error> {
    let mut res = 0;

    loop {
        let g = decode_n_bits(i, 5)?;
        res <<= 4;
        res += g & 0x0F;

        if g & (1 << 4) == 0 {
            break;
        }
    }

    Ok(res)
}

fn consume_padding<I: Iterator<Item = u64>>(i: &mut I) -> Result<(), Error> {
    for b in i {
        if b != 0 {
            bail!("Non-zero padding found");
        }
    }
    Ok(())
}

fn take_n_bits<I: Iterator<Item = u64>>(
    i: &mut I,
    n: usize,
) -> Result<impl Iterator<Item = u64>, Error> {
    let bits: Vec<u64> = i.take(n).collect();
    if bits.len() < n {
        bail!("Not enough bits {} instead of {n}", bits.len());
    } else {
        Ok(bits.into_iter())
    }
}

enum PacketType {
    Literal(u64),
    Operator(u64, bool, usize, Vec<Packet>),
}

struct Packet(u64, PacketType);

impl Packet {
    fn decode_from_hexadecimal(input: &str) -> Result<Self, Error> {
        let mut bits = decode_to_binary(input);
        let packet = Packet::decode(&mut bits)?;
        consume_padding(&mut bits)?;
        Ok(packet)
    }
    fn decode<I: Iterator<Item = u64>>(i: &mut I) -> Result<Self, Error> {
        let version = decode_n_bits(i, 3)?;
        Ok(Packet(version, PacketType::decode(i)?))
    }

    fn version_sum(&self) -> u64 {
        self.0 + self.1.sum_of_versions()
    }

    fn value(&self) -> u64 {
        self.1.value()
    }
}

impl PacketType {
    fn decode<I: Iterator<Item = u64>>(i: &mut I) -> Result<Self, Error> {
        let packet_type = decode_n_bits(i, 3)?;
        if packet_type == 4 {
            let literal = decode_literal_value(i)?;
            Ok(PacketType::Literal(literal))
        } else {
            let length_type = decode_n_bits(i, 1)?;
            let length_type = length_type == 1;

            let mut subpackets = Vec::new();
            if length_type {
                let subpacket_count = decode_n_bits(i, 11)? as usize;
                for _ in 0..subpacket_count {
                    subpackets.push(Packet::decode(i)?);
                }
                Ok(PacketType::Operator(
                    packet_type,
                    length_type,
                    subpacket_count,
                    subpackets,
                ))
            } else {
                let total_length = decode_n_bits(i, 15)? as usize;
                let mut subsequence = take_n_bits(i, total_length)?;
                while let Ok(packet) = Packet::decode(&mut subsequence) {
                    subpackets.push(packet);
                }
                Ok(PacketType::Operator(
                    packet_type,
                    length_type,
                    total_length,
                    subpackets,
                ))
            }
        }
    }

    fn sum_of_versions(&self) -> u64 {
        match self {
            PacketType::Literal(_) => 0,
            PacketType::Operator(_, _, _, sps) => sps.iter().map(|sb| sb.version_sum()).sum(),
        }
    }

    fn value(&self) -> u64 {
        match self {
            PacketType::Literal(i) => *i,
            PacketType::Operator(op, _, _, sps) => match op {
                0 => sps.iter().map(|p| p.value()).sum(),
                1 => sps.iter().map(|p| p.value()).product(),
                2 => sps.iter().map(|p| p.value()).min().unwrap(),
                3 => sps.iter().map(|p| p.value()).max().unwrap(),
                5 => (sps[0].value() > sps[1].value()).into(),
                6 => (sps[0].value() < sps[1].value()).into(),
                7 => (sps[0].value() == sps[1].value()).into(),
                _ => panic!("invalid operator {op}"),
            },
        }
    }
}

fn part_1(input: &str) -> Result<u64, Error> {
    Ok(Packet::decode_from_hexadecimal(input)?.version_sum())
}

fn part_2(input: &str) -> Result<u64, Error> {
    Ok(Packet::decode_from_hexadecimal(input)?.value())
}

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        decode_literal_value, decode_n_bits, decode_to_binary, part_1, part_2, Packet, PacketType,
    };
    use test_case::test_case;

    #[test]
    fn test_decode_to_binary() {
        let res: String = decode_to_binary("D2FE28").map(|d| format!("{d}")).collect();
        assert_eq!("110100101111111000101000", res);
    }

    #[test]
    fn test_decode_literal() {
        let mut i = decode_to_binary("D2FE28");
        assert_eq!(6, decode_n_bits(&mut i, 3).unwrap());
        assert_eq!(4, decode_n_bits(&mut i, 3).unwrap());
        assert_eq!(2021, decode_literal_value(&mut i).unwrap());
    }

    #[test]
    fn test_decode_operator() {
        let packet = Packet::decode_from_hexadecimal("EE00D40C823060").unwrap();
        assert_eq!(7, packet.0);
        let PacketType::Operator(op, lt, l, sb) = packet.1 else { panic!("not an operator packet")};
        assert_eq!(3, op);
        assert!(lt);
        assert_eq!(3, l);
        assert_eq!(3, sb.len());
    }

    #[test_case("8A004A801A8002F478", 16)]
    #[test_case("620080001611562C8802118E34", 12)]
    #[test_case("C0015000016115A2E0802F182340", 23)]
    #[test_case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_part_1(input: &str, version_sum: u64) {
        assert_eq!(version_sum, part_1(input).unwrap());
    }

    #[test_case("C200B40A82", 3)]
    #[test_case("04005AC33890", 54)]
    #[test_case("880086C3E88112", 7)]
    #[test_case("CE00C43D881120", 9)]
    #[test_case("D8005AC2A8F0", 1)]
    #[test_case("F600BC2D8F", 0)]
    #[test_case("9C005AC2F8F0", 0)]
    #[test_case("9C0141080250320F1802104A08", 1)]
    fn test_part_2(input: &str, value: u64) {
        assert_eq!(value, part_2(input).unwrap());
    }
}
