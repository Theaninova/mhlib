use godot::builtin::{
    Basis, Color, EulerOrder, Quaternion, ToVariant, Transform3D, Variant, Vector3,
};
use godot::engine::{load, Image, ImageTexture, ShaderMaterial};
use godot::log::{godot_error, godot_print};
use godot::obj::{Gd, Share};
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::{
    ProjectionMode, SurfaceBlockImageTextureSubChunk,
};
use lightwave_3d::lwo2::sub_tags::blocks::texture_mapping::{
    CoordinateSystem, FalloffType, TextureMappingSubChunk,
};
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;
use std::collections::HashMap;

pub struct MaterialUvInfo {
    pub diffuse_channel: Option<String>,
    pub color_channel: Option<String>,
    pub material: Gd<ShaderMaterial>,
}

impl MaterialUvInfo {
    pub fn collect(surface: SurfaceDefinition, images: &HashMap<u32, Gd<Image>>) -> Self {
        let mut m = MaterialUvInfo {
            diffuse_channel: None,
            color_channel: None,
            material: ShaderMaterial::new(),
        };
        m.material.set_name(surface.name.to_string().into());
        m.material
            .set_shader(load("res://starforce/starforce.gdshader"));

        for attr in surface.attributes {
            match attr {
                SurfaceParameterSubChunk::Blocks(blocks) => {
                    if let SurfaceBlocks::ImageMapTexture { header, attributes } = blocks.data {
                        let mut texture = ImageTexture::new();
                        let mut chan = TextureChannel::Color;
                        let mut uv_channel = None;
                        let mut projection_mode = ProjectionMode::UV;
                        let mut mapping_info = Vec::<(&str, Variant)>::new();
                        for attr in header.data.block_attributes {
                            match attr {
                                SurfaceBlockHeaderSubChunk::Channel(c) => {
                                    chan = c.data.texture_channel;
                                }
                                SurfaceBlockHeaderSubChunk::Opacity(_) => {
                                    // TODO;
                                }
                                SurfaceBlockHeaderSubChunk::EnabledState(_) => {
                                    // TODO;
                                }
                                SurfaceBlockHeaderSubChunk::Negative(_) => {
                                    // TODO;
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
                                    projection_mode = projection.data;
                                }
                                SurfaceBlockImageTextureSubChunk::UvVertexMap(map) => {
                                    uv_channel = Some(map.txuv_map_name.to_string());
                                }
                                SurfaceBlockImageTextureSubChunk::TextureMapping(mapping) => {
                                    let mut pos = Vector3::default();
                                    let mut rot = Vector3::default();
                                    let mut size = Vector3::default();
                                    for mapping_param in mapping.data.attributes {
                                        match mapping_param {
                                            TextureMappingSubChunk::Center(it) => {
                                                pos = Vector3 {
                                                    x: it.base_color[0],
                                                    y: it.base_color[1],
                                                    z: it.base_color[2],
                                                };
                                            }
                                            TextureMappingSubChunk::Rotation(it) => {
                                                rot = Vector3 {
                                                    x: it.base_color[0],
                                                    y: it.base_color[1],
                                                    z: it.base_color[2],
                                                }
                                                .normalized();
                                            }
                                            TextureMappingSubChunk::Size(it) => {
                                                size = Vector3 {
                                                    x: it.base_color[0],
                                                    y: it.base_color[1],
                                                    z: it.base_color[2],
                                                };
                                            }
                                            TextureMappingSubChunk::Falloff(it) => {
                                                mapping_info.push((
                                                    "falloff",
                                                    Vector3 {
                                                        x: it.vector[0],
                                                        y: it.vector[1],
                                                        z: it.vector[2],
                                                    }
                                                    .to_variant(),
                                                ));
                                                mapping_info.push((
                                                    "falloff_type",
                                                    match it.kind {
                                                        FalloffType::Cubic => 0,
                                                        FalloffType::Spherical => 1,
                                                        FalloffType::LinearX => 2,
                                                        FalloffType::LinearY => 3,
                                                        FalloffType::LinearZ => 4,
                                                    }
                                                    .to_variant(),
                                                ));
                                            }
                                            TextureMappingSubChunk::CoordinateSystem(it) => {
                                                mapping_info.push((
                                                    "world_coords",
                                                    matches!(
                                                        it.data,
                                                        CoordinateSystem::WorldCoordinates
                                                    )
                                                    .to_variant(),
                                                ));
                                            }
                                            TextureMappingSubChunk::ReferenceObject(it) => {
                                                if !matches!(
                                                    it.object_name.to_string().as_str(),
                                                    "" | "(none)"
                                                ) {
                                                    godot_error!("Reference object '{}': not supported for texture mapping", it.object_name)
                                                }
                                            }
                                        }
                                    }

                                    mapping_info.push((
                                        "transform",
                                        Transform3D {
                                            basis: Basis::from_euler(EulerOrder::XYZ, rot)
                                                .scaled(size),
                                            origin: pos,
                                        }
                                        .to_variant(),
                                    ));
                                }
                                SurfaceBlockImageTextureSubChunk::MajorAxis(_) => {
                                    // TODO;
                                }
                                SurfaceBlockImageTextureSubChunk::ImageWrapOptions(_) => {
                                    // TODO;
                                }
                                SurfaceBlockImageTextureSubChunk::ImageWrapAmountWidth(_) => {
                                    // TODO;
                                }
                                SurfaceBlockImageTextureSubChunk::ImageWrapAmountHeight(_) => {
                                    // TODO;
                                }
                                SurfaceBlockImageTextureSubChunk::AntialiasingStrength(_) => {
                                    // TODO;
                                }
                                SurfaceBlockImageTextureSubChunk::PixelBlending(_) => {
                                    // TODO;
                                }
                                x => {
                                    godot_error!("TODO: Image texture chunk {:?}", x)
                                }
                            }
                        }
                        godot_print!("TX: {:?} @ UV{:?}", chan, uv_channel);
                        let channel_name = match chan {
                            TextureChannel::Color => "color",
                            TextureChannel::Diffuse => "diffuse",
                            TextureChannel::Luminosity => "luminosity",
                            TextureChannel::Specular => "specular",
                            TextureChannel::Glossy => "glossy",
                            TextureChannel::Reflectivity => "reflectivity",
                            TextureChannel::Transparency => "transparency",
                            TextureChannel::RefractiveIndex => "refractive_index",
                            TextureChannel::Translucency => "translucency",
                            TextureChannel::Bump => "bump",
                        };
                        m.material.set_shader_parameter(
                            format!("tex_{}_projection", channel_name).into(),
                            match projection_mode {
                                ProjectionMode::Planar => 0,
                                ProjectionMode::Cylindrical => 1,
                                ProjectionMode::Spherical => 2,
                                ProjectionMode::Cubic => 3,
                                ProjectionMode::FrontProjection => 4,
                                ProjectionMode::UV => 5,
                            }
                            .to_variant(),
                        );
                        m.material.set_shader_parameter(
                            format!("tex_{}", channel_name).into(),
                            texture.to_variant(),
                        );

                        for (name, value) in mapping_info {
                            m.material.set_shader_parameter(
                                format!("tex_{}_projection_{}", channel_name, name).into(),
                                value,
                            );
                        }

                        match chan {
                            TextureChannel::Diffuse => m.diffuse_channel = uv_channel,
                            TextureChannel::Color => m.color_channel = uv_channel,
                            _ => (),
                        }
                    }
                }
                SurfaceParameterSubChunk::BaseColor(base_color) => m.material.set_shader_parameter(
                    "color".into(),
                    Color {
                        r: base_color.base_color[0],
                        g: base_color.base_color[1],
                        b: base_color.base_color[2],
                        a: 1.0,
                    }
                    .to_variant(),
                ),
                SurfaceParameterSubChunk::BaseShadingValueDiffuse(it) => {
                    m.material
                        .set_shader_parameter("diffuse".into(), it.value.to_variant());
                    m.material
                        .set_shader_parameter("diffuse_envelope".into(), it.envelope.to_variant());
                }
                SurfaceParameterSubChunk::BaseShadingValueSpecular(it) => {
                    m.material
                        .set_shader_parameter("specular".into(), it.value.to_variant());
                    m.material
                        .set_shader_parameter("specular_envelope".into(), it.envelope.to_variant());
                }
                SurfaceParameterSubChunk::BaseShadingValueLuminosity(it) => {
                    m.material
                        .set_shader_parameter("luminosity".into(), it.value.to_variant());
                    m.material.set_shader_parameter(
                        "luminosity_envelope".into(),
                        it.envelope.to_variant(),
                    );
                }
                SurfaceParameterSubChunk::BaseShadingValueReflectivity(it) => {
                    m.material
                        .set_shader_parameter("reflectivity".into(), it.value.to_variant());
                    m.material.set_shader_parameter(
                        "reflectivity_envelope".into(),
                        it.envelope.to_variant(),
                    );
                }
                SurfaceParameterSubChunk::BaseShadingValueTranslucency(it) => {
                    m.material
                        .set_shader_parameter("translucency".into(), it.value.to_variant());
                    m.material.set_shader_parameter(
                        "translucency_envelope".into(),
                        it.envelope.to_variant(),
                    );
                }
                SurfaceParameterSubChunk::BaseShadingValueTransparency(it) => {
                    m.material
                        .set_shader_parameter("transparency".into(), it.value.to_variant());
                    m.material.set_shader_parameter(
                        "transparency_envelope".into(),
                        it.envelope.to_variant(),
                    );
                }
                x => {
                    godot_error!("TODO: Surface Chunk {:?}", x)
                }
            }
        }

        m
    }
}
