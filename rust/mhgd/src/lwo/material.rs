use godot::builtin::{Basis, Color, EulerOrder, ToVariant, Transform3D, Vector3};
use godot::engine::{load, PlaceholderTexture2D, ShaderMaterial, Texture2D};
use godot::log::{godot_error, godot_print};
use godot::obj::{Gd, Share};
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::{
    ProjectionMode, SurfaceBlockImageTextureSubChunk,
};
use lightwave_3d::lwo2::sub_tags::blocks::texture_mapping::{FalloffType, TextureMappingSubChunk};
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;
use std::collections::HashMap;

#[derive(Debug)]
pub enum MaterialProjectionMode {
    UvChannelName(String),
    Planar { transform: Transform3D, axis: u16 },
    Cubic { transform: Transform3D },
}

#[derive(Debug)]
pub struct MaterialUvInfo {
    pub diffuse_projection: Option<MaterialProjectionMode>,
    pub color_projection: Option<MaterialProjectionMode>,
    pub max_smoothing_angle: Option<f32>,
    pub material: Gd<ShaderMaterial>,
    pub id: u16,
}

impl MaterialUvInfo {
    pub fn collect(
        surface: SurfaceDefinition,
        textures: &HashMap<u32, Gd<Texture2D>>,
        id: u16,
    ) -> Self {
        let mut m = MaterialUvInfo {
            diffuse_projection: None,
            color_projection: None,
            max_smoothing_angle: None,
            material: ShaderMaterial::new(),
            id,
        };
        m.material.set_name(surface.name.into());
        m.material
            .set_shader(load("res://starforce/starforce.gdshader"));

        for attr in surface.attributes {
            match attr {
                SurfaceParameterSubChunk::Blocks(blocks) => {
                    if let SurfaceBlocks::ImageMapTexture { header, attributes } = blocks.data {
                        let mut texture: Gd<Texture2D> = PlaceholderTexture2D::new().upcast();
                        let mut chan = TextureChannel::Color;
                        let mut uv_channel = None;
                        let mut major_axis = 0;
                        let mut transform: Transform3D = Transform3D::IDENTITY;
                        let mut projection_mode = ProjectionMode::UV;
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
                                    if let Some(tex) = textures.get(&r.texture_image) {
                                        godot_print!("{}", tex.get_name());
                                        debug_assert!(texture
                                            .try_cast::<PlaceholderTexture2D>()
                                            .is_some());
                                        texture = tex.share()
                                    } else {
                                        godot_error!("Missing texture {:?}", r);
                                    }
                                }
                                SurfaceBlockImageTextureSubChunk::ProjectionMode(projection) => {
                                    projection_mode = projection.data;
                                }
                                SurfaceBlockImageTextureSubChunk::UvVertexMap(map) => {
                                    uv_channel = Some(map.txuv_map_name.clone());
                                }
                                SurfaceBlockImageTextureSubChunk::TextureMapping(mapping) => {
                                    let mut pos = Vector3::default();
                                    let mut rot = Vector3::default();
                                    let mut size = Vector3::default();
                                    for mapping_param in mapping.data.attributes {
                                        match mapping_param {
                                            TextureMappingSubChunk::Center(it) => {
                                                pos = Vector3 {
                                                    z: it.base_color[0],
                                                    y: it.base_color[1],
                                                    x: it.base_color[2],
                                                };
                                            }
                                            TextureMappingSubChunk::Rotation(it) => {
                                                rot = Vector3 {
                                                    z: it.base_color[0],
                                                    y: it.base_color[1],
                                                    x: it.base_color[2],
                                                }
                                                .normalized();
                                            }
                                            TextureMappingSubChunk::Size(it) => {
                                                size = Vector3 {
                                                    z: it.base_color[0],
                                                    y: it.base_color[1],
                                                    x: it.base_color[2],
                                                };
                                            }
                                            TextureMappingSubChunk::Falloff(it) => {
                                                /* TODO
                                                mapping_info.push((
                                                    "falloff",
                                                    Vector3 {
                                                        z: it.vector[0],
                                                        y: it.vector[1],
                                                        x: it.vector[2],
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
                                                ));*/
                                            }
                                            TextureMappingSubChunk::CoordinateSystem(it) => {
                                                /* TODO
                                                mapping_info.push((
                                                    "world_coords",
                                                    matches!(
                                                        it.data,
                                                        CoordinateSystem::WorldCoordinates
                                                    )
                                                    .to_variant(),
                                                ));*/
                                            }
                                            TextureMappingSubChunk::ReferenceObject(it) => {
                                                if !matches!(it.object_name.as_str(), "" | "(none)")
                                                {
                                                    godot_error!("Reference object '{}': not supported for texture mapping", it.object_name)
                                                }
                                            }
                                        }
                                    }

                                    transform = Transform3D {
                                        basis: Basis::from_euler(EulerOrder::ZYX, rot).scaled(size),
                                        origin: pos,
                                    }
                                    .affine_inverse()
                                }
                                SurfaceBlockImageTextureSubChunk::MajorAxis(axis) => {
                                    major_axis = axis.data.texture_axis;
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

                        let channel_name = match &chan {
                            TextureChannel::Color => "color",
                            // this is a bit confusing, but this is actually diffuse *lighting*
                            // aka baked lightmaps
                            TextureChannel::Diffuse => "diffuse",
                            TextureChannel::Bump => "bump",
                            TextureChannel::Specular => "specular",
                            TextureChannel::Glossy => "glossy",
                            TextureChannel::Reflectivity => "reflectivity",
                            TextureChannel::Transparency => "transparency",
                            TextureChannel::RefractiveIndex => "refractive_index",
                            TextureChannel::Translucency => "translucency",
                            TextureChannel::Luminosity => "luminosity",
                        };
                        m.material.set_shader_parameter(
                            format!("tex_{}", channel_name).into(),
                            texture.to_variant(),
                        );

                        let projection_info = match projection_mode {
                            ProjectionMode::UV => {
                                MaterialProjectionMode::UvChannelName(uv_channel.unwrap())
                            }
                            ProjectionMode::Cubic => MaterialProjectionMode::Cubic { transform },
                            ProjectionMode::Planar => MaterialProjectionMode::Planar {
                                transform,
                                axis: major_axis,
                            },
                            x => {
                                godot_error!("TODO: {:?}", x);
                                MaterialProjectionMode::UvChannelName("[[unsupported]]".into())
                            }
                        };

                        match chan {
                            TextureChannel::Diffuse => m.diffuse_projection = Some(projection_info),
                            TextureChannel::Color => m.color_projection = Some(projection_info),
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
                SurfaceParameterSubChunk::MaxSmoothingAngle(it) => {
                    m.max_smoothing_angle = Some(it.max_smoothing_angle);
                }
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
