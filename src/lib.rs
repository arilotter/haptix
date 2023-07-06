use std::convert::TryFrom;

use cacao::appkit::haptics::{FeedbackPattern, HapticFeedbackPerformer, PerformanceTime};
use neon::prelude::*;
fn perform(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let pattern = FeedbackPattern::try_from(cx.argument::<JsNumber>(0)?.value(&mut cx))
        .or_else(|e| cx.throw_type_error(&e.to_string()))?;
    let performance_time = PerformanceTime::try_from(cx.argument::<JsNumber>(1)?.value(&mut cx))
        .or_else(|e| cx.throw_type_error(&e.to_string()))?;
    HapticFeedbackPerformer::default().perform(pattern, performance_time);
    Ok(JsUndefined::new(&mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("perform", perform)?;
    let zero = cx.number(0);
    let one = cx.number(1);
    let two = cx.number(2);
    let feedback_pattern = {
        let o = JsObject::new(&mut cx);
        o.set(&mut cx, "Generic", zero)?;
        o.set(&mut cx, "Alignment", one)?;
        o.set(&mut cx, "LevelChange", two)?;
        o
    };
    cx.export_value("FeedbackPattern", feedback_pattern)?;

    let performance_time = {
        let o = JsObject::new(&mut cx);
        o.set(&mut cx, "Default", zero)?;
        o.set(&mut cx, "Now", one)?;
        o.set(&mut cx, "DrawCompleted", two)?;
        o
    };
    cx.export_value("PerformanceTime", performance_time)?;

    Ok(())
}
