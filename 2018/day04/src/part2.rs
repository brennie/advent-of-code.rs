use std::collections::HashMap;
use std::process::exit;

use day04::state_machine::{run_state_machine, NightSummary};
use day04::{read_records, Record};

fn main() {
    match read_records().map(find_guard_and_time) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok((guard_id, minute)) => println!("{}", guard_id * minute),
    }
}

fn find_guard_and_time(mut records: Vec<Record>) -> (u32, u32) {
    records.sort_unstable();

    let mut asleep_freq = HashMap::<u32, [u32; 60]>::new();

    for summary in run_state_machine(records) {
        process_summary(&mut asleep_freq, summary);
    }

    asleep_freq
        .into_iter()
        .map(|(id, minutes_asleep)| {
            (
                id,
                minutes_asleep
                    .into_iter()
                    .enumerate()
                    .max_by(|(_, a_freq), (_, b_freq)| Ord::cmp(a_freq, b_freq))
                    .map(|(minute, freq)| (minute, *freq)),
            )
        })
        .filter_map(|(id, maybe_info)| match maybe_info {
            Some((minute, freq)) => Some((id, minute, freq)),
            None => None,
        })
        .max_by(|(_, _, a_freq), (_, _, b_freq)| Ord::cmp(a_freq, b_freq))
        .map(|(guard_id, minute, _)| (guard_id as u32, minute as u32))
        .expect("No guard was asleep?")
}

fn process_summary(asleep_freq: &mut HashMap<u32, [u32; 60]>, summary: NightSummary) {
    let minutes_asleep = asleep_freq
        .entry(summary.guard_id)
        .or_insert_with(|| [0; 60]);

    for (i, &asleep) in summary.minutes_asleep.0.into_iter().enumerate() {
        minutes_asleep[i] += asleep as u32;
    }
}
