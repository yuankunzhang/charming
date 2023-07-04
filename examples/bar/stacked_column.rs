use echarts::{component::tooltip, element::tooltip_trigger, Chart};

fn main() {
    let chart = Chart::new()
        .tooltip(tooltip::Tooltip::new().trigger(tooltip_trigger::TooltipTrigger::Axis));
}
