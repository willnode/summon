use std::{process::Command, thread, time};

use chashmap::CHashMap;

use crate::{
    mem::GLOBAL_PROCESSES,
    utils::{now_timestamp, HARDKILL_TIMEOUT, PATH_KILL}, SafeModuleCtx,
};

pub fn idling_killer() {
    let mut kill_count = 0;
    eprintln!("Summon: killer started");

    thread::sleep(time::Duration::from_secs(1));

    let mut interval = determine_lowest_interval(GLOBAL_PROCESSES.clone());

    loop {
        // Get the current time
        let now = now_timestamp();

        for (key, value) in GLOBAL_PROCESSES.clone().into_iter() {
                if value.timeout > 0 && value.lastreq + value.timeout < now {

                if value.lastreq + value.timeout + HARDKILL_TIMEOUT < now {
                    eprintln!("Summon: killer kill force {}", value.pid);
                    Command::new(PATH_KILL)
                    .arg("-9")
                    .arg(value.pid.to_string())
                    .spawn()
                    .expect("Unable to kill");

                } else {
                    eprintln!("Summon: killer kill {}", value.pid);
                    Command::new(PATH_KILL)
                    .arg(value.pid.to_string())
                    .spawn()
                    .expect("Unable to kill");
                }
                kill_count += 1;
                GLOBAL_PROCESSES.remove(&key);
            }
        }

        if kill_count > 100 {
            interval = determine_lowest_interval(GLOBAL_PROCESSES.clone());
            GLOBAL_PROCESSES.shrink_to_fit();
            kill_count = 0;
        }

        // Sleep for a minute (60 seconds)
        thread::sleep(time::Duration::from_secs(interval));

        // You can also calculate the exact time to sleep to compensate for operation time,
        // ensuring that the operation truly starts every minute, but this simple example
        // uses a fixed sleep duration.
    }
}

fn determine_lowest_interval(cloned_tb: CHashMap<String, SafeModuleCtx>) -> u64 {
    let mut lowest = 15 * 60;
    for (_, value) in cloned_tb.into_iter() {
        if value.timeout > 0 && value.timeout < lowest {
            lowest = value.timeout;
        }
    }
    return lowest;
}
