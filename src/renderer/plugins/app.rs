// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::{OnceLock, mpsc::Sender};

use super::{RenderSender, offscreen::OffscreenPlugin};
use bevy::{
    asset::UnapprovedPathMode,
    ecs::error::{ErrorContext, FallbackErrorHandler},
    prelude::*,
    render::{RenderPlugin, pipelined_rendering::PipelinedRenderingPlugin},
    window::ExitCondition,
    winit::WinitPlugin,
};

static ERROR_TX: OnceLock<Sender<Result<Vec<u8>>>> = OnceLock::new();

pub struct AppPlugin {
    pub tx: Sender<Result<Vec<u8>>>,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let _ = ERROR_TX.set(self.tx.clone());
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                })
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
                .disable::<WinitPlugin>()
                .disable::<PipelinedRenderingPlugin>(),
            OffscreenPlugin,
        ))
        .insert_resource(RenderSender(self.tx.clone()))
        .insert_resource(FallbackErrorHandler(error_handler));
    }
}

fn error_handler(err: BevyError, _ctx: ErrorContext) {
    if let Some(tx) = ERROR_TX.get() {
        let _ = tx.send(Err(err));
    }
}
