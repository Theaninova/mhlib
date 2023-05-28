use godot::builtin::Color;
use godot::engine::base_material_3d::TextureParam;
use godot::engine::{load, Image, ImageTexture, StandardMaterial3D};
use godot::log::{godot_error, godot_print, godot_warn};
use godot::obj::Gd;
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::SurfaceBlockImageTextureSubChunk;
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;

pub fn collect_material(surface: SurfaceDefinition, clip: ImageClip) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new();
    material.set_name(surface.name.to_string().into());

    let mut i: Option<Gd<Image>> = None;
    for img in clip.attributes {
        match img {
            ImageClipSubChunk::StillImage(still) => {
                let path = format!(
                    "sar://{}",
                    still.name.to_string().replace('\\', "/").replace(':', ":/")
                );
                godot_print!("Loading {}", &path);
                i = Some(load(path));
            }
            x => {
                godot_warn!("Invalid clip chunk {:?}", x)
            }
        }
    }

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
                                if let Some(i) = i {
                                    texture.set_image(i);
                                } else {
                                    godot_error!("Missing texture {:?}", r);
                                }
                                i = None;
                            }
                            x => {
                                godot_warn!("Invalid image texture chunk {:?}", x)
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
            SurfaceParameterSubChunk::BaseShadingValueSpecular(base_specular) => {
                material.set_specular(base_specular.value as f64);
                if base_specular.envelope != 0 {
                    godot_error!(
                        "Not implemented: Specular envelope {}",
                        base_specular.envelope
                    );
                }
            }
            x => {
                godot_error!("Invalid Surface Chunk {:?}", x)
            }
        }
    }

    material
}
