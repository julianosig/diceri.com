//! The per-minute call meter, run beside the HTTP server and stopped with it.

use std::time::Duration;

use diceri_application::use_cases::MeteringService;
use tokio::sync::watch;
use tokio::time::MissedTickBehavior;

/// Call minutes are charged once a minute.
const METER_PERIOD: Duration = Duration::from_secs(60);

/// Meters call minutes every minute until `stop` turns true.
pub async fn run_meter(metering: MeteringService, mut stop: watch::Receiver<bool>) {
    let mut every_minute = tokio::time::interval(METER_PERIOD);
    every_minute.set_missed_tick_behavior(MissedTickBehavior::Delay);
    // The first tick fires at once; a whole minute must pass before charging one.
    every_minute.tick().await;
    loop {
        tokio::select! {
            _ = every_minute.tick() => metering.tick().await,
            changed = stop.changed() => {
                if changed.is_err() || *stop.borrow() {
                    break;
                }
            }
        }
    }
}
