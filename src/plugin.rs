// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::log;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

use crate::renderer::RenderProcessor;

pub mod filter;
pub mod mixer2;
pub mod mixer3;
pub mod source;

pub struct W0rldPlugin<K: frei0r_rs2::PluginKind, const S: usize> {
    gltf_path: CString,
    width: u32,
    height: u32,
    processor: Option<Result<RenderProcessor<S>, ()>>,
    _phantom: PhantomData<K>,
}

impl<K, const S: usize> W0rldPlugin<K, S>
where
    K: frei0r_rs2::PluginKind,
{
    fn new(width: u32, height: u32) -> Self {
        Self {
            gltf_path: c"".to_owned(),
            width,
            height,
            processor: None,
            _phantom: PhantomData,
        }
    }

    fn update(&mut self, time: f64, inframes: [&[u32]; S], outframe: &mut [u32]) {
        let processor = match self.processor {
            Some(Ok(ref mut processor)) => processor,
            Some(Err(())) => return,
            None => match self.gltf_path.to_str() {
                Ok(gltf_path) => {
                    self.processor = Some(Ok(RenderProcessor::<S>::new(
                        gltf_path.into(),
                        self.width,
                        self.height,
                    )));
                    self.processor.as_mut().unwrap().as_mut().unwrap()
                }
                Err(_) => {
                    log::error!("w0rld: invalid gltf_path `{:?}'", self.gltf_path);
                    self.processor = Some(Err(()));
                    return;
                }
            },
        };

        if let Err(e) = processor.render(time, inframes, outframe) {
            log::error!("w0rld: failed to render frame: {e:?}");
            self.processor = Some(Err(()));
        }
    }
}

trait PluginInfo {
    const NAME: &'static CStr;
    const EXPLANATION: &'static CStr;
}

impl<K, const S: usize> frei0r_rs2::Plugin for W0rldPlugin<K, S>
where
    K: frei0r_rs2::PluginKind + PluginInfo + Send + 'static,
{
    type Kind = K;

    const PARAMS: &'static [frei0r_rs2::ParamInfo<Self>] = &[frei0r_rs2::ParamInfo::new_string(
        c"gltf_path",
        c"Path to GLTF/GLB scene file",
        |plugin| plugin.gltf_path.as_c_str(),
        |plugin, value| value.clone_into(&mut plugin.gltf_path),
    )];

    fn info() -> frei0r_rs2::PluginInfo {
        frei0r_rs2::PluginInfo {
            name: K::NAME,
            author: c"Andrew Wason",
            color_model: frei0r_rs2::ColorModel::RGBA8888,
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            explanation: Some(K::EXPLANATION),
        }
    }

    fn new(width: usize, height: usize) -> Self {
        W0rldPlugin::new(width as u32, height as u32)
    }
}
