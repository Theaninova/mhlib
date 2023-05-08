use crate::media::rle::{bgra_to_rgba, RleImage};
use image::error::{LimitError, LimitErrorKind};
use image::{AnimationDecoder, Delay, Frame, Frames, ImageBuffer, ImageError};
use std::time::Duration;

impl<'a> AnimationDecoder<'a> for RleImage {
    fn into_frames(self) -> Frames<'a> {
        Frames::new(Box::new(self.frames.into_iter().map(move |frame| {
            let buffer = ImageBuffer::from_raw(
                frame.width,
                frame.height,
                frame
                    .data
                    .into_iter()
                    .flat_map(|it| bgra_to_rgba(self.color_table[it as usize]))
                    .collect(),
            )
            .ok_or(ImageError::Limits(LimitError::from_kind(
                LimitErrorKind::InsufficientMemory,
            )))?;
            Ok(Frame::from_parts(
                buffer,
                frame.left,
                frame.top,
                Delay::from_saturating_duration(Duration::from_millis(80)),
            ))
        })))
    }
}
