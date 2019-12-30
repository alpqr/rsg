use crate::scene::*;

pub type RSGDirtySubtreeRootList = smallvec::SmallVec<[RSGNodeKey; 16]>;

#[derive(Debug, Default)]
pub struct RSGSceneObserver {
    pub changed: bool,
    pub hierarchy_changed: bool,
    pub dirty_world_roots: RSGDirtySubtreeRootList,
    pub dirty_opacity_roots: RSGDirtySubtreeRootList,
    pub dirty_material_roots: RSGDirtySubtreeRootList,
    pub dirty_material_value_roots: RSGDirtySubtreeRootList,
    pub dirty_mesh_roots: RSGDirtySubtreeRootList
}

impl RSGObserver for RSGSceneObserver {
    fn notify(&mut self, event: RSGEvent) {
        self.changed = true;
        match event {
            RSGEvent::SubtreeAddedOrReattached(key) => {
                self.hierarchy_changed = true;
                self.dirty_world_roots.push(key);
                self.dirty_opacity_roots.push(key);
                self.dirty_material_roots.push(key);
                self.dirty_material_value_roots.push(key);
                self.dirty_mesh_roots.push(key);
            }
            RSGEvent::SubtreeAboutToBeRemoved(_) => self.hierarchy_changed = true,
            RSGEvent::Dirty(key, flags) if flags.contains(RSGDirtyFlags::TRANSFORM) => self.dirty_world_roots.push(key),
            RSGEvent::Dirty(key, flags) if flags.contains(RSGDirtyFlags::OPACITY) => self.dirty_opacity_roots.push(key),
            RSGEvent::Dirty(key, flags) if flags.contains(RSGDirtyFlags::MATERIAL) => self.dirty_material_roots.push(key),
            RSGEvent::Dirty(key, flags) if flags.contains(RSGDirtyFlags::MATERIAL_VALUES) => self.dirty_material_value_roots.push(key),
            RSGEvent::Dirty(key, flags) if flags.contains(RSGDirtyFlags::MESH) => self.dirty_mesh_roots.push(key),
            _ => {}
        }
    }
}

impl RSGSceneObserver {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.changed = false;
        self.hierarchy_changed = false;
        self.dirty_world_roots.clear();
        self.dirty_opacity_roots.clear();
        self.dirty_material_roots.clear();
        self.dirty_material_value_roots.clear();
        self.dirty_mesh_roots.clear();
    }
}
