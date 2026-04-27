// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod offscreen;
mod scene;

pub use app::AppPlugin;
pub use scene::ScenePlugin;

const CAMERA_NAME: &str = "Camera";
const VIDEO_NAME_PREFIX: &str = "Video";
