#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate itertools;

use itertools::Itertools;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use regex::Regex;
use std::error;

use aoc::{read_input, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for CustomError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn get_value<'a>(
    caps: &regex::Captures<'a>,
    index: usize,
) -> std::result::Result<i32, CustomError> {
    caps.get(index)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<CustomError, _>(|| {
            CustomError(format!("Invalid {}", index))
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum EventType {
    ShiftStart(i32),
    FallAsleep,
    WakeUp,
}

impl FromStr for EventType {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            //         1
            // Guard #id
            static ref RE: Regex = Regex::new(r"Guard #(\d+)").unwrap();
        }
        if s.contains("falls asleep") {
            return Ok(EventType::FallAsleep);
        }

        if s.contains("wakes up") {
            return Ok(EventType::WakeUp);
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

        let id = get_value(&caps, 1)?;

        Ok(EventType::ShiftStart(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Timestamp {
    year: i32,
    month: i32,
    day: i32,

    hour: i32,
    minute: i32,
}

impl FromStr for Timestamp {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            // 1      2    3   4    5
            // year-month-day hour:min
            static ref RE: Regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})").unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

        let year = get_value(&caps, 1)?;
        let month = get_value(&caps, 2)?;
        let day = get_value(&caps, 3)?;
        let hour = get_value(&caps, 4)?;
        let minute = get_value(&caps, 5)?;

        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct LogEvent {
    timestamp: Timestamp,
    event: EventType,
}

impl FromStr for LogEvent {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let timestamp = s.parse::<Timestamp>()?;
        let event = s.parse::<EventType>()?;

        Ok(LogEvent { timestamp, event })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Sleeping {
    start: i32,
    end: i32,
    duration: i32,
}

fn main() -> Result<()> {
    let s = read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<i32> {
    let events: std::result::Result<Vec<_>, _> =
        s.lines().map(|v| v.parse::<LogEvent>()).collect();

    let mut events = events?;

    events.sort();

    let (id, min) = gather(&events)?;

    let res = id * min;

    eprintln!("part1: {}", res);

    Ok(res)
}

fn part2(s: &str) -> Result<i32> {
    let events: std::result::Result<Vec<_>, _> =
        s.lines().map(|v| v.parse::<LogEvent>()).collect();

    let mut events = events?;

    events.sort();

    let (id, min) = gather_2(&events)?;

    let res = id * min;

    eprintln!("part2: {}", res);

    Ok(res)
}

fn gather(events: &[LogEvent]) -> Result<(i32, i32)> {
    let mut sleeping_start = 0;
    let mut sleeping_end;
    let mut guard = 0;
    let mut next_guard = guard;

    let mut map: HashMap<i32, Vec<Sleeping>> = HashMap::new();

    for event in events {
        match event.event {
            EventType::ShiftStart(id) => {
                if event.timestamp.hour > 0 {
                    next_guard = id;
                } else {
                    next_guard = id;
                }
            }
            EventType::WakeUp => {
                sleeping_end = event.timestamp.minute;

                map.entry(guard)
                    .or_insert_with(|| Vec::new())
                    .push(Sleeping {
                        start: sleeping_start,
                        end: sleeping_end,
                        duration: sleeping_end - sleeping_start,
                    });
            }
            EventType::FallAsleep => {
                sleeping_start = event.timestamp.minute;
            }
        }
        guard = next_guard;
    }

    let entry = map
        .iter()
        .max_by_key(|(_, v)| {
            let total_sleep: i32 = v.iter().map(|v| v.duration).sum();
            total_sleep
        })
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Unable to find entry".to_string()).into()
        });

    let (id, values) = entry?;
    let mut sleeping = vec![0; 60];

    for value in values {
        for index in value.start..value.end {
            sleeping[index as usize] += 1;
        }
    }

    let minute = sleeping
        .iter()
        .enumerate()
        .max_by(|(_, val), (_, v2)| val.cmp(&v2))
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing minute".to_string()).into()
        });

    let (index, _) = minute?;

    Ok((*id, index as i32))
}

fn gather_2(events: &[LogEvent]) -> Result<(i32, i32)> {
    let mut sleeping_start = 0;
    let mut sleeping_end;
    let mut guard = 0;

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for event in events {
        match event.event {
            EventType::ShiftStart(id) => {
                guard = id;
            }
            EventType::WakeUp => {
                assert!(guard > 0);
                sleeping_end = event.timestamp.minute;

                let minutes = map.entry(guard).or_insert_with(|| vec![0; 60]);

                for i in sleeping_start..sleeping_end {
                    minutes[i as usize] += 1;
                }
            }
            EventType::FallAsleep => {
                sleeping_start = event.timestamp.minute;
            }
        }
    }

    let entry = map
        .iter()
        .max_by_key(|(_, v)| v.iter().max())
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Unable to find entry".to_string()).into()
        });

    let entry = entry?;

    let minute = entry
        .1
        .iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing minute".to_string()).into()
        });

    let minute = minute?;

    Ok((*entry.0, minute.0 as i32))
}

/// Visualizes the given events
/// events should be sorted before calling this
/// ```
/// Date   ID   Minute
///             000000000011111111112222222222333333333344444444445555555555
///             012345678901234567890123456789012345678901234567890123456789
/// 11-01  #10  .....####################.....#########################.....
/// 11-02  #99  ........................................##########..........
/// 11-03  #10  ........................#####...............................
/// 11-04  #99  ....................................##########..............
/// 11-05  #99  .............................................##########.....
/// ```
#[allow(dead_code)]
fn visualize(events: &[LogEvent]) {
    let header = format!("{:7}{:6}{}", "Date", "ID", "Minute");

    let id = " ".repeat(13);

    let minutes_top = format!(
        "{}{}",
        id, "000000000011111111112222222222333333333344444444445555555555"
    );
    let minutes_bot = format!(
        "{}{}",
        id, "012345678901234567890123456789012345678901234567890123456789"
    );

    eprintln!("{}", header);
    eprintln!("{}", minutes_top);
    eprintln!("{}", minutes_bot);

    let groups = events
        .iter()
        .group_by(|e| (e.timestamp.month, e.timestamp.day));

    let mut sleeping_start = 0;
    let mut sleeping_end;
    let mut sleeping = vec!['.'; 60];
    let mut guard = 0;
    let mut next_guard = guard;

    for (key, group) in &groups {
        let date = format!("{:0>2}-{:0>2}", key.0, key.1);

        for event in group {
            match event.event {
                EventType::ShiftStart(id) => {
                    if event.timestamp.hour > 0 {
                        next_guard = id;
                    } else {
                        guard = id;
                        next_guard = id;
                    }
                }
                EventType::WakeUp => {
                    sleeping_end = event.timestamp.minute;

                    for index in sleeping_start..sleeping_end {
                        sleeping[index as usize] = '#';
                    }
                }
                EventType::FallAsleep => {
                    sleeping_start = event.timestamp.minute;
                }
            }
        }

        let s = sleeping.iter().collect::<String>();

        let id = format!("#{}", guard);

        eprintln!("{:7}{:6}{}", date, id, s);

        sleeping = vec!['.'; 60];
        guard = next_guard;
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = r"
[1518-11-01 00:00] Guard #10 begins shift
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
[1518-11-05 00:55] wakes up
        ";

        assert_eq!(240, part1(input.trim()).unwrap());
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = r"
[1518-11-01 00:00] Guard #10 begins shift
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
[1518-11-05 00:55] wakes up
        ";

        assert_eq!(99 * 45, part2(input.trim()).unwrap());
    }
}
