use crate::scene::RSGNodeKey;

slotmap::new_key_type! {
    pub struct RSGViewportKey;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSGViewportRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32
}

#[derive(Clone, Copy)]
pub struct RSGViewportComponent {
    pub rect: Option<RSGViewportRect>,
    pub camera_node_key: Option<RSGNodeKey>
}

impl RSGViewportComponent {
    pub fn new(rect: Option<RSGViewportRect>, camera_node_key: Option<RSGNodeKey>) -> Self {
        RSGViewportComponent {
            rect: rect,
            camera_node_key: camera_node_key
        }
    }
}

pub type RSGViewportComponentList = slotmap::SlotMap<RSGViewportKey, RSGViewportComponent>;
