use serde::Serialize;

/// Types of triggering.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TooltipTrigger {
    Item,
    Axis,
    None,
}