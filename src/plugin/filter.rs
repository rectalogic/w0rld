// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{W0rldPlugin, PluginInfo};

pub type FilterPlugin = W0rldPlugin<frei0r_rs2::KindFilter, 1>;

impl PluginInfo for frei0r_rs2::KindFilter {
    const NAME: &'static CStr = c"3D filter";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 1 input video";
}

impl frei0r_rs2::FilterPlugin for FilterPlugin {
    fn update_filter(&mut self, time: f64, inframe: &[u32], outframe: &mut [u32]) {
        self.update(time, [inframe], outframe);
    }
}
