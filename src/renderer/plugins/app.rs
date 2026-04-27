// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::Sender;

use super::offscreen::OffscreenPlugin;
use bevy::{prelude::*, render::RenderPlugin, window::ExitCondition, winit::WinitPlugin};

pub struct AppPlugin {
    pub tx: Sender<Vec<u8>>,
    pub gltf_path: String,
    pub width: u32,
    pub height: u32,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                })
                .build()
                .disable::<WinitPlugin>(),
            OffscreenPlugin {
                tx: self.tx.clone(),
            },
        ))
        .insert_resource(Scene {
            gltf_path: self.gltf_path.clone(),
            width: self.width,
            height: self.height,
        })
        .add_systems(Startup, load_scene.before(configure_camera));
    }
}

#[derive(Resource)]
struct Scene {
    gltf_path: String,
    width: u32,
    height: u32,
}

fn load_scene(mut commands: Commands, scene: Res<Scene>) {
    //XXX blocking load of gltf
}

fn configure_camera(cameras: Query<&Name, With<Camera>>) {
    //XXX find camera named Camera and insert OffscreenSurface::new(WIDTH, HEIGHT)
}
