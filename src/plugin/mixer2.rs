// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{PluginInfo, W0rldPlugin};

pub type Mixer2Plugin = W0rldPlugin<Mixer2Info, 2>;

pub struct Mixer2Info;
impl PluginInfo for Mixer2Info {
    const NAME: &'static CStr = c"3D mixer2";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 2 input videos";
}
