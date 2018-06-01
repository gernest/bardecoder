mod blockedmean;

pub use self::blockedmean::BlockedMean;

pub trait Prepare<IMG, PREPD> {
    fn prepare(&self, source: IMG) -> PREPD;
}