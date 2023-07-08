#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use io_kit::actuator::MultitouchActuator;
use napi::{bindgen_prelude::*, JsNumber, Status};

use cacao::appkit::haptics::{
  FeedbackPattern as HFeedbackPattern, HapticFeedbackPerformer, PerformanceTime as HPerformanceTime,
};

/**
 * WARNING: This uses an undocumented internal MacOS API, and might break at any time
 * or cause your App to be rejected from the Mac App Store.
 *
 * You do get way deeper clicks though :D
 */
#[napi]
pub fn internal_api_perform(actuation: Actuation) -> Result<()> {
  // See https://github.com/niw/HapticKey/blob/8386f835551cb529aacac6c1937933b0545c72b3/HapticKey/Sources/HTKMultitouchActuator.m#L82
  // for the reverse-engineered low-level haptics APIs.

  let mut actuator = MultitouchActuator::new();
  actuator
    .actuate_actuation_id(actuation as u32)
    .map_err(|e| napi::Error::new(Status::GenericFailure, format!("Failed to actuate: {}", e)))?;

  Ok(())
}

/**
 * WARNING WARNING WARNING WARNING WARNING
 * WARNING WARNING WARNING WARNING WARNING
 *
 * THIS IS AN UNSAFE RUST FUNCTION
 * THIS CAN SEGFAULT
 * CALL AT YOUR OWN RISK
 *
 * WARNING WARNING WARNING WARNING WARNING
 * WARNING WARNING WARNING WARNING WARNING
 *
 * This uses an undocumented internal MacOS API, and might break at any time
 * or cause your App to be rejected from the Mac App Store.
 *
 * You do get way deeper clicks though :D
 */
#[napi]
pub unsafe fn __unsafe_internal_api_perform(
  actuation: u32,
  unknown1: u32,
  unknown2: JsNumber,
  unknown3: JsNumber,
) -> Result<()> {
  // See https://github.com/niw/HapticKey/blob/8386f835551cb529aacac6c1937933b0545c72b3/HapticKey/Sources/HTKMultitouchActuator.m#L82
  // for the reverse-engineered low-level haptics APIs.

  let mut actuator = MultitouchActuator::new();
  unsafe {
    actuator.actuate_actuation_id_raw(
      actuation,
      unknown1,
      unknown2
        .get_double()
        .expect("invalid float passed to unknown2") as f32,
      unknown3
        .get_double()
        .expect("invalid float passed to unknown3") as f32,
    )
  }
  .map_err(|e| napi::Error::new(Status::GenericFailure, format!("Failed to actuate: {}", e)))?;

  Ok(())
}

#[napi]
#[non_exhaustive]
pub enum Actuation {
  // To find predefined actuation IDs, run the following command:
  // $ otool -s __TEXT __tpad_act_plist /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/Current/MultitouchSupport|tail -n +3|awk -F'\t' '{print $2}'|xxd -r -p
  // This show an embedded plist file in `MultitouchSupport.framework`.
  // Valid IDs were, when last checked, 1, 2, 3, 4, 5, 6, 15, and 16.
  None = 0,
  Weak = 3,
  Medium = 4,
  Strong = 6,

  One = 1,
  Two = 2,
  Fifteen = 15,
  Sixteen = 16,
}

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

#[test]
fn test_internal_api() {
  internal_api_perform(Actuation::Strong).unwrap();
}

#[test]
fn test_public_api() {
  perform(FeedbackPattern::Generic, PerformanceTime::Default);
  perform(FeedbackPattern::Alignment, PerformanceTime::Now);
  perform(FeedbackPattern::LevelChange, PerformanceTime::DrawCompleted);
}
