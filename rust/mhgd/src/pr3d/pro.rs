use godot::builtin::{
    Array, Color, Dictionary, PackedByteArray, PackedColorArray, PackedInt32Array,
    PackedVector2Array, PackedVector3Array, ToVariant, VariantArray, Vector2, Vector3,
};
use godot::engine::base_material_3d::{CullMode, TextureParam, Transparency};
use godot::engine::image::AlphaMode;
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::node::InternalMode;
use godot::engine::{
    ArrayMesh, Image, ImageTexture, MeshInstance3D, Node3D, PackedScene, StandardMaterial3D,
};
use godot::obj::{EngineEnum, Gd, Share};
use powerrender_3d::pro::PowerRenderObject;
use std::fs::File;
use std::io::Read;

pub fn pro_to_gd(pro: PowerRenderObject) -> Gd<PackedScene> {
    let mut root = Node3D::new_alloc();
    root.set_name(pro.name.into());

    let materials: Vec<Gd<StandardMaterial3D>> = pro
        .materials
        .into_iter()
        .map(|m| {
            let mut material = StandardMaterial3D::new();
            material.set_name(m.name.into());
            let pr_tex = &pro.textures[m.texture_index];

            let mut image_file =
                File::open(format!("E:\\Games\\Moorhuhn Kart\\data\\{}", pr_tex.name)).unwrap();
            let mut image = Image::new();
            let mut buffer = vec![];
            image_file.read_to_end(&mut buffer).unwrap();
            image.load_tga_from_buffer(PackedByteArray::from(buffer.as_slice()));
            let mut texture = ImageTexture::new();
            texture.set_name(pr_tex.name.clone().into());
            texture.set_image(image);
            material.set_texture(TextureParam::TEXTURE_ALBEDO, texture.upcast());
            if pr_tex.has_alpha {
                material.set_transparency(Transparency::TRANSPARENCY_ALPHA_SCISSOR);
                material.set_alpha_scissor_threshold(0.5);
            }

            material.set_cull_mode(if m.two_sided {
                CullMode::CULL_DISABLED
            } else {
                CullMode::CULL_BACK
            });
            material
        })
        .collect();

    for segment in pro.segments {
        let mut mesh = ArrayMesh::new();
        mesh.set_name(segment.name.clone().into());

        for surface in segment.surfaces {
            let mut arrays = VariantArray::new();
            arrays.resize(ArrayType::ARRAY_MAX.ord() as usize);
            arrays.set(
                ArrayType::ARRAY_VERTEX.ord() as usize,
                PackedVector3Array::from_iter(surface.vertices.into_iter().map(|v| Vector3 {
                    x: v[0],
                    y: v[1],
                    z: v[2],
                }))
                .to_variant(),
            );
            arrays.set(
                ArrayType::ARRAY_INDEX.ord() as usize,
                PackedInt32Array::from_iter(surface.indices.into_iter()).to_variant(),
            );
            arrays.set(
                ArrayType::ARRAY_NORMAL.ord() as usize,
                PackedVector3Array::from_iter(surface.normals.into_iter().map(|n| Vector3 {
                    x: n[0],
                    y: n[1],
                    z: n[2],
                }))
                .to_variant(),
            );
            arrays.set(
                ArrayType::ARRAY_COLOR.ord() as usize,
                PackedColorArray::from_iter(surface.colors.into_iter().map(|c| Color {
                    r: c[0],
                    g: c[1],
                    b: c[2],
                    a: 1.0,
                }))
                .to_variant(),
            );
            arrays.set(
                ArrayType::ARRAY_TEX_UV.ord() as usize,
                PackedVector2Array::from_iter(
                    surface
                        .uvs
                        .into_iter()
                        .map(|uv| Vector2 { x: uv[0], y: uv[1] }),
                )
                .to_variant(),
            );

            mesh.add_surface_from_arrays(
                PrimitiveType::PRIMITIVE_TRIANGLES,
                arrays,
                Array::new(),
                Dictionary::new(),
                ArrayFormat::default(),
            );
            let surf_idx = mesh.get_surface_count() - 1;
            mesh.surface_set_material(surf_idx, materials[surface.material_index].share().upcast());
        }

        let mut instance = MeshInstance3D::new_alloc();
        instance.set_name(segment.name.into());
        instance.set_mesh(mesh.upcast());
        root.add_child(
            instance.share().upcast(),
            false,
            InternalMode::INTERNAL_MODE_DISABLED,
        );
        instance.set_owner(root.share().upcast());
    }

    let mut scene = PackedScene::new();
    scene.pack(root.share().upcast());
    root.queue_free();
    scene
}
