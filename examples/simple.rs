use rsg::scene::*;
use rsg::components::*;
use rsg::observer::*;
use rsg::camera::*;
use rsg::viewport::*;
use rsg::material::*;
use rsg::mesh::*;
use nalgebra_glm as glm;
use smallvec::*;

type Scene = RSGScene::<RSGComponentLinks, RSGSceneObserver>;
type MeshBuffers = std::collections::HashMap<u32, RSGMeshBuffer>;
type ShaderSets = std::collections::HashMap<u32, RSGMaterialShaderSet>;

static TRIANGLE_BUF_ID: u32 = 1;

static COLOR_SH_ID: u32 = 1;

fn make_camera(components: &mut RSGComponentContainer, local_transform: glm::Mat4, camera: RSGCamera) -> RSGNode<RSGComponentLinks> {
    RSGNode::with_component_links(
        RSGComponentBuilder::new(components)
        .transform(local_transform)
        .camera(camera)
        .links())
}

fn make_viewport(components: &mut RSGComponentContainer, rect: RSGViewportRect, camera_node_key: RSGNodeKey)
    -> RSGNode<RSGComponentLinks>
{
    RSGNode::with_component_links(
        RSGComponentBuilder::new(components)
        .viewport(rect, Some(camera_node_key))
        .links())
}

fn make_color_material(shader_sets: &mut ShaderSets) -> RSGMaterial {
    let mvp_name = "mvp".to_owned();
    let color_name = "color".to_owned();

    if !shader_sets.contains_key(&COLOR_SH_ID) {
        let shader_set = RSGMaterialShaderSet {
            vertex_shader: "".to_owned(),
            fragment_shader: "".to_owned(),
            properties: vec![
                RSGMaterialProperty::Mat4(mvp_name.clone(), glm::one()),
                RSGMaterialProperty::Vec3(color_name.clone(), glm::zero())
            ]
        };
        shader_sets.insert(COLOR_SH_ID, shader_set);
    }

    let mut material = RSGMaterial {
        shader_set_id: COLOR_SH_ID,
        property_values: Default::default(),
        graphics_state: Default::default()
    };
    material.property_values.insert(mvp_name, RSGMaterialPropertyValue::Builtin(RSGMaterialBuiltinValue::ModelViewProjectionMatrix));
    material.property_values.insert(color_name, RSGMaterialPropertyValue::Custom(RSGMaterialCustomValue::Vec3(glm::vec3(1.0, 0.0, 0.0))));
    material
}

fn make_triangle(components: &mut RSGComponentContainer, buffers: &mut MeshBuffers, shader_sets: &mut ShaderSets,
    local_transform: glm::Mat4, opacity: f32) -> RSGNode<RSGComponentLinks>
{
    if !buffers.contains_key(&TRIANGLE_BUF_ID) {
        let buf = RSGMeshBuffer {
            data: vec![
                -1.0, -1.0, 0.0,
                1.0, -1.0, 0.0,
                0.5, 1.0, 0.0
            ],
            source: Default::default()
        };
        buffers.insert(TRIANGLE_BUF_ID, buf);
    }

    let mesh = RSGMesh {
        vertex_views: smallvec::smallvec![RSGMeshBufferView {
            buffer_id: TRIANGLE_BUF_ID,
            offset: 0,
            size: 9 * 4,
            stride: 3 * 4
        }],
        submeshes: smallvec::smallvec![RSGSubMesh {
            topology: RSGMeshTopology::Triangles,
            vertex_count: 3,
            inputs: smallvec::smallvec![RSGMeshVertexInput::Position(RSGMeshVertexInputType::Vec3, 0, 0)],
            index_count: None,
            index_view: None
        }],
        bounds: RSGAabb {
            minimum: glm::vec3(-1.0, -1.0, 0.0),
            maximum: glm::vec3(1.0, 1.0, 0.0)
        },
    };

    let material = make_color_material(shader_sets);

    RSGNode::with_component_links(
        RSGComponentBuilder::new(components)
        .transform(local_transform)
        .opacity(opacity)
        .material(material)
        .mesh(mesh)
        .links())
}

#[derive(Default)]
struct Data {
    components: RSGComponentContainer,
    mesh_buffers: MeshBuffers,
    shader_sets: ShaderSets,
    opaque_list: RSGRenderList,
    alpha_list: RSGRenderList,
    root_key: RSGNodeKey,
    frame_count: u32
}

fn sync(d: &mut Data, scene: &mut Scene) {
    println!("Frame {} sync", d.frame_count);
    if d.frame_count == 0 {
        let cam_key = scene.append(d.root_key, make_camera(&mut d.components,
            glm::translation(&glm::vec3(0.0, 0.0, 600.0)),
            RSGCamera::Perspective(RSGPerspectiveProjection {
                aspect_ratio: 16.0 / 9.0,
                fov: 45.0,
                near: 0.01,
                far: 1000.0
            })));
        let vp_key = scene.append(d.root_key, make_viewport(&mut d.components,
            RSGViewportRect { x: 0, y: 0, w: 800, h: 600 },
            cam_key));

        let mut transaction = RSGSubtreeAddTransaction::new();
        let tri1_key = scene.append_with_transaction(vp_key, make_triangle(&mut d.components, &mut d.mesh_buffers, &mut d.shader_sets,
            glm::translation(&glm::vec3(0.5, 0.5, -10.0)), 1.0),
            &mut transaction);
        scene.append_with_transaction(tri1_key, make_triangle(&mut d.components, &mut d.mesh_buffers, &mut d.shader_sets,
            glm::translation(&glm::vec3(0.3, 0.3, -2.0)), 1.0),
            &mut transaction);
        scene.commit(transaction);
    }
}

fn prepare(d: &mut Data, scene: &Scene, observer: &RSGSceneObserver, pool: &scoped_pool::Pool) {
    println!("Frame {} prepare, changes={:?}", d.frame_count, observer);
    if observer.changed {
        prepare_scene(&mut d.components, &scene,
            &observer.dirty_world_roots, &observer.dirty_opacity_roots,
            &mut d.opaque_list, &mut d.alpha_list,
            &pool);
        d.components.print_scene(&scene, d.root_key, Some(10));
    }
}

fn render(d: &mut Data, _scene: &Scene) {
    println!("Frame {} render", d.frame_count);
    println!("  Opaque list={:?}", d.opaque_list);
    println!("  Alpha list={:?}", d.alpha_list);
}

fn frame(d: &mut Data, scene: &mut Scene, pool: &scoped_pool::Pool) {
    let mut observer = RSGSceneObserver::new();
    scene.set_observer(observer);
    sync(d, scene);
    observer = scene.take_observer().unwrap();
    prepare(d, scene, &observer, pool);
    render(d, scene);
    d.frame_count += 1;
}

fn main() {
    let pool = scoped_pool::Pool::new(4);
    let mut scene = Scene::new();
    let mut d: Data = Default::default();
    d.root_key = d.components.add_default_root(&mut scene);

    frame(&mut d, &mut scene, &pool);
    frame(&mut d, &mut scene, &pool);
    frame(&mut d, &mut scene, &pool);

    pool.shutdown();
}
