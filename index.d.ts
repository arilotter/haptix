declare module "haptix" {
  export function perform(
    performanceTime: (typeof PerformanceTime)[keyof typeof PerformanceTime],
    feedbackPattern: (typeof FeedbackPattern)[keyof typeof FeedbackPattern]
  ): void;

  export const PerformanceTime: {
    Default: 0;
    Now: 1;
    DrawCompleted: 2;
  };

  export const FeedbackPattern: {
    Generic: 0;
    Alignment: 1;
    LevelChange: 2;
  };
}
