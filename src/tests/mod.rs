mod round_trip;
mod format;

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
