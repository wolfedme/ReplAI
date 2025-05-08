use tracing::Level;

pub fn init() {
    #[cfg(debug_assertions)]
    let level = Level::DEBUG;

    #[cfg(not(debug_assertions))]
    let level = Level::INFO;

    tracing_subscriber::fmt().with_max_level(level).init();
}
