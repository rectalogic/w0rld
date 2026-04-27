// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    sync::mpsc::{Receiver, channel},
    time::Duration,
};

use super::plugins;
use bevy::{
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
    pub fn new(gltf_path: String, width: u32, height: u32) -> Result<Self> {
        let (tx, rx) = channel();
        let now = Instant::now();
        let mut app = App::new();
        app.add_plugins((
            plugins::AppPlugin { tx },
            plugins::ScenePlugin::<S> {
                gltf_path,
                width,
                height,
            },
        ))
        .insert_resource(TimeUpdateStrategy::ManualInstant(now));

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

        app.insert_resource(plugins::VideoImages::<S>(video_images));

        // Preroll 2 frames to let pipelines load etc.
        // https://github.com/bevyengine/bevy/issues/20756
        app.update();
        rx.recv()??;
        app.update();
        rx.recv()??;

        Ok(Self {
            app,
            rx,
            time: now,
            last_time: None,
        })
    }

    pub fn render(&mut self, time: f64, inframes: [&[u8]; S]) -> Result<Vec<u8>> {
        let dt = if let Some(last_time) = self.last_time.replace(time) {
            time - last_time
        } else {
            0.0
        };
        self.time += Duration::from_secs(dt as u64);
        self.app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.time));

        let video_images = self
            .app
            .world()
            .resource::<plugins::VideoImages<S>>()
            .0
            .clone();
        let mut images = self.app.world_mut().resource_mut::<Assets<Image>>();
        for (image_handle, inframe) in video_images.iter().zip(inframes) {
            if let Some(mut image) = images.get_mut(image_handle) {
                image.data.as_mut().unwrap().copy_from_slice(inframe);
            }
        }

        self.app.update();
        self.rx.recv()?
    }
}
