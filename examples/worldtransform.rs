use rsg::scene::*;
use rsg::components::*;
use rsg::observer::*;
use nalgebra_glm as glm;

type Scene = RSGScene::<RSGComponentLinks, RSGSceneObserver>;

#[derive(Default)]
struct Data {
    components: RSGComponentContainer,
    root_key: Option<RSGNodeKey>,
    node1_key: Option<RSGNodeKey>,
    node11_key: Option<RSGNodeKey>,
    node111_key: Option<RSGNodeKey>,
    node112_key: Option<RSGNodeKey>
}

fn sync_scene(d: &mut Data, scene: &mut Scene, stage: i32) {
    // simulate doing something with the scenegraph
    match stage {
        0 => {
            // ROOT
            d.root_key = Some(scene.set_root(RSGNode::with_component_links(
                RSGComponentBuilder::new(&mut d.components).transform(glm::one()).opacity(1.0).links())));
        },
        2 => {
            // ROOT(NODE1)
            // where NODE1 has a -100, 200 translation on it, and an opacity of 0.5
            d.node1_key = Some(scene.append(d.root_key.unwrap(), RSGNode::with_component_links(
                RSGComponentBuilder::new(&mut d.components)
                .transform(glm::translation(&glm::vec3(-100.0, 200.0, 0.0)))
                .opacity(0.5)
                .links())));
        },
        3 => {
            // ROOT(NODE1(NODE11))
            // where NODE11 has a (10, -20) translation on it -> world pos is (-90, 180)
            d.node11_key = Some(scene.append(d.node1_key.unwrap(), RSGNode::with_component_links(
                RSGComponentBuilder::new(&mut d.components)
                .transform(glm::translation(&glm::vec3(10.0, -20.0, 0.0)))
                .opacity(1.0)
                .links())));
        },
        4 => {
            // ROOT(NODE1(NODE11(NODE111, NODE112)))
            // both children of NODE11 have local (0, 0), so their world is equal to NODE11's
            // NODE111 has an opacity of 0.2 (so inherited opacities are 0.1 and 0.5)
            d.node112_key = Some(scene.prepend(d.node11_key.unwrap(), RSGNode::with_component_links(
                RSGComponentBuilder::new(&mut d.components).transform(glm::one()).opacity(1.0).links())));
            d.node111_key = Some(scene.prepend(d.node11_key.unwrap(), RSGNode::with_component_links(
                RSGComponentBuilder::new(&mut d.components).transform(glm::one()).opacity(0.2).links())));
        },
        5 => {
            // ROOT(NODE11(NODE111, NODE112))
            // NODE1 goes away so world pos for NODE11 (and its children) becomes (10, -20) and inherited opacities are back to 0.2 and 1
            let node_count_before = scene.node_count();
            assert!(d.components.transforms.len() == node_count_before && d.components.opacities.len() == node_count_before);
            {
                let node1_key = d.node1_key.take().unwrap();
                let node1_component_links = scene.remove_without_children(node1_key);
                d.components.remove(node1_component_links);
            }
            assert!(scene.node_count() == node_count_before - 1 && d.components.transforms.len() == node_count_before - 1 && d.components.opacities.len() == node_count_before - 1);
        },
        6 => {
            // Now just change the transform on NODE11. Expected: children follow (all three are (15, -10) world).
            d.components.transforms[scene.get_component_links(d.node11_key.unwrap()).transform_key.unwrap()].local_transform = glm::translation(&glm::vec3(15.0, -10.0, 0.0));
            scene.mark_dirty(d.node11_key.unwrap(), RSGDirtyFlags::TRANSFORM);
        },
        7 => {
            // Now just change the opacity on NODE11. Children's inherited opacity should then take that into account (0.15 and 0.75).
            d.components.opacities[scene.get_component_links(d.node11_key.unwrap()).opacity_key.unwrap()].opacity = 0.75;
            scene.mark_dirty(d.node11_key.unwrap(), RSGDirtyFlags::OPACITY);
        },
        8 => {
            // Add a new subtree with 100000 nodes.
            let mut k = d.root_key.unwrap();
            let mut t = RSGSubtreeAddTransaction::new();
            for _ in 0..100000 {
                k = scene.append_with_transaction(k, RSGNode::with_component_links(
                    RSGComponentBuilder::new(&mut d.components).transform(glm::one()).opacity(1.0).links()),
                    &mut t);
            }
            scene.commit(t);
        },
        _ => {}
    }
}

fn main() {
    let mut scene = Scene::new();
    let mut obs = RSGSceneObserver::new();
    let mut d = Default::default();
    let mut opaque_list = vec![];
    let mut alpha_list = vec![];
    let pool = scoped_pool::Pool::new(4);

    for stage in 0..9 {
        println!("Simulation step {}", stage);
        scene.set_observer(obs);

        let timestamp = std::time::Instant::now();
        sync_scene(&mut d, &mut scene, stage);
        println!("  scene synchronization took {} microseconds", timestamp.elapsed().as_micros());

        obs = scene.take_observer().unwrap();
        if obs.changed {
            println!("  total node count is {}", scene.node_count());
            println!("  roots for subtrees with dirty world transform: {:?}", obs.dirty_world_roots);
            println!("  roots for subtrees with dirty inherited opacity: {:?}", obs.dirty_opacity_roots);
            let timestamp = std::time::Instant::now();
            prepare_scene(&mut d.components, &scene, &obs.dirty_world_roots, &obs.dirty_opacity_roots,
                &mut opaque_list, &mut alpha_list, &pool);
            println!("  inherited property update took {} microseconds", timestamp.elapsed().as_micros());
            obs.reset();
            d.components.print_scene(&scene, d.root_key.unwrap(), Some(5));
        } else {
            println!("  no changes");
        }
    }

    pool.shutdown();
}
