use anyhow::{bail, Error};
use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn parse_boundaries(input: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let bounds = input.split_whitespace().last().unwrap();
    let dims: Vec<_> = bounds.split(',').collect();
    assert_eq!(3, dims.len());
    (
        parse_range(dims[0]),
        parse_range(dims[1]),
        parse_range(dims[2]),
    )
}

fn parse_range(dims: &str) -> (i64, i64) {
    let range = dims.split('=').last().unwrap();
    let ends: Vec<_> = range.split("..").collect();
    assert_eq!(2, ends.len());
    (ends[0].parse().unwrap(), ends[1].parse().unwrap())
}

fn part_1(input: &str) -> usize {
    let mut on_cubes = HashSet::new();

    for line in input.lines() {
        let on = line.starts_with("on ");

        let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) = parse_boundaries(line);

        if x_min.abs() > 50
            || x_max.abs() > 50
            || y_min.abs() > 50
            || y_max.abs() > 50
            || z_min.abs() > 50
            || z_max.abs() > 50
        {
            continue;
        }

        for z in z_min..=z_max {
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if on {
                        on_cubes.insert((x, y, z));
                    } else {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    on_cubes.len()
}

#[derive(Debug, Eq, PartialEq)]
enum Switch {
    On,
    Off,
}

#[derive(Debug)]
struct Step {
    switch: Switch,
    bounding_box: BoundingBox,
}

impl TryFrom<&str> for Step {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split_whitespace().collect();
        let switch = match parts[0] {
            "on" => Switch::On,
            "off" => Switch::Off,
            _ => bail!("Invalid switch mode {}", parts[0]),
        };
        let bounding_box = BoundingBox::try_from(parts[1])?;

        Ok(Step {
            switch,
            bounding_box,
        })
    }
}

// Part 2:
// idea: keep track of boxes with On cubes; each line
// starting with "on" will add its box to that list
// but before adding a new box (or if the box is an Off box),
// check for existing boxes in the list that intersect with the
// new box, and but those into pieces to remove the intersection
//
// The "Off" boxes will thus just remove the parts of "On" boxes they
// overlap with; the "On" boxes will do the same to avoid double-counting
// the intersections
//
// In the end, just sum the volume of the "On" boxes
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point(i64, i64, i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct BoundingBox {
    min_point: Point,
    max_point: Point,
}

impl BoundingBox {
    fn volume(&self) -> i64 {
        (self.max_point.0 - self.min_point.0)
            * (self.max_point.1 - self.min_point.1)
            * (self.max_point.2 - self.min_point.2)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let x_min = self.min_point.0.max(other.min_point.0);
        let y_min = self.min_point.1.max(other.min_point.1);
        let z_min = self.min_point.2.max(other.min_point.2);

        let x_max = self.max_point.0.min(other.max_point.0);
        let y_max = self.max_point.1.min(other.max_point.1);
        let z_max = self.max_point.2.min(other.max_point.2);

        if x_min < x_max && y_min < y_max && z_min < z_max {
            Some(BoundingBox {
                min_point: Point(x_min, y_min, z_min),
                max_point: Point(x_max, y_max, z_max),
            })
        } else {
            None
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        !(self.max_point.0 <= other.min_point.0
            || other.max_point.0 <= self.min_point.0
            || self.max_point.1 <= other.min_point.1
            || other.max_point.1 <= self.min_point.1
            || self.max_point.2 <= other.min_point.2
            || other.max_point.2 <= self.min_point.2)
    }

    // assumes other is contained in self
    fn cut_out(&self, other: &Self) -> Vec<Self> {
        let mut parts = vec![];

        let mut block = *self;

        if block.min_point.0 != other.min_point.0 {
            parts.push(BoundingBox {
                max_point: Point(other.min_point.0, block.max_point.1, block.max_point.2),
                ..block
            });

            block = BoundingBox {
                min_point: Point(other.min_point.0, block.min_point.1, block.min_point.2),
                ..block
            };
        }

        if block.max_point.0 != other.max_point.0 {
            parts.push(BoundingBox {
                min_point: Point(other.max_point.0, block.min_point.1, block.min_point.2),
                ..block
            });

            block = BoundingBox {
                max_point: Point(other.max_point.0, block.max_point.1, block.max_point.2),
                ..block
            };
        }

        if block.min_point.1 != other.min_point.1 {
            parts.push(BoundingBox {
                max_point: Point(block.max_point.0, other.min_point.1, block.max_point.2),
                ..block
            });

            block = BoundingBox {
                min_point: Point(block.min_point.0, other.min_point.1, block.min_point.2),
                ..block
            }
        }

        if block.max_point.1 != other.max_point.1 {
            parts.push(BoundingBox {
                min_point: Point(block.min_point.0, other.max_point.1, block.min_point.2),
                ..block
            });

            block = BoundingBox {
                max_point: Point(block.max_point.0, other.max_point.1, block.max_point.2),
                ..block
            };
        }

        if block.min_point.2 != other.min_point.2 {
            parts.push(BoundingBox {
                max_point: Point(block.max_point.0, block.max_point.1, other.min_point.2),
                ..block
            });

            block = BoundingBox {
                min_point: Point(block.min_point.0, other.min_point.1, other.min_point.2),
                ..block
            }
        }

        if block.max_point.2 != other.max_point.2 {
            parts.push(BoundingBox {
                min_point: Point(block.min_point.0, block.min_point.1, other.max_point.2),
                ..block
            });

            block = BoundingBox {
                max_point: Point(block.max_point.0, block.max_point.1, other.max_point.2),
                ..block
            };
        }

        assert_eq!(&block, other);

        assert_eq!(
            parts.iter().map(|b| b.volume()).sum::<i64>() + block.volume(),
            self.volume()
        );

        parts
    }
}

impl TryFrom<&str> for BoundingBox {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn parse_range(dims: &str) -> Result<(i64, i64), Error> {
            let range = dims.split('=').last().unwrap();
            let ends: Vec<_> = range.split("..").collect();
            assert_eq!(2, ends.len());
            Ok((ends[0].parse()?, ends[1].parse()?))
        }
        let dims: Vec<_> = value.split(',').collect();
        if dims.len() != 3 {
            bail!("cannot parse {value} as Box");
        }
        let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) = (
            parse_range(dims[0])?,
            parse_range(dims[1])?,
            parse_range(dims[2])?,
        );
        Ok(BoundingBox {
            min_point: Point(x_min, y_min, z_min),
            max_point: Point(x_max + 1, y_max + 1, z_max + 1),
        })
    }
}

fn part_2(input: &str) -> i64 {
    let mut blocks = HashSet::new();

    for line in input.lines() {
        let step = Step::try_from(line).unwrap();
        let tmp_blocks: Vec<_> = blocks
            .iter()
            .filter(|b| step.bounding_box.intersects(b))
            .cloned()
            .collect();
        blocks.retain(|b| !step.bounding_box.intersects(b));
        for block in tmp_blocks {
            let i = step.bounding_box.intersection(&block).unwrap();
            let remains = block.cut_out(&i);
            blocks.extend(remains);
        }
        if step.switch == Switch::On {
            blocks.insert(step.bounding_box);
        }
    }

    blocks.into_iter().map(|b| b.volume()).sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{part_1, part_2, BoundingBox};
    static TEST_INPUT_SMALL: &str = r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    static TEST_INPUT_LARGE: &str = r"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    static TEST_INPUT_VERY_LARGE: &str = r"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test_case(TEST_INPUT_SMALL, 39)]
    #[test_case(TEST_INPUT_LARGE, 590784)]
    #[test_case(crate::INPUT, 527915)]
    fn test_part_1(input: &str, count: usize) {
        assert_eq!(count, part_1(input));
    }

    #[test_case(TEST_INPUT_SMALL)]
    #[test_case(TEST_INPUT_LARGE)]
    #[test_case(TEST_INPUT_VERY_LARGE)]
    #[test_case(crate::INPUT)]
    fn test_box_intersect(input: &str) {
        let boxes = Vec::from_iter(
            input
                .lines()
                .map(|l| l.split_whitespace().last().unwrap())
                .map(|l| BoundingBox::try_from(l).unwrap()),
        );
        for b1 in &boxes {
            for b2 in &boxes {
                assert_eq!(b1.intersects(b2), b1.intersection(b2).is_some());
            }
        }
    }

    #[test_case(TEST_INPUT_VERY_LARGE, 2758514936282235)]
    #[test_case(crate::INPUT, 1218645427221987)]
    fn test_part_2(input: &str, count: i64) {
        assert_eq!(count, part_2(input));
    }
}
