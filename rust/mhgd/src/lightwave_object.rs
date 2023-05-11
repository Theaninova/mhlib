use godot::bind::godot_api;
use godot::builtin::{
    Dictionary, GodotString, PackedInt32Array, PackedVector2Array, PackedVector3Array,
    VariantArray, Vector2, Vector3,
};
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::{ArrayMesh, SurfaceTool};
use godot::obj::{EngineEnum, Gd};
use godot::prelude::{godot_warn, Array, GodotClass, Share, ToVariant};
use lightwave_3d::iff::Chunk;
use lightwave_3d::lwo2::tags::point_list::PointList;
use lightwave_3d::lwo2::tags::polygon_list::PolygonLists;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;

#[derive(GodotClass)]
#[class(init)]
struct Lwo {}

#[godot_api]
impl Lwo {
    #[func]
    pub fn get_mesh(path: GodotString) -> Gd<ArrayMesh> {
        lightwave_to_gd(LightWaveObject::read_file(path.to_string()).unwrap())
    }
}

pub fn lightwave_to_gd(lightwave: LightWaveObject) -> Gd<ArrayMesh> {
    let mut mesh = ArrayMesh::new();
    let mut arrays: Option<VariantArray> = None;

    let mut vert_count = 0;

    for tag in lightwave.data {
        match tag {
            Tag::PointList(points) => {
                try_commit(&mut mesh, &arrays);
                vert_count = points.point_location.len();

                let mut ars = Array::new();
                ars.resize(ArrayType::ARRAY_MAX.ord() as usize);

                ars.set(
                    ArrayType::ARRAY_VERTEX.ord() as usize,
                    collect_points(points).to_variant(),
                );
                arrays = Some(ars);
            }
            Tag::VertexMapping(vmap) => match &vmap.kind {
                b"TXUV" => {
                    if let Some(arrays) = &mut arrays {
                        let mut arr = PackedVector2Array::new();
                        arr.resize(vert_count);

                        for uv in vmap.data.mapping {
                            arr.set(
                                uv.vert as usize,
                                Vector2 {
                                    x: uv.value[0],
                                    y: uv.value[1],
                                },
                            );
                        }

                        arrays.set(ArrayType::ARRAY_TEX_UV.ord() as usize, arr.to_variant());
                    }
                }
                x => godot_warn!("{}", String::from_utf8(x.to_vec()).unwrap()),
            },
            Tag::PolygonList(polygons) => match &polygons.kind {
                b"FACE" => {
                    if let Some(arrays) = &mut arrays {
                        let indices = collect_polygons(polygons);
                        arrays.set(ArrayType::ARRAY_INDEX.ord() as usize, indices.to_variant());
                    }
                }
                x => godot_warn!("{}", String::from_utf8(x.to_vec()).unwrap()),
            },
            _ => (),
        }
    }

    try_commit(&mut mesh, &arrays);
    let mut out_mesh = ArrayMesh::new();
    for i in 0..mesh.get_surface_count() {
        let mut tool = SurfaceTool::new();
        tool.create_from(mesh.share().upcast(), i);
        tool.generate_normals(false);
        tool.generate_tangents();
        try_commit(&mut out_mesh, &Some(tool.commit_to_arrays()));
    }
    out_mesh
}

fn try_commit(mesh: &mut ArrayMesh, arrays: &Option<VariantArray>) {
    if let Some(arrays) = arrays {
        mesh.add_surface_from_arrays(
            PrimitiveType::PRIMITIVE_TRIANGLES,
            arrays.share(),
            Array::new(),
            Dictionary::new(),
            ArrayFormat::ARRAY_FORMAT_NORMAL,
        );
    }
}

fn collect_points(chunk: Chunk<PointList>) -> PackedVector3Array {
    PackedVector3Array::from(
        chunk
            .data
            .point_location
            .into_iter()
            .map(|[x, y, z]| Vector3 { x, y, z })
            .collect::<Vec<Vector3>>()
            .as_slice(),
    )
}

fn collect_polygons(chunk: Chunk<PolygonLists>) -> PackedInt32Array {
    debug_assert!(chunk.polygons.len() >= 3, "{:?}", chunk);
    PackedInt32Array::from(
        chunk
            .data
            .polygons
            .into_iter()
            .flat_map(|mut it| {
                let fan_v = it.vert.remove(0) as i32;
                it.vert
                    .windows(2)
                    .flat_map(|w| [w[1] as i32, w[0] as i32, fan_v])
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<i32>>()
            .as_slice(),
    )
}
