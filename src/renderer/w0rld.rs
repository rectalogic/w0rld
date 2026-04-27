// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::{Receiver, channel};

use super::plugins;
use bevy::prelude::*;

pub struct W0rld<const S: usize> {
    app: App,
    video_images: [Option<Handle<Image>>; S],
    rx: Receiver<Vec<u8>>,
}

impl<const S: usize> W0rld<S> {
    pub fn new(gltf_path: String, width: u32, height: u32) -> Result<Self> {
        let (tx, rx) = channel();
        let mut app = App::new();
        app.add_plugins(plugins::AppPlugin {
            gltf_path,
            width,
            height,
            tx,
        });
        //XXX preroll frames, returning error if signaled
        Ok(Self {
            app,
            rx,
            video_images: [const { None }; S],
        })
    }

    pub fn render(&mut self, time: f64, inframes: [&[u8]; S]) -> Result<Vec<u8>> {
        todo!()
    }
}
