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
    internalApiPerform(Actuation.Strong);
  });
});

test("internal unsafe perform raw doesn't crash on macos", (t) => {
  t.notThrows(() => {
    unsafeInternalApiPerform(6, 0, 0.0, 0.0);
  });
});
