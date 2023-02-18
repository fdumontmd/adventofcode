//  for any two pair of scanners
//  for any rotation, rotate the positions of beacons in scanner 2
//  for any two beacons in scanner 1 and rotated scanner 2,
//  compute s1 -> s2 (using s1 -> b1 - s2 -> b2)
//  then for all b in scanner 2, compute s1 -> s2 + b and compare against
//  set of beacons in scanner 1
//  if >= 12 in common, already identified those and everything else
//  we have n^2 operations no matter what, this seems to go to the
//  right answer faster

static INPUT: &str = include_str!("input.txt");

mod math;
use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use math::Vec3D;

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Vec3D>,
}

impl Scanner {
    fn parse(input: &str) -> Self {
        let mut beacons = Vec::new();
        let mut id = 0;
        for line in input.lines() {
            if let Some(i) = line.strip_prefix("--- scanner ") {
                let Some(i) = i.strip_suffix(" ---") else { panic!("cannot parse id in {line}")};
                id = i.parse().unwrap();
            } else {
                beacons.push(Vec3D::parse(line));
            }
        }
        Self { id, beacons }
    }
}

struct Problem {
    scanners: Vec<Scanner>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let mut scanners: Vec<Scanner> = input.split("\n\n").map(Scanner::parse).collect();
        scanners.sort_by_key(|s| s.id);

        Self { scanners }
    }

    fn len(&self) -> usize {
        self.scanners.len()
    }
}

impl Index<usize> for Problem {
    type Output = Scanner;

    fn index(&self, index: usize) -> &Self::Output {
        &self.scanners[index]
    }
}

fn solve_problem(input: &str) -> (usize, i64) {
    let problem = Problem::parse(input);
    let mut merged = HashSet::new();
    let mut merged_beacons: HashMap<usize, (Vec3D, HashSet<Vec3D>)> = HashMap::new();

    let mut beacons: HashSet<Vec3D> = HashSet::new();

    let mut scanners = Vec::new();
    scanners.push(Vec3D::origin());

    for beacon in &problem[0].beacons {
        beacons.insert(*beacon);
    }

    let mut fingerprints = HashMap::new();

    for s in &problem.scanners {
        for b1 in &s.beacons {
            for b2 in &s.beacons {
                if b1 != b2 {
                    fingerprints
                        .entry(s.id)
                        .or_insert(HashSet::new())
                        .insert((b1 - b2).len());
                }
            }
        }
    }

    merged.insert(problem[0].id);
    merged_beacons.insert(
        problem[0].id,
        (
            Vec3D::origin(),
            HashSet::from_iter(problem[0].beacons.iter().cloned()),
        ),
    );

    // optimised after checking reddit
    // had almost the right idea: distance between pairs of beacon
    // instead of just differences
    while merged.len() < problem.len() {
        'search_loop: for s in problem.scanners.iter().filter(|s| !merged.contains(&s.id)) {
            for (id, mb) in &merged_beacons {
                // 66 = (12 * 11)/2 is the number of distances between any two among 12 different beacons
                if fingerprints[id].intersection(&fingerprints[&s.id]).count() >= 66 {
                    for r in math::ROTATIONS {
                        let rotated_beacons: Vec<Vec3D> =
                            Vec::from_iter(s.beacons.iter().map(|b| r * b));
                        for b1 in &mb.1 {
                            for b2 in &rotated_beacons {
                                let s2 = b1 - b2;
                                let s2_beacons: HashSet<Vec3D> =
                                    HashSet::from_iter(rotated_beacons.iter().map(|b2| s2 + *b2));

                                if mb.1.intersection(&s2_beacons).count() >= 12 {
                                    merged.insert(s.id);
                                    for b in &s2_beacons {
                                        beacons.insert(mb.0 + *b);
                                    }
                                    scanners.push(mb.0 + s2);

                                    merged_beacons.insert(s.id, (mb.0, s2_beacons));

                                    break 'search_loop;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut distances = Vec::new();

    for s1 in &scanners {
        for s2 in &scanners {
            distances.push((s1 - s2).len());
        }
    }

    (beacons.len(), distances.into_iter().max().unwrap())
}

fn main() {
    let solution = solve_problem(INPUT);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

#[cfg(test)]
mod tests {
    use crate::solve_problem;
    use test_case::test_case;

    static TEST_INPUT: &str = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test_case(TEST_INPUT, 79, 3621)]
    #[test_case(crate::INPUT, 381, 12201)]
    fn test_problem(input: &str, beacon_count: usize, max_dist: i64) {
        let solution = solve_problem(input);
        assert_eq!(beacon_count, solution.0);
        assert_eq!(max_dist, solution.1);
    }
}
