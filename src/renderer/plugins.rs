// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod offscreen;
mod scene;

pub use app::AppPlugin;
use bevy::{prelude::*, render::render_resource::TextureFormat};
pub use scene::{AssetTracker, Scene, ScenePlugin};

const CAMERA_NAME: &str = "Camera";
const VIDEO_MATERIAL_NAME_PREFIX: &str = "Video";

pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;
const PIXEL_SIZE: usize = 4;

#[derive(Resource)]
pub struct VideoImages<const S: usize>(pub [Handle<Image>; S]);
