// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{W0rldPlugin, PluginInfo};

pub type SourcePlugin = W0rldPlugin<frei0r_rs2::KindSource, 0>;

impl PluginInfo for frei0r_rs2::KindSource {
    const NAME: &'static CStr = c"3D mixer3";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 3 input videos";
}

impl frei0r_rs2::SourcePlugin for SourcePlugin {
    fn update_source(&mut self, time: f64, outframe: &mut [u32]) {
        self.update(time, [], outframe);
    }
}
