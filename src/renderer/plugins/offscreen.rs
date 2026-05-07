// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::Sender;

use super::{PIXEL_SIZE, TEXTURE_FORMAT};
use bevy::{
    camera::RenderTarget,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_resource::{
            Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode,
            PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TextureUsages,
        },
        renderer::{RenderContext, RenderDevice, RenderGraph, RenderGraphSystems, RenderQueue},
    },
};

pub struct OffscreenPlugin {
    pub tx: Sender<Result<Vec<u8>>>,
}

impl Plugin for OffscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<OffscreenTexture>::default(),
            ExtractResourcePlugin::<RenderSender>::default(),
        ))
        .insert_resource(RenderSender(self.tx.clone()))
        .add_observer(setup_offscreen_texture);

        app.sub_app_mut(RenderApp)
            .add_systems(Render, readback_buffer.after(RenderSystems::Render))
            .add_systems(
                RenderGraph,
                copy_texture_to_buffer.after(RenderGraphSystems::Submit),
            );
    }
}

#[derive(Resource, Clone, ExtractResource)]
struct RenderSender(Sender<Result<Vec<u8>>>);

#[derive(Component)]
#[require(Camera3d)]
pub struct OffscreenSurface {
    width: u32,
    height: u32,
}

impl OffscreenSurface {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Component, ExtractComponent)]
struct OffscreenTexture {
    buffer: Buffer,
    texture: Handle<Image>,
    size: Extent3d,
    padded_bytes_per_row: usize,
}

impl OffscreenTexture {
    fn new(texture: Handle<Image>, size: Extent3d, render_device: &RenderDevice) -> Self {
        let padded_bytes_per_row =
            RenderDevice::align_copy_bytes_per_row(size.width as usize * PIXEL_SIZE);

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: padded_bytes_per_row as u64 * size.height as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            texture,
            buffer,
            size,
            padded_bytes_per_row,
        }
    }
}

fn setup_offscreen_texture(
    _add: On<Add, OffscreenSurface>,
    mut commands: Commands,
    offscreen_surface: Single<(Entity, &OffscreenSurface)>,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
) {
    let (entity, offscreen_surface) = *offscreen_surface;
    let size = Extent3d {
        width: offscreen_surface.width,
        height: offscreen_surface.height,
        ..Default::default()
    };
    let mut render_target_image =
        Image::new_target_texture(size.width, size.height, TEXTURE_FORMAT, None);
    render_target_image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    let render_target_image_handle = images.add(render_target_image);
    commands.entity(entity).insert((
        RenderTarget::Image(render_target_image_handle.clone().into()),
        OffscreenTexture::new(render_target_image_handle, size, &render_device),
    ));
}

fn copy_texture_to_buffer(
    render_context: RenderContext,
    offscreen_texture: Single<&OffscreenTexture>,
    render_queue: Res<RenderQueue>,
    gpu_images: Res<RenderAssets<bevy::render::texture::GpuImage>>,
) -> Result<()> {
    let gpu_image = gpu_images
        .get(&offscreen_texture.texture)
        .ok_or("GPU image not found")?;

    let mut encoder = render_context
        .render_device()
        .create_command_encoder(&CommandEncoderDescriptor::default());

    assert_eq!(gpu_image.texture_descriptor.format, TEXTURE_FORMAT);

    encoder.copy_texture_to_buffer(
        gpu_image.texture.as_image_copy(),
        TexelCopyBufferInfo {
            buffer: &offscreen_texture.buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(
                    std::num::NonZero::<u32>::new(offscreen_texture.padded_bytes_per_row as u32)
                        .ok_or("invalid padded bytes")?
                        .into(),
                ),
                rows_per_image: None,
            },
        },
        gpu_image.texture_descriptor.size,
    );

    render_queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

fn readback_buffer(
    offscreen_texture: Single<&OffscreenTexture>,
    render_sender: Res<RenderSender>,
    render_device: Res<RenderDevice>,
) -> Result<()> {
    let buffer_slice = offscreen_texture.buffer.slice(..);
    let buffer = offscreen_texture.buffer.clone();
    let tx = render_sender.0.clone();
    let bytes_per_row = offscreen_texture.size.width as usize * PIXEL_SIZE;
    let padded_bytes_per_row = offscreen_texture.padded_bytes_per_row;
    let height = offscreen_texture.size.height;
    buffer_slice.map_async(MapMode::Read, move |result| {
        result.expect("Failed to map buffer");
        let image_data = buffer.slice(..).get_mapped_range();
        if let Err(e) = if bytes_per_row == padded_bytes_per_row {
            tx.send(Ok(image_data.to_vec()))
        } else {
            tx.send(Ok(image_data
                .chunks(padded_bytes_per_row)
                .take(height as usize)
                .flat_map(|row| &row[..bytes_per_row.min(row.len())])
                .cloned()
                .collect()))
        } {
            warn!("Failed to send readback result: {}", e);
        }
        drop(image_data);
        buffer.unmap();
    });

    render_device.poll(PollType::wait_indefinitely())?;

    Ok(())
}
