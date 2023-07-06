import { perform, FeedbackPattern, PerformanceTime } from "./index.js";

console.log(FeedbackPattern, perform, PerformanceTime);
while (true) {
  perform(FeedbackPattern.LevelChange, PerformanceTime.Now);
  await new Promise((resolve) => setTimeout(resolve, 500));
}
