use crate::iff::Chunk;
use crate::lwo2::tags::bounding_box::BoundingBox;
use crate::lwo2::tags::discontinuous_vertex_mapping::DiscontinuousVertexMappings;
use crate::lwo2::tags::image_clip::ImageClip;
use crate::lwo2::tags::layer::Layer;
use crate::lwo2::tags::meta::{DescriptionLine, ThumbnailIconImage};
use crate::lwo2::tags::point_list::PointList;
use crate::lwo2::tags::polygon_list::PolygonLists;
use crate::lwo2::tags::polygon_tag_mapping::PolygonTagMappings;
use crate::lwo2::tags::surface_definition::SurfaceDefinition;
use crate::lwo2::tags::tag_strings::TagStrings;
use crate::lwo2::tags::vertex_map_parameter::VertexMapParameter;
use crate::lwo2::tags::vertex_mapping::VertexMappings;
use binrw::binread;

pub mod bounding_box;
pub mod discontinuous_vertex_mapping;
pub mod envelope;
pub mod image_clip;
pub mod layer;
pub mod meta;
pub mod point_list;
pub mod polygon_list;
pub mod polygon_tag_mapping;
pub mod surface_definition;
pub mod tag_strings;
pub mod vertex_map_parameter;
pub mod vertex_mapping;

#[binread]
#[derive(Debug)]
pub enum Tag {
    #[br(magic(b"LAYR"))]
    Layer(Chunk<Layer>),
    #[br(magic(b"PNTS"))]
    PointList(Chunk<PointList>),
    #[br(magic(b"VMAP"))]
    VertexMapping(Chunk<VertexMappings>),
    #[br(magic(b"TAGS"))]
    TagStrings(Chunk<TagStrings>),
    #[br(magic(b"PTAG"))]
    PolygonTagMapping(Chunk<PolygonTagMappings>),
    #[br(magic(b"VMAD"))]
    DiscontinuousVertexMapping(Chunk<DiscontinuousVertexMappings>),
    #[br(magic(b"VMPA"))]
    VertexMapParameter(Chunk<VertexMapParameter>),
    #[br(magic(b"BBOX"))]
    BoundingBox(Chunk<BoundingBox>),
    #[br(magic(b"DESC"))]
    DescriptionLine(Chunk<DescriptionLine>),
    #[br(magic(b"TEXT"))]
    CommentaryText(Chunk<DescriptionLine>),
    #[br(magic(b"ICON"))]
    ThumbnailIconImage(Chunk<ThumbnailIconImage>),
    #[br(magic(b"POLS"))]
    PolygonList(Chunk<PolygonLists>),
    #[br(magic(b"SURF"))]
    SurfaceDefinition(Chunk<SurfaceDefinition>),
    #[br(magic(b"CLIP"))]
    ImageClip(Chunk<ImageClip>),
}
