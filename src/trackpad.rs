use derive_new::new;
use std::collections::HashMap;
use std::path::PathBuf;

pub trait TrackpadAction {
    fn act(&mut self, defition: &TrackpadDefinition);
}

#[derive(new)]
pub struct TrackpadActor {
    pub definition: TrackpadDefinition,
    pub action: Box<dyn TrackpadAction>,
}

#[derive(Debug, new, Default, Clone)]
pub struct TrackpadDefinition {
    pub name: String,
    pub log_path: PathBuf,
    pub phy_path: String,
    pub slotable: bool,
    pub touchable: bool,
    pub pressurable: bool,
    pub state: TrackpadState,
}

#[derive(Debug, new, Default, Clone)]
pub struct TrackpadState {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
    pub touching: bool,
    pub slot: i32,
    pub touches: HashMap<i32, TrackedTouchState>,
    pub touch_limits: TouchLimits,
}

#[derive(Debug, new, Default, Copy, Clone)]
pub struct TrackedTouchState {
    pub x: i32,
    pub y: i32,
    pub p: i32,
}

#[derive(Debug, new, Default, Copy, Clone)]
pub struct TouchLimits {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_p: i32,
    pub max_p: i32,
}

impl Default for TrackpadActor {
    fn default() -> Self {
        Self::new(
            TrackpadDefinition::default(),
            Box::new(TrackpadNullAction::default()),
        )
    }
}

#[derive(Debug, new, Default)]
struct TrackpadNullAction {}
impl TrackpadAction for TrackpadNullAction {
    fn act(&mut self, _: &TrackpadDefinition) {}
}
