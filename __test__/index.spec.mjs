import test from "ava";

import { perform, FeedbackPattern, PerformanceTime } from "../index.js";

test("perform doesn't crash on macos", (t) => {
  t.notThrows(() => {
    perform(FeedbackPattern.LevelChange, PerformanceTime.Now);
  });
});
