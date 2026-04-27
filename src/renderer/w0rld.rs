// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    sync::mpsc::{Receiver, channel},
    time::Duration,
};

use super::plugins;
use bevy::{platform::time::Instant, prelude::*, time::TimeUpdateStrategy};

pub struct W0rld<const S: usize> {
    app: App,
    video_images: [Option<Handle<Image>>; S],
    time: Instant,
    rx: Receiver<Result<Vec<u8>>>,
}

impl<const S: usize> W0rld<S> {
    pub fn new(gltf_path: String, width: u32, height: u32) -> Result<Self> {
        let (tx, rx) = channel();
        let now = Instant::now();
        let mut app = App::new();
        app.add_plugins((
            plugins::AppPlugin { tx },
            plugins::ScenePlugin {
                gltf_path,
                width,
                height,
            },
        ))
        .insert_resource(TimeUpdateStrategy::ManualInstant(now));

        app.finish();
        app.cleanup();

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
            video_images: [const { None }; S],
        })
    }

    pub fn render(&mut self, time: f64, inframes: [&[u8]; S]) -> Result<Vec<u8>> {
        self.time += Duration::from_secs(time as u64);
        self.app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.time));
        //XXX push frames to video_images
        self.app.update();
        self.rx.recv()?
    }
}
