use godot::log::godot_error;
use lightwave_3d::iff::Chunk;
use lightwave_3d::lwo2::tags::discontinuous_vertex_mapping::DiscontinuousVertexMappings;
use lightwave_3d::lwo2::tags::vertex_mapping::VertexMappings;
use std::collections::HashMap;

pub fn find_mapping<T: Default + Copy + std::fmt::Debug>(
    target: &HashMap<i32, HashMap<i32, T>>,
    poly: usize,
    vert: i32,
) -> Option<T> {
    target
        .get(&(poly as i32))
        .and_then(|mapping| mapping.get(&vert).copied())
        .or_else(|| {
            target
                .get(&-1)
                .and_then(|mapping| mapping.get(&vert).copied())
        })
}

pub fn collect_discontinuous_mappings<T>(
    target: &mut HashMap<i32, HashMap<i32, T>>,
    vmap: Chunk<DiscontinuousVertexMappings>,
    map_fn: fn(Vec<f32>) -> T,
) {
    for mapping in vmap.data.mappings {
        target
            .entry(mapping.poly as i32)
            .or_insert_with(|| HashMap::new())
            .insert(mapping.vert as i32, map_fn(mapping.values));
    }
}

pub fn collect_mappings<T>(
    target: &mut HashMap<i32, HashMap<i32, T>>,
    vmap: Chunk<VertexMappings>,
    map_fn: fn(Vec<f32>) -> T,
) {
    let entry = target.entry(-1).or_insert_with(|| HashMap::new());
    for mapping in vmap.data.mapping {
        entry.insert(mapping.vert as i32, map_fn(mapping.value));
    }
}
