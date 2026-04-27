// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{W0rldPlugin, PluginInfo};

pub type Mixer3Plugin = W0rldPlugin<frei0r_rs2::KindMixer3, 3>;

impl PluginInfo for frei0r_rs2::KindMixer3 {
    const NAME: &'static CStr = c"3D mixer3";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 3 input videos";
}

impl frei0r_rs2::Mixer3Plugin for Mixer3Plugin {
    fn update_mixer3(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        inframe3: &[u32],
        outframe: &mut [u32],
    ) {
        self.update(time, [inframe1, inframe2, inframe3], outframe);
    }
}
