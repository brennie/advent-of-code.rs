use std::fmt;
use std::iter::once;
use std::mem::replace;

use chrono::{prelude::*, Duration, NaiveDateTime, NaiveTime};

use crate::{Record, RecordKind};

pub struct TimeCard(pub [bool; 60]);

impl Default for TimeCard {
    fn default() -> Self {
        TimeCard([false; 60])
    }
}

impl fmt::Debug for TimeCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[..]")
    }
}

#[derive(Debug)]
pub struct NightSummary {
    pub guard_id: u32,
    pub minutes_asleep: TimeCard,
}

#[derive(Debug, Eq, PartialEq)]
enum GuardState {
    Awake,
    Asleep { since: NaiveDateTime },
}

#[derive(Debug)]
struct State {
    guard_id: u32,
    guard_state: GuardState,
    minutes_asleep: TimeCard,
}

#[derive(Debug)]
struct StateMachine(Option<State>);

pub fn run_state_machine<I>(records: I) -> impl Iterator<Item = NightSummary>
where
    I: IntoIterator<Item = Record>,
{
    records
        .into_iter()
        .map(|record| Some(record))
        .chain(once(None))
        .scan(StateMachine(None), |state_machine, record| {
            if let Some(record) = record {
                Some(state_machine.next(record))
            } else if let Some(mut state) = state_machine.0.take() {
                Some(Some(state.finalize_summary(None)))
            } else {
                None
            }
        })
        .filter_map(|x| x)
}

impl StateMachine {
    fn next(&mut self, record: Record) -> Option<NightSummary> {
        let (next_state, to_yield) = match (self.0.take(), record.kind) {
            (None, RecordKind::BeginShift(guard_id)) => {
                let next_state = State {
                    guard_id: guard_id,
                    guard_state: GuardState::Awake,
                    minutes_asleep: TimeCard::default(),
                };

                (next_state, None)
            }

            (None, _) => {
                unimplemented!("Invalid state machine transition; expected RecordKind::BeginShift")
            }

            (Some(mut state), RecordKind::BeginShift(guard_id)) => {
                let to_yield = state.finalize_summary(Some(&record));
                state.guard_id = guard_id;

                (state, Some(to_yield))
            }

            (Some(mut state), RecordKind::FallAsleep) => {
                assert!(state.guard_state == GuardState::Awake);

                state.guard_state = GuardState::Asleep {
                    since: record.timestamp,
                };

                (state, None)
            }

            (Some(mut state), RecordKind::WakeUp) => {
                if let GuardState::Asleep { .. } = state.guard_state {
                    let to_yield = state.finalize_summary(Some(&record));
                    state.guard_state = GuardState::Awake;

                    (state, Some(to_yield))
                } else {
                    unimplemented!(
                        "Invalid state machine transition {:?} -> {:?}",
                        state.guard_state,
                        GuardState::Awake,
                    );
                }
            }
        };

        self.0 = Some(next_state);
        to_yield
    }
}

impl State {
    fn finalize_summary(&mut self, record: Option<&Record>) -> NightSummary {
        if let GuardState::Asleep { since } = self.guard_state {
            self.record_sleep(since, record.map(|r| r.timestamp));
        }

        NightSummary {
            guard_id: self.guard_id,
            minutes_asleep: replace(&mut self.minutes_asleep, TimeCard([false; 60])),
        }
    }

    fn record_sleep(&mut self, since: NaiveDateTime, timestamp: Option<NaiveDateTime>) {
        let start = if time_in_range(
            since.time(),
            NaiveTime::from_hms(1, 0, 0),
            NaiveTime::from_hms(23, 59, 59),
        ) {
            NaiveDateTime::new(
                since.date() + Duration::days(1),
                NaiveTime::from_hms(0, 0, 0),
            )
        } else {
            since
        };

        let end = if let Some(timestamp) = timestamp {
            if timestamp.date() > start.date() {
                NaiveDateTime::new(start.date(), NaiveTime::from_hms(1, 0, 0))
            } else if timestamp.date() == start.date() {
                if time_in_range(
                    timestamp.time(),
                    NaiveTime::from_hms(0, 0, 0),
                    NaiveTime::from_hms(0, 59, 59),
                ) {
                    timestamp
                } else {
                    start.date().and_hms(1, 0, 0)
                }
            } else {
                // The guard fell asleep and woke up before midnight.
                assert!(since < start);
                assert!(since < timestamp);

                return;
            }
        } else {
            start.date().and_hms(1, 0, 0)
        };

        assert!(start <= end);

        let delta = end - start;

        assert!(delta > Duration::seconds(0));
        assert!(delta <= Duration::hours(1));
        assert_eq!(delta.num_seconds() % 60, 0);

        for offset in 0..delta.num_minutes() {
            let minute = start.minute() as usize + offset as usize;
            self.minutes_asleep.0[minute] = true;
        }
    }
}

fn time_in_range(time: NaiveTime, lower_bound: NaiveTime, upper_bound: NaiveTime) -> bool {
    lower_bound <= time && time <= upper_bound
}
