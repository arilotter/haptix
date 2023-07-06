const { perform, FeedbackPattern, PerformanceTime } = require(".");
async function main() {
  console.log(FeedbackPattern, perform, PerformanceTime);
  while (true) {
    perform(FeedbackPattern.Generic, PerformanceTime.Now);
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
}
main();
