use crate::scene::*;
use crate::transform::*;
use crate::opacity::*;
use crate::material::*;
use crate::mesh::*;
use crate::camera::*;
use crate::viewport::*;
use nalgebra_glm as glm;
use scoped_pool;

#[derive(Clone, Copy, Default)]
pub struct RSGComponentLinks {
    pub transform_key: Option<RSGTransformKey>,
    pub opacity_key: Option<RSGOpacityKey>,
    pub material_key: Option<RSGMaterialKey>,
    pub mesh_key: Option<RSGMeshKey>,
    pub camera_key: Option<RSGCameraKey>,
    pub viewport_key: Option<RSGViewportKey>
}

#[derive(Default)]
pub struct RSGComponentContainer {
    pub transforms: RSGTransformComponentList,
    pub opacities: RSGOpacityComponentList,
    pub materials: RSGMaterialComponentList,
    pub material_data: RSGMaterialComponentData,
    pub meshes: RSGMeshComponentList,
    pub mesh_data: RSGMeshComponentData,
    pub cameras: RSGCameraComponentList,
    pub viewports: RSGViewportComponentList
}

impl RSGComponentContainer {
    pub fn add_default_root<ObserverT>(&mut self, scene: &mut RSGScene<RSGComponentLinks, ObserverT>) -> RSGNodeKey
        where ObserverT: RSGObserver
    {
        scene.set_root(RSGNode::with_component_links(
            RSGComponentBuilder::new(self).transform(glm::one()).opacity(1.0).links()))
    }

    pub fn remove(&mut self, component_links: RSGComponentLinks) {
        if let Some(key) = component_links.transform_key {
            self.transforms.remove(key);
        }
        if let Some(key) = component_links.opacity_key {
            self.opacities.remove(key);
        }
        if let Some(key) = component_links.material_key {
            self.materials.remove(key);
        }
        if let Some(key) = component_links.mesh_key {
            self.meshes.remove(key);
        }
        if let Some(key) = component_links.camera_key {
            self.cameras.remove(key);
        }
        if let Some(key) = component_links.viewport_key {
            self.viewports.remove(key);
        }
    }

    pub fn is_opaque(&self, links: &RSGComponentLinks) -> bool {
        if let Some(opacity_key) = links.opacity_key {
            if self.opacities[opacity_key].inherited_opacity < 1.0 {
                return false;
            }
        }
        if let Some(material_key) = links.material_key {
            if self.material_data[material_key].graphics_state.blend.blend_enable {
                return false;
            }
        }
        return true;
    }

    pub fn print_scene<ObserverT>(&self, scene: &RSGScene<RSGComponentLinks, ObserverT>,
        start_node_key: RSGNodeKey, max_depth: Option<u32>)
        where ObserverT: RSGObserver
    {
        for (key, depth) in scene.traverse(start_node_key) {
            if max_depth.is_some() && depth > max_depth.unwrap() {
                println!("... <truncated>");
                break;
            }

            let component_links = scene.get_component_links(key);
            let indent = (0..depth).map(|_| "    ").collect::<String>();
            println!("{}----{:?} alpha={}", indent, key, !self.is_opaque(component_links));

            if let Some(transform_key) = component_links.transform_key {
                let t = self.transforms[transform_key];
                println!("{}    local translate=({}, {}, {}) world translate=({}, {}, {})", indent,
                    t.local_transform[12], t.local_transform[13], t.local_transform[14],
                    t.world_transform[12], t.world_transform[13], t.world_transform[14]);
            }

            if let Some(opacity_key) = component_links.opacity_key {
                let o = self.opacities[opacity_key];
                println!("{}    opacity={} inherited opacity={}", indent, o.opacity, o.inherited_opacity);
            }

            if let Some(material_key) = component_links.material_key {
                let material = &self.material_data[material_key];
                println!("{}    material property value count={}", indent, material.property_values.len());
            }

            if let Some(mesh_key) = component_links.mesh_key {
                let mesh_component = self.meshes[mesh_key];
                let mesh = &self.mesh_data[mesh_key];
                println!("{}    mesh submesh count={} bounds={} active viewport={:?} sort dist={}",
                    indent, mesh.submeshes.len(), mesh.bounds,
                    mesh_component.viewport_node_key, mesh_component.sorting_distance);
            }

            if let Some(camera_key) = component_links.camera_key {
                let c = &self.cameras[camera_key];
                println!("{}    camera={:?}", indent, c.camera);
            }

            if let Some(viewport_key) = component_links.viewport_key {
                let v = &self.viewports[viewport_key];
                println!("{}    viewport rect={:?} active camera={:?}",
                    indent, v.rect, v.camera_node_key);
            }
        }
    }
}

pub struct RSGComponentBuilder<'a> {
    links: RSGComponentLinks,
    container: &'a mut RSGComponentContainer
}

impl<'a> RSGComponentBuilder<'a> {
    pub fn new(container: &'a mut RSGComponentContainer) -> Self {
        RSGComponentBuilder {
            links: Default::default(),
            container: container
        }
    }

    pub fn transform(&mut self, local_transform: glm::Mat4) -> &mut Self {
        self.links.transform_key = Some(self.container.transforms.insert(RSGTransformComponent::new(local_transform)));
        self
    }

    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.links.opacity_key = Some(self.container.opacities.insert(RSGOpacityComponent::new(opacity)));
        self
    }

    pub fn material(&mut self, material: RSGMaterial) -> &mut Self {
        let key = self.container.materials.insert(RSGMaterialComponent::new());
        self.links.material_key = Some(key);
        self.container.material_data.insert(key, material);
        self
    }

    pub fn mesh(&mut self, mesh: RSGMesh) -> &mut Self {
        let key = self.container.meshes.insert(RSGMeshComponent::new());
        self.links.mesh_key = Some(key);
        self.container.mesh_data.insert(key, mesh);
        self
    }

    pub fn camera(&mut self, camera: RSGCamera) -> &mut Self {
        self.links.camera_key = Some(self.container.cameras.insert(RSGCameraComponent::new(camera)));
        self
    }

    pub fn viewport(&mut self, rect: RSGViewportRect, camera_node_key: RSGNodeKey) -> &mut Self {
        self.links.viewport_key = Some(self.container.viewports.insert(RSGViewportComponent::new(rect, camera_node_key)));
        self
    }

    pub fn links(&mut self) -> RSGComponentLinks {
        self.links
    }
}

fn calculate_camera_world_properties(world_transform: &glm::Mat4) -> RSGCameraWorldTransformDerivedProperties
{
    let camera_world = world_transform;
    let camera_position = glm::vec3(camera_world[12], camera_world[13], camera_world[14]);
    let scaling_correct_camera_world = glm::transpose(&glm::inverse(&glm::mat4_to_mat3(&camera_world)));
    let camera_direction = glm::normalize(&(scaling_correct_camera_world * glm::vec3(0.0, 0.0, -1.0)));
    RSGCameraWorldTransformDerivedProperties {
        position: camera_position,
        direction: camera_direction
    }
}

fn calculate_sorting_distance(world_transform: &glm::Mat4, bounds: &RSGAabb,
    camera_properties: &RSGCameraWorldTransformDerivedProperties) -> f32
{
    let center = bounds.center();
    let world_center = glm::vec4_to_vec3(&(world_transform * glm::vec4(center.x, center.y, center.z, 1.0)));
    glm::dot(&(world_center - camera_properties.position), &camera_properties.direction)
}

fn update_inherited_opacities<ObserverT>(
    opacity_components: RSGOpacityComponentList,
    scene: &RSGScene<RSGComponentLinks, ObserverT>,
    subtree_roots: &[RSGNodeKey]) -> RSGOpacityComponentList
    where ObserverT: RSGObserver
{
    let mut opacities = opacity_components;
    for subtree_root_key in subtree_roots {
        for (key, _) in scene.traverse(*subtree_root_key) {
            if let Some(opacity_key) = scene.get_component_links(key).opacity_key {
                let mut inherited_opacity = opacities[opacity_key].opacity;
                for key in scene.ancestors(key) {
                    if let Some(opacity_key) = scene.get_component_links(key).opacity_key {
                        inherited_opacity *= opacities[opacity_key].inherited_opacity;
                        break;
                    }
                }
                opacities[opacity_key].inherited_opacity = inherited_opacity;
            }
        }
    }
    opacities
}

// calculates (for component):
//   - world transform (transform)
//   - world position/direction (camera)
//   - sorting distance, active viewport (mesh)
//   - inherited opacity (opacity)
pub fn update_inherited_properties<ObserverT>(
    components: &mut RSGComponentContainer,
    scene: &RSGScene<RSGComponentLinks, ObserverT>,
    dirty_world_roots: &[RSGNodeKey],
    dirty_opacity_roots: &[RSGNodeKey],
    pool: &scoped_pool::Pool)
    where ObserverT: RSGObserver + Sync
{
    pool.scoped(|scope| {
        let (tx, rx) = std::sync::mpsc::channel();
        if !dirty_opacity_roots.is_empty() {
            let opacities = std::mem::replace(&mut components.opacities, Default::default());
            scope.execute(move || {
                tx.send(update_inherited_opacities(opacities, scene, dirty_opacity_roots)).unwrap();
            });
        }

        for subtree_root_key in dirty_world_roots {
            for (key, _) in scene.traverse(*subtree_root_key) {
                let links = scene.get_component_links(key);
                if let Some(transform_key) = links.transform_key {
                    let mut world_transform = components.transforms[transform_key].local_transform;
                    for key in scene.ancestors(key) {
                        if let Some(transform_key) = scene.get_component_links(key).transform_key {
                            world_transform *= components.transforms[transform_key].world_transform;
                            break;
                        }
                    }
                    components.transforms[transform_key].world_transform = world_transform;
                    if let Some(camera_key) = links.camera_key {
                        let cam_prop = calculate_camera_world_properties(&world_transform);
                        components.cameras[camera_key].world_properties = cam_prop;
                    }
                }
            }

            for (viewport_node_key, _) in scene.traverse(*subtree_root_key) {
                if let Some(viewport_key) = scene.get_component_links(viewport_node_key).viewport_key {
                    let cam_links = scene.get_component_links(components.viewports[viewport_key].camera_node_key);
                    let cam_props = components.cameras[cam_links.camera_key.unwrap()].world_properties;
                    for (key, _) in scene.traverse(viewport_node_key) {
                        let links = scene.get_component_links(key);
                        if let Some(mesh_key) = links.mesh_key {
                            let mesh_component = &mut components.meshes[mesh_key];
                            match links.transform_key {
                                Some(transform_key) => {
                                    let dist = calculate_sorting_distance(
                                        &components.transforms[transform_key].world_transform,
                                        &components.mesh_data[mesh_key].bounds,
                                        &cam_props);
                                    mesh_component.sorting_distance = dist;
                                    mesh_component.viewport_node_key = Some(viewport_node_key);
                                }
                                _ => {
                                    mesh_component.sorting_distance = 0.0;
                                    mesh_component.viewport_node_key = None;
                                }
                            }
                        }
                    }
                }
            }
        }

        if !dirty_opacity_roots.is_empty() {
            components.opacities = rx.recv().unwrap();
        }
    });
}
