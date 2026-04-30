// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::DerefMut;

use super::{CAMERA_NAME, VIDEO_MATERIAL_NAME_PREFIX, VideoImages, offscreen::OffscreenSurface};
use bevy::{gltf::GltfMaterialName, prelude::*, world_serialization::WorldInstanceReady};

pub struct ScenePlugin<const S: usize> {
    pub scene_path: String,
    pub width: u32,
    pub height: u32,
}

impl<const S: usize> Plugin for ScenePlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scene {
            path: self.scene_path.clone(),
            width: self.width,
            height: self.height,
            ready: false,
        })
        .add_systems(Startup, load_scene)
        .add_systems(Update, mark_video_images_modified::<S>)
        .add_observer(configure_scene::<S>)
        .add_observer(play_animations);
    }
}

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

#[derive(Resource)]
pub struct Scene {
    path: String,
    width: u32,
    height: u32,
    ready: bool,
}

impl Scene {
    pub fn ready(&self) -> bool {
        self.ready
    }
}

fn load_scene(
    mut commands: Commands,
    scene: Res<Scene>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) -> Result<()> {
    // Export from Blender with "Active Actions merged"
    let (graph, index) = AnimationGraph::from_clip(
        asset_server
            .load_builder()
            .load(GltfAssetLabel::Animation(0).from_asset(scene.path.clone())),
    );
    let graph_handle = graphs.add(graph);

    let gltf_handle: Handle<WorldAsset> = asset_server
        .load_builder()
        .load(GltfAssetLabel::Scene(0).from_asset(scene.path.clone()));

    commands.spawn((
        WorldAssetRoot(gltf_handle),
        AnimationToPlay {
            graph_handle,
            index,
        },
    ));
    Ok(())
}

fn configure_scene<const S: usize>(
    _scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    cameras: Query<(Entity, &Name), With<Camera>>,
    video_materials: Query<(&GltfMaterialName, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    video_images: Res<VideoImages<S>>,
) -> Result<()> {
    let name = Name::new(CAMERA_NAME);
    if let Some(camera_entity) = cameras
        .iter()
        .find_map(|(e, n)| if *n == name { Some(e) } else { None })
    {
        commands
            .entity(camera_entity)
            .insert(OffscreenSurface::new(scene.width, scene.height));
    } else {
        return Err("Camera node not found".into());
    }

    (0..S).for_each(|i| {
        let material_name = format!("{VIDEO_MATERIAL_NAME_PREFIX}{}", i + 1);
        if let Some(video_material) = video_materials.iter().find_map(|(name, mesh_material)| {
            if name.0 == material_name {
                Some(mesh_material)
            } else {
                None
            }
        }) && let Some(mut material) = materials.get_mut(&video_material.0)
        {
            material.base_color_texture = Some(video_images.0[i].clone())
        } else {
            warn!("w0rld: Video node {material_name} not found");
        }
    });

    scene.ready = true;
    Ok(())
}

fn play_animations(
    scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation_to_play.index);

                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}

// We modify images untracked in main loop, this marks the assets as modified
fn mark_video_images_modified<const S: usize>(
    video_images: Res<VideoImages<S>>,
    mut images: ResMut<Assets<Image>>,
    _query: Single<&OffscreenSurface>, // We only need to run once the surface is installed
) {
    (0..S).for_each(|i| {
        if let Some(mut image) = images.get_mut(&video_images.0[i]) {
            image.deref_mut();
        }
    });
}
