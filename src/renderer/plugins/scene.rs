// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{CAMERA_NAME, VIDEO_NAME_PREFIX, VideoImages, offscreen::OffscreenSurface};
use bevy::{prelude::*, tasks::block_on};

pub struct ScenePlugin<const S: usize> {
    pub gltf_path: String,
    pub width: u32,
    pub height: u32,
}

impl<const S: usize> Plugin for ScenePlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scene {
            gltf_path: self.gltf_path.clone(),
            width: self.width,
            height: self.height,
        })
        .add_systems(Startup, load_scene.before(configure_scene::<S>));
    }
}

#[derive(Resource)]
struct Scene {
    gltf_path: String,
    width: u32,
    height: u32,
}

fn load_scene(
    mut commands: Commands,
    scene: Res<Scene>,
    asset_server: Res<AssetServer>,
) -> Result<()> {
    let gltf_handle: Handle<WorldAsset> = asset_server
        .load_builder()
        .load(GltfAssetLabel::Scene(0).from_asset(scene.gltf_path.clone()));
    block_on(asset_server.wait_for_asset(&gltf_handle))?;
    commands.spawn(WorldAssetRoot(gltf_handle));
    Ok(())
}

fn configure_scene<const S: usize>(
    mut commands: Commands,
    scene: Res<Scene>,
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

    Ok(())
}
