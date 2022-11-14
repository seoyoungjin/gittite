//!

use super::remotes::{push::ProgressNotification, tags::PushTagsProgress};
use easy_cast::{Conv, ConvFloat};
use std::cmp;
use git2::PackBuilderStage;
use serde::Serialize;

///
#[derive(Serialize, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ProgressPercent {
    /// percent 0..100
    pub progress: u8,
}

impl ProgressPercent {
    ///
    pub fn new(current: usize, total: usize) -> Self {
        let total = f64::conv(cmp::max(current, total));
        let progress = f64::conv(current) / total * 100.0;
        let progress = u8::try_conv_nearest(progress).unwrap_or(100);
        Self { progress }
    }
    ///
    pub const fn empty() -> Self {
        Self { progress: 0 }
    }
    ///
    pub const fn full() -> Self {
        Self { progress: 100 }
    }
}

/// used for push/pull
#[derive(Serialize, Clone, Debug)]
pub enum RemoteProgressState {
    ///
    PackingAddingObject,
    ///
    PackingDeltafiction,
    ///
    Pushing,
    /// fetch progress
    Transfer,
    /// remote progress done
    Done,
}

///
#[derive(Serialize, Clone, Debug)]
pub struct RemoteProgress {
    ///
    pub state: RemoteProgressState,
    ///
    pub progress: ProgressPercent,
}

impl RemoteProgress {
    ///
    pub fn new(
        state: RemoteProgressState,
        current: usize,
        total: usize,
    ) -> Self {
        Self {
            state,
            progress: ProgressPercent::new(current, total),
        }
    }
}

impl From<ProgressNotification> for RemoteProgress {
    fn from(progress: ProgressNotification) -> Self {
        match progress {
            ProgressNotification::Packing {
                stage,
                current,
                total,
            } => match stage {
                PackBuilderStage::AddingObjects => Self::new(
                    RemoteProgressState::PackingAddingObject,
                    current,
                    total,
                ),
                PackBuilderStage::Deltafication => Self::new(
                    RemoteProgressState::PackingDeltafiction,
                    current,
                    total,
                ),
            },
            ProgressNotification::PushTransfer {
                current,
                total,
                ..
            } => Self::new(
                RemoteProgressState::Pushing,
                current,
                total,
            ),
            ProgressNotification::Transfer {
                objects,
                total_objects,
                ..
            } => Self::new(
                RemoteProgressState::Transfer,
                objects,
                total_objects,
            ),
            _ => Self::new(RemoteProgressState::Done, 1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_zero_total() {
        let prog = ProgressPercent::new(1, 0);

        assert_eq!(prog.progress, 100);
    }

    #[test]
    fn test_progress_zero_all() {
        let prog = ProgressPercent::new(0, 0);
        assert_eq!(prog.progress, 100);
    }

    #[test]
    fn test_progress_rounding() {
        let prog = ProgressPercent::new(2, 10);

        assert_eq!(prog.progress, 20);
    }
}
