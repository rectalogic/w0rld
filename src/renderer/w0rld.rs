// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    sync::mpsc::{Receiver, channel},
    time::Duration,
};

use super::plugins;
use bevy::{
    app::PluginsState,
    asset::RenderAssetUsages,
    platform::time::Instant,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension},
    time::TimeUpdateStrategy,
};

pub struct W0rld<const S: usize> {
    app: App,
    time: Instant,
    last_time: Option<f64>,
    rx: Receiver<Result<Vec<u8>>>,
}

impl<const S: usize> W0rld<S> {
    pub fn new(scene_path: String, width: u32, height: u32) -> Result<Self> {
        let (render_tx, render_rx) = channel();
        let now = Instant::now();
        let mut app = App::new();
        app.add_plugins((
            plugins::AppPlugin { tx: render_tx },
            plugins::ScenePlugin::<S> {
                scene_path,
                width,
                height,
            },
        ))
        .insert_resource(TimeUpdateStrategy::ManualInstant(now));

        while app.plugins_state() == PluginsState::Adding {
            bevy::tasks::tick_global_task_pools_on_main_thread();
        }
        app.finish();
        app.cleanup();

        let mut images = app
            .world_mut()
            .get_resource_mut::<Assets<Image>>()
            .ok_or("Image assets not found")?;

        let video_images: [Handle<Image>; S] = (0..S)
            .map(|_| {
                images.add(Image::new_fill(
                    Extent3d {
                        width,
                        height,
                        ..default()
                    },
                    TextureDimension::D2,
                    &[0, 0, 0, 255],
                    plugins::TEXTURE_FORMAT,
                    RenderAssetUsages::default(),
                ))
            })
            .collect::<Vec<Handle<Image>>>()
            .try_into()
            .unwrap();

        let (asset_tx, asset_rx) = channel();
        app.insert_resource(plugins::VideoImages::<S>(video_images))
            .insert_resource(plugins::AssetTracker::new(asset_tx));

        Self::preroll(&mut app, &render_rx)?;

        if let Some(asset_tracker) = app.world_mut().remove_resource::<plugins::AssetTracker>() {
            asset_tracker.wait(asset_rx)?;
        }

        // Preroll frames while we wait for asset to load, also to let render pipelines load etc.
        // https://github.com/bevyengine/bevy/issues/20756
        while !app.world().resource::<plugins::Scene>().ready() {
            Self::preroll(&mut app, &render_rx)?;
            bevy::platform::thread::sleep(Duration::from_millis(40));
        }
        // Self::preroll(&mut app, &rx)?;
        // Self::preroll(&mut app, &rx)?;

        Ok(Self {
            app,
            rx: render_rx,
            time: now,
            last_time: None,
        })
    }

    fn preroll(app: &mut App, rx: &Receiver<Result<Vec<u8>>>) -> Result<()> {
        app.update();
        if let Ok(result) = rx.try_recv() {
            result?;
        }
        Ok(())
    }

    pub fn render(&mut self, time: f64, inframes: [&[u8]; S]) -> Result<Vec<u8>> {
        let dt = if let Some(last_time) = self.last_time.replace(time) {
            time - last_time
        } else {
            0.0
        };
        self.time += Duration::from_secs_f64(dt);
        self.app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.time));

        if S > 0 {
            let video_images = self
                .app
                .world()
                .resource::<plugins::VideoImages<S>>()
                .0
                .clone();
            let mut images = self.app.world_mut().resource_mut::<Assets<Image>>();
            for (image_handle, inframe) in video_images.iter().zip(inframes) {
                // Bypass asset change tracking since we are outside the Main schedule.
                // We mark the image modified in mark_video_images_modified
                if let Some(image) = images.get_mut_untracked(image_handle)
                    && let Some(ref mut data) = image.data
                {
                    data.copy_from_slice(inframe);
                }
            }
        }

        self.app.update();
        self.rx.recv()?
    }
}
