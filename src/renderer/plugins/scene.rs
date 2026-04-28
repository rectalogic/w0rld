// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{CAMERA_NAME, VIDEO_NAME_PREFIX, VideoImages, offscreen::OffscreenSurface};
use bevy::{prelude::*, world_serialization::WorldInstanceReady};

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
        .add_observer(configure_scene::<S>);
    }
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
) -> Result<()> {
    let gltf_handle: Handle<WorldAsset> = asset_server
        .load_builder()
        .load(GltfAssetLabel::Scene(0).from_asset(scene.path.clone()));
    commands.spawn(WorldAssetRoot(gltf_handle));
    Ok(())
}

fn configure_scene<const S: usize>(
    _scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    cameras: Query<(Entity, &Name), With<Camera>>,
    video_materials: Query<(&Name, &MeshMaterial3d<StandardMaterial>)>,
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
        let name = Name::new(format!("{VIDEO_NAME_PREFIX}{}", i + 1));
        if let Some(video_material) = video_materials
            .iter()
            .find_map(|(n, m)| if *n == name { Some(m) } else { None })
            && let Some(mut material) = materials.get_mut(&video_material.0)
        {
            material.base_color_texture = Some(video_images.0[i].clone())
        } else {
            warn!("Missing video node {}", name.as_str());
        }
    });

    scene.ready = true;
    Ok(())
}
