// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{CAMERA_NAME, offscreen::OffscreenSurface};
use bevy::{prelude::*, tasks::block_on};

pub struct ScenePlugin {
    pub gltf_path: String,
    pub width: u32,
    pub height: u32,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scene {
            gltf_path: self.gltf_path.clone(),
            width: self.width,
            height: self.height,
        })
        .add_systems(Startup, load_scene.before(configure_scene));
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
        .override_unapproved()
        .load(GltfAssetLabel::Scene(0).from_asset(scene.gltf_path.clone()));
    block_on(asset_server.wait_for_asset(&gltf_handle))?;
    commands.spawn(WorldAssetRoot(gltf_handle));
    Ok(())
}

fn configure_scene(
    mut commands: Commands,
    scene: Res<Scene>,
    cameras: Query<(Entity, &Name), With<Camera>>,
) -> Result<()> {
    let name = Name::new(CAMERA_NAME);
    if let Some(camera_entity) = cameras
        .iter()
        .find_map(|(e, n)| if *n == name { Some(e) } else { None })
    {
        commands
            .entity(camera_entity)
            .insert(OffscreenSurface::new(scene.width, scene.height));
        Ok(())
    } else {
        Err("Camera not found".into())
    }
}
