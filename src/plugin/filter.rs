// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;

use super::{PluginInfo, W0rldPlugin};

pub type FilterPlugin = W0rldPlugin<FilterInfo, 1>;

pub struct FilterInfo;
impl PluginInfo for FilterInfo {
    const NAME: &'static CStr = c"3D filter";
    const EXPLANATION: &'static CStr = c"Renders 3D scenes with 1 input video";
}
