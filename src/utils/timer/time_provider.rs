pub trait TimeProvider {
    fn now(&mut self) -> std::time::Instant;
}

#[derive(Clone, Copy, Debug)]
pub struct RealTimeProvider;

impl TimeProvider for RealTimeProvider {
    fn now(&mut self) -> std::time::Instant {
        std::time::Instant::now()
    }
}

pub trait TimeProviderExt: TimeProvider + Clone + Copy { }

impl TimeProviderExt for RealTimeProvider {}
