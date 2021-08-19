use crate::map_data::MapData;
use crate::map_pipeline::{MapMaterial, MapPipeline};
use bevy::core::FixedTimestep;
use bevy::pbr::render_graph::PBR_PIPELINE_HANDLE;
use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;
use bevy::render::render_graph::base::MainPass;
use bevy::render::wireframe::Wireframe;
use bevy_inspector_egui::InspectableRegistry;

mod map_data;
mod map_generation;
mod map_pipeline;

/// A bundle containing all the components required to spawn a map.
#[derive(Bundle, Default)]
struct MapBundle {
    map_data: MapData,
    mesh: Handle<Mesh>,
    material: Handle<MapMaterial>,
    draw: Draw,
    visible: Visible,
    render_pipelines: RenderPipelines,
    main_pass: MainPass,
    transform: Transform,
    global_transform: GlobalTransform,
}

/// A plugin that procedurally generates a map.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<MapMaterial>()
            .init_resource::<MapPipeline>()
            .add_startup_system(setup)
            .add_system(update_map.with_run_criteria(FixedTimestep::step(0.1)));

        // getting registry from world
        let mut registry = app
            .world
            .get_resource_or_insert_with(InspectableRegistry::default);

        // register components to be able to edit them in the inspector (works recursively)
        registry.register::<MapData>();
    }
}

/// Creates the map and spawns an entity with its mesh and its material.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MapMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    map_pipeline: Res<MapPipeline>,
) {
    // create the render pipelines for the maps
    let render_pipelines = RenderPipelines::from_pipelines(vec![
        // RenderPipeline::new(PBR_PIPELINE_HANDLE.typed()),
        RenderPipeline::new(map_pipeline.pipeline.clone()),
    ]);
    /*
    // prepare the map
    let map_data = MapData::default();
    let (mesh, material) = map_data.generate();
    let mesh = meshes.add(mesh);
    let material = materials.add(material);

    // create the map entity
    commands.spawn_bundle(MapBundle {
        material,
        map_data,
        mesh,
        render_pipelines: render_pipelines.clone(),
        transform: Transform::from_xyz(0.0, -10.0, 0.0),
        ..Default::default()
    });
     */

    let map_data = MapData::default();
    let (mesh, material) = map_data.generate();
    let mesh = meshes.add(mesh);
    let material = materials.add(material);

    commands
        .spawn_bundle(MapBundle {
            material,
            map_data,
            mesh,
            render_pipelines,
            transform: Transform::from_xyz(-15.0, -20.0, -120.0),
            ..Default::default()
        })
        .insert(standard_materials.add(StandardMaterial::default()))
        .insert(Wireframe);
}

/// Updates the mesh and the material of the map, if the map data of it changed.
fn update_map(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MapMaterial>>,
    query: Query<(&Handle<Mesh>, &Handle<MapMaterial>, &MapData), Changed<MapData>>,
) {
    // regenerate the mesh and the material of the map
    for (mesh, material, map_data) in query.iter() {
        let mesh = meshes.get_mut(mesh).unwrap();
        let material = materials.get_mut(material).unwrap();
        let (new_mesh, new_material) = map_data.generate();
        *mesh = new_mesh;
        *material = new_material;
    }
}
