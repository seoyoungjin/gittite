//!

use easy_cast::{Conv, ConvFloat};
use std::cmp;
use git2::PackBuilderStage;
use super::remotes::{push::ProgressNotification, tags::PushTagsProgress};

///
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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

///
pub trait AsyncProgress: Clone + Send + Sync {
    ///
    fn is_done(&self) -> bool;
    ///
    fn progress(&self) -> ProgressPercent;
}

impl AsyncProgress for ProgressNotification {
    fn is_done(&self) -> bool {
        *self == Self::Done
    }
    fn progress(&self) -> ProgressPercent {
        match *self {
            Self::Packing {
                stage,
                current,
                total,
            } => match stage {
                PackBuilderStage::AddingObjects
                | PackBuilderStage::Deltafication => {
                    ProgressPercent::new(current, total)
                }
            },
            Self::PushTransfer { current, total, .. } => {
                ProgressPercent::new(current, total)
            }
            Self::Transfer {
                objects,
                total_objects,
                ..
            } => ProgressPercent::new(objects, total_objects),
            _ => ProgressPercent::full(),
        }
    }
}

impl AsyncProgress for PushTagsProgress {
    fn progress(&self) -> ProgressPercent {
        match self {
            Self::CheckRemote => ProgressPercent::empty(),
            Self::Push { pushed, total } => {
                ProgressPercent::new(*pushed, *total)
            }
            Self::Done => ProgressPercent::full(),
        }
    }
    fn is_done(&self) -> bool {
        *self == Self::Done
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
