slotmap::new_key_type! {
    pub struct RSGOpacityKey;
}

#[derive(Clone, Copy)]
pub struct RSGOpacityComponent {
    pub opacity: f32,
    pub inherited_opacity: f32
}

impl RSGOpacityComponent {
    pub fn new(opacity: f32) -> Self {
        RSGOpacityComponent {
            opacity: opacity,
            inherited_opacity: opacity
        }
    }
}

pub type RSGOpacityComponentList = slotmap::SlotMap<RSGOpacityKey, RSGOpacityComponent>;
