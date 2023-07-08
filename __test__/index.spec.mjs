import test from "ava";

import {
  perform,
  FeedbackPattern,
  PerformanceTime,
  internalApiPerform,
  Actuation,
  unsafeInternalApiPerform,
} from "../index.js";

test("perform doesn't crash on macos", (t) => {
  t.notThrows(() => {
    perform(FeedbackPattern.LevelChange, PerformanceTime.Now);
  });
});

test("internal perform doesn't crash on macos", (t) => {
  t.notThrows(() => {
    try {
      internalApiPerform(Actuation.Strong);
    } catch (e) {
      if (
        e
          .toString()
          .includes(
            "Failed to actuate: Couldn't find any devices that support haptics."
          )
      ) {
        // ok!
      } else {
        throw e;
      }
    }
  });
});

test("internal unsafe perform raw doesn't crash on macos", (t) => {
  t.notThrows(() => {
    try {
      unsafeInternalApiPerform(6, 0, 0.0, 0.0);
    } catch (e) {
      if (
        e
          .toString()
          .includes(
            "Failed to actuate: Couldn't find any devices that support haptics."
          )
      ) {
        // ok!
      } else {
        throw e;
      }
    }
  });
});
