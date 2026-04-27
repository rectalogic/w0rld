// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::{Receiver, Sender};

use crate::processor::Processor;
use bevy::ecs::error::Result;
use frei0r_rs2::slice_to_bytes_mut;

mod plugins;
mod w0rld;

struct RenderJob<const S: usize> {
    time: f64,
    inputs: [(*const u8, usize); S],
}

// SAFETY: The caller guarantees input references remain valid until channel signals completion
unsafe impl<const S: usize> Send for RenderJob<S> {}

impl<const S: usize> RenderJob<S> {
    fn new(time: f64, inputs: [&[u32]; S]) -> Self {
        let inputs: [(*const u8, usize); S] = inputs
            .into_iter()
            .map(|input| (input.as_ptr().cast::<u8>(), size_of_val(input)))
            .collect::<Vec<(*const u8, usize)>>()
            .try_into()
            .unwrap();
        Self { time, inputs }
    }
}

pub struct RenderResult {
    output: Result<Vec<u8>>,
}

pub struct RenderProcessor<const S: usize> {
    processor: Processor<RenderJob<S>, RenderResult>,
}

impl<const S: usize> RenderProcessor<S> {
    pub fn new(gltf_path: String, width: u32, height: u32) -> Self {
        let processor = Processor::new(
            move |rx: Receiver<RenderJob<S>>, tx: Sender<RenderResult>| {
                let mut renderer = w0rld::W0rld::new(gltf_path, width, height)?;
                for job in rx {
                    let inputs: [&[u8]; S] = job
                        .inputs
                        .into_iter()
                        .map(|(input_ptr, input_len)| unsafe {
                            std::slice::from_raw_parts(input_ptr, input_len)
                        })
                        .collect::<Vec<&[u8]>>()
                        .try_into()
                        .unwrap();
                    let output = renderer.render(job.time, inputs);
                    tx.send(RenderResult { output })?;
                }
                Ok(())
            },
        );
        Self { processor }
    }

    pub fn render(&mut self, time: f64, inframes: [&[u32]; S], output: &mut [u32]) -> Result<()> {
        let job = RenderJob::new(time, inframes);
        let response = self.processor.process(job)?;
        slice_to_bytes_mut(output).copy_from_slice(response.output?.as_slice());
        Ok(())
    }
}
