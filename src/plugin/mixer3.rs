// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{PluginInfo, W0rldPlugin};

pub type Mixer3Plugin = W0rldPlugin<Mixer3Info, 3>;

pub struct Mixer3Info;
impl PluginInfo for Mixer3Info {
    const NAME: &'static CStr = c"3D mixer3";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 3 input videos";
}
