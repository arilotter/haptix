import {
  Actuation,
  FeedbackPattern,
  PerformanceTime,
  internalApiPerform,
  perform,
} from "./index.js";

while (true) {
  for (let i = 0; i < 3; i++) {
    internalApiPerform(Actuation.Strong);
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  await new Promise((resolve) => setTimeout(resolve, 500));

  for (let i = 0; i < 3; i++) {
    perform(FeedbackPattern.LevelChange, PerformanceTime.Now);
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  await new Promise((resolve) => setTimeout(resolve, 500));
}
