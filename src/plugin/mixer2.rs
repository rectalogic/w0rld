// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{W0rldPlugin, PluginInfo};

pub type Mixer2Plugin = W0rldPlugin<frei0r_rs2::KindMixer2, 2>;

impl PluginInfo for frei0r_rs2::KindMixer2 {
    const NAME: &'static CStr = c"3D mixer2";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 2 input videos";
}

impl frei0r_rs2::Mixer2Plugin for Mixer2Plugin {
    fn update_mixer2(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        outframe: &mut [u32],
    ) {
        self.update(time, [inframe1, inframe2], outframe);
    }
}
