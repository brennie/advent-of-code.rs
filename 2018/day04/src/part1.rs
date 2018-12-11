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

    let guard_id = asleep_freq
        .iter()
        .map(|(id, minutes_asleep)| (id, minutes_asleep.iter().fold(0, std::ops::Add::add)))
        .max_by(|(_, a_asleep), (_, b_asleep)| Ord::cmp(a_asleep, b_asleep))
        .map(|(id, _)| id)
        .expect("No guards were asleep?");

    let minute = asleep_freq[&guard_id]
        .into_iter()
        .enumerate()
        .max_by(|(_, a_freq), (_, b_freq)| Ord::cmp(a_freq, b_freq))
        .map(|(min, _)| min)
        .expect("This guard wasn't asleep?");

    (*guard_id, minute as u32)
}

fn process_summary(asleep_freq: &mut HashMap<u32, [u32; 60]>, summary: NightSummary) {
    let minutes_asleep = asleep_freq
        .entry(summary.guard_id)
        .or_insert_with(|| [0; 60]);

    for (i, &asleep) in summary.minutes_asleep.0.into_iter().enumerate() {
        minutes_asleep[i] += asleep as u32;
    }
}
