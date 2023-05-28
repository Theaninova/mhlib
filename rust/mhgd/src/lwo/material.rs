use godot::builtin::Color;
use godot::engine::base_material_3d::{DiffuseMode, TextureParam};
use godot::engine::{Image, ImageTexture, StandardMaterial3D};
use godot::log::{godot_error, godot_warn};
use godot::obj::{Gd, Share};
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::{
    ProjectionMode, SurfaceBlockImageTextureSubChunk,
};
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;
use std::collections::HashMap;

pub fn collect_material(
    surface: SurfaceDefinition,
    images: &HashMap<u32, Gd<Image>>,
) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new();
    material.set_name(surface.name.to_string().into());
    material.set_diffuse_mode(DiffuseMode::DIFFUSE_TOON);

    for attr in surface.attributes {
        match attr {
            SurfaceParameterSubChunk::Blocks(blocks) => {
                if let SurfaceBlocks::ImageMapTexture { header, attributes } = blocks.data {
                    let mut texture = ImageTexture::new();
                    let mut chan = TextureParam::TEXTURE_ALBEDO;
                    for attr in header.data.block_attributes {
                        match attr {
                            SurfaceBlockHeaderSubChunk::Channel(c) => {
                                chan = match c.data.texture_channel {
                                    TextureChannel::Color => TextureParam::TEXTURE_ALBEDO,
                                    TextureChannel::Diffuse => TextureParam::TEXTURE_ALBEDO,
                                    TextureChannel::Bump => TextureParam::TEXTURE_HEIGHTMAP,
                                    TextureChannel::RefractiveIndex => {
                                        TextureParam::TEXTURE_REFRACTION
                                    }
                                    TextureChannel::Specular => TextureParam::TEXTURE_METALLIC,
                                    TextureChannel::Glossy => TextureParam::TEXTURE_ROUGHNESS,
                                    x => {
                                        godot_error!("Invalid channel {:?}", x);
                                        TextureParam::TEXTURE_ORM
                                    }
                                }
                            }
                            x => {
                                godot_error!("Invalid surface header chunk {:?}", x)
                            }
                        }
                    }
                    for attr in attributes {
                        match attr {
                            SurfaceBlockImageTextureSubChunk::ImageMap(r) => {
                                if let Some(i) = images.get(&r.texture_image) {
                                    texture.set_image(i.share());
                                } else {
                                    godot_error!("Missing texture {:?}", r);
                                }
                            }
                            SurfaceBlockImageTextureSubChunk::ProjectionMode(projection) => {
                                match projection.data {
                                    ProjectionMode::UV => {}
                                    _ => {
                                        godot_error!("TODO: Projection mode {:?}", projection)
                                    }
                                }
                            }
                            SurfaceBlockImageTextureSubChunk::UvVertexMap(map) => {
                                godot_error!(
                                    "TODO: UV maps (this one is using '{}')",
                                    map.txuv_map_name
                                )
                            }
                            x => {
                                godot_error!("TODO: Image texture chunk {:?}", x)
                            }
                        }
                    }
                    material.set_texture(chan, texture.upcast());
                }
            }
            SurfaceParameterSubChunk::BaseColor(base_color) => material.set_albedo(Color {
                r: base_color.base_color[0],
                g: base_color.base_color[1],
                b: base_color.base_color[2],
                a: 1.0,
            }),
            SurfaceParameterSubChunk::BaseShadingValueDiffuse(base_diffuse) => {
                if base_diffuse.envelope != 0 || base_diffuse.value != 1.0 {
                    godot_error!(
                        "TODO: Diffuse {{envelope: {}, value: {}}}",
                        base_diffuse.envelope,
                        base_diffuse.value
                    );
                }
            }
            SurfaceParameterSubChunk::BaseShadingValueSpecular(base_specular) => {
                material.set_specular(base_specular.value as f64);
                if base_specular.envelope != 0 {
                    godot_error!("TODO: Specular envelope {}", base_specular.envelope);
                }
            }
            x => {
                godot_error!("TODO: Surface Chunk {:?}", x)
            }
        }
    }

    material
}
