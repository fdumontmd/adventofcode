use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_utils::get_input;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RECORD_RE: Regex = Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (Guard #(\d+) begins shift|falls asleep|wakes up)$").unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Activity {
    StartShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    id: usize,
    activity: Activity,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{:02}-{:02} {:02}:{:02}] ", self.year, self.month, self.day,
               self.hour, self.minute)?;
        match self.activity {
            Activity::StartShift => write!(f, "Guard #{} begins shift", self.id),
            Activity::FallsAsleep => write!(f, "falls asleep"),
            Activity::WakesUp => write!(f, "wakes up"),
        }
    }
}

#[derive(Debug)]
struct RecordParseError {
    line: String,
    cause: Option<ParseIntError>,
}

impl RecordParseError {
    fn new(s: &str) -> Self {
        RecordParseError {
            line: s.into(),
            cause: None,
        }
    }

    fn parse_int(s: &str, e: ParseIntError) -> Self {
        RecordParseError {
            line: s.into(),
            cause: Some(e),
        }
    }
}

impl fmt::Display for RecordParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RecordParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|r| r as &Error)
    }
}

impl FromStr for Record {
    type Err = RecordParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for caps in RECORD_RE.captures_iter(s) {
            let year = caps[1].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
            let month = caps[2].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
            let day = caps[3].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
            let hour = caps[4].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
            let minute = caps[5].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
            let mut id: usize = 0;

            let activity = match &caps[6] {
                "falls asleep" => Activity::FallsAsleep,
                "wakes up" => Activity::WakesUp,
                _ => {
                    id = caps[7].parse().map_err(|e| RecordParseError::parse_int(s, e))?;
                    Activity::StartShift
                }
            };

            return Ok(Record {
                year,
                month,
                day,
                hour,
                minute,
                id,
                activity,
            });
        }
        Err(RecordParseError::new(s))
    }
}

fn input() -> Result<Vec<Record>, Box<dyn Error>> {
    use std::io::BufRead;
    let mut v: Vec<Record> = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        v.push(line.parse()?);
    }
    v.sort();

    let mut id = 0;

    for r in &mut v {
        match r.activity {
            Activity::StartShift => id = r.id,
            _ => r.id = id,
        }
    }
    Ok(v)
}

use std::collections::HashMap;
fn common(v: &Vec<Record>) -> (HashMap<usize, usize>, HashMap<usize, HashMap<usize, usize>>) {
    let mut guards: HashMap<usize, usize> = HashMap::new();
    let mut minutes: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    let mut sleep_start = None;
    let mut id = None;

    for r in v {
        match r.activity {
            Activity::StartShift => id = Some(r.id),
            Activity::FallsAsleep => sleep_start = Some(r.minute),
            Activity::WakesUp => {
                let sleep_start = sleep_start.unwrap();
                *guards.entry(id.unwrap()).or_default() += r.minute - sleep_start;
                let day = minutes.entry(id.unwrap()).or_default();
                for m in sleep_start..r.minute {
                    *day.entry(m).or_default() += 1;
                }
            }
        }
    }

    (guards, minutes)
}



fn part_one(v: &Vec<Record>) -> usize {
    let (guards, minutes) = common(v);

    if let Some(&sleep) = guards.values().max() {
        let mut id = None;
        for (i, s) in guards {
            if sleep == s {
                id = Some(i);
                break;
            }
        }
        let id = id.unwrap();

        let minutes = minutes.get(&id).unwrap();
        let times = *minutes.values().max().unwrap();

        let minute = minutes.iter().filter(|(_, &t)| t == times).map(|(m, _)| *m).next().unwrap();

        println!("most asleep guard {} slept most ({} times) at minute {}", id, times, minute);

        id * minute
    } else {
        unreachable!()
    }
}

fn part_two(v: &Vec<Record>) -> usize {
    let (_, minutes) = common(v);

    let mut current: Option<(usize, usize, usize)> = None;

    for (id, minutes) in minutes {
        let most = *minutes.values().max().unwrap();
        let minute = minutes.iter().filter(|(_, &c)| most == c).map(|(&m, _)| m).next().unwrap();

        current = Some(match current.take() {
            None => (id, minute, most),
            Some((i, m, c)) => if most > c {
                (id, minute, most)
            } else {
                (i, m, c)
            }
        });
    }

    let current = current.unwrap();

    println!("guard {} slept most ({} times) at minute {}", current.0, current.2, current.1);

    current.0 * current.1
}

fn main() -> Result<(), Box<dyn Error>> {
    let v = input()?;
    println!("part one: {}", part_one(&v));
    println!("part two: {}", part_two(&v));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part_one() {
        let v: Vec<Record> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(part_one(&v), 240);
    }

    #[test]
    fn test_part_two() {
        let v: Vec<Record> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(part_two(&v), 4455);
    }
}
