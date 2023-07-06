#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

use cacao::appkit::haptics::{
  FeedbackPattern as HFeedbackPattern, HapticFeedbackPerformer, PerformanceTime as HPerformanceTime,
};

#[napi]
pub fn perform(pattern: FeedbackPattern, performance_time: PerformanceTime) {
  HapticFeedbackPerformer::default().perform(pattern.into(), performance_time.into());
}

#[napi]
pub enum PerformanceTime {
  Default = 0,
  Now = 1,
  DrawCompleted = 2,
}

#[napi]
pub enum FeedbackPattern {
  Generic = 0,
  Alignment = 1,
  LevelChange = 2,
}

impl From<PerformanceTime> for HPerformanceTime {
  fn from(value: PerformanceTime) -> Self {
    match value {
      PerformanceTime::Default => Self::Default,
      PerformanceTime::Now => Self::Now,
      PerformanceTime::DrawCompleted => Self::DrawCompleted,
    }
  }
}

impl From<FeedbackPattern> for HFeedbackPattern {
  fn from(value: FeedbackPattern) -> Self {
    match value {
      FeedbackPattern::Generic => Self::Generic,
      FeedbackPattern::Alignment => Self::Alignment,
      FeedbackPattern::LevelChange => Self::LevelChange,
    }
  }
}
