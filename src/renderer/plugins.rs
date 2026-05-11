// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod offscreen;
mod scene;

use std::sync::mpsc::Sender;

pub use app::AppPlugin;
use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::TextureFormat},
};
pub use scene::{AssetTracker, Scene, ScenePlugin};

const CAMERA_NAME: &str = "Camera";
const VIDEO_NAME_PREFIX: &str = "Video";

pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;
const PIXEL_SIZE: usize = 4;

#[derive(Resource)]
pub struct VideoImages<const S: usize>(pub [Handle<Image>; S]);

#[derive(Resource, Clone, ExtractResource)]
pub struct RenderSender(Sender<Result<Vec<u8>>>);
