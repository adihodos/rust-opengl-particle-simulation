use super::colors::RGBAColor;
use super::utility::saturate;

pub struct StdColors {}

impl StdColors {
    pub const RED: (u8, u8, u8) = (255, 0, 0);
    pub const BLUE: (u8, u8, u8) = (0, 0, 255);
    pub const GREEN: (u8, u8, u8) = (0, 255, 0);
    pub const CYAN: (u8, u8, u8) = (0, 255, 255);
    pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
    pub const MAGENTA: (u8, u8, u8) = (255, 0, 255);
    pub const WHITE: (u8, u8, u8) = (255, 255, 255);
    pub const BLACK: (u8, u8, u8) = (0, 0, 0);
    pub const ORANGE: (u8, u8, u8) = (255, 128, 0);

    pub const DK_RED: (u8, u8, u8) = (128, 0, 0);
    pub const DK_BLUE: (u8, u8, u8) = (0, 0, 128);
    pub const DK_GREEN: (u8, u8, u8) = (0, 128, 0);
    pub const DK_CYAN: (u8, u8, u8) = (0, 128, 128);
    pub const DK_YELLOW: (u8, u8, u8) = (128, 128, 0);
    pub const DK_MAGENTA: (u8, u8, u8) = (128, 0, 128);
    pub const DK_GREY: (u8, u8, u8) = (128, 128, 128);
    pub const DK_ORANGE: (u8, u8, u8) = (128, 64, 0);

    pub const VDK_RED: (u8, u8, u8) = (64, 0, 0);
    pub const VDK_BLUE: (u8, u8, u8) = (0, 0, 64);
    pub const VDK_GREEN: (u8, u8, u8) = (0, 64, 0);
    pub const VDK_CYAN: (u8, u8, u8) = (0, 64, 64);
    pub const VDK_YELLOW: (u8, u8, u8) = (128, 64, 0);
    pub const VDK_MAGENTA: (u8, u8, u8) = (64, 0, 64);
    pub const VDK_GREY: (u8, u8, u8) = (64, 64, 64);

    pub const LT_RED: (u8, u8, u8) = (255, 128, 128);
    pub const LT_BLUE: (u8, u8, u8) = (128, 128, 255);
    pub const LT_GREEN: (u8, u8, u8) = (128, 255, 128);
    pub const LT_CYAN: (u8, u8, u8) = (128, 255, 255);
    pub const LT_YELLOW: (u8, u8, u8) = (255, 255, 128);
    pub const LT_MAGENTA: (u8, u8, u8) = (255, 128, 255);
    pub const LT_GREY: (u8, u8, u8) = (192, 192, 192);
    pub const LT_ORANGE: (u8, u8, u8) = (255, 192, 128);

    pub const VLT_GREY: (u8, u8, u8) = (224, 224, 224);
}

pub fn create_linear_colormap(start: RGBAColor, end: RGBAColor, num: u32) -> Vec<RGBAColor> {
    let r0 = start.r as f32 / 255f32;
    let g0 = start.g as f32 / 255f32;
    let b0 = start.b as f32 / 255f32;

    if num > 1 {
        let r_inc = (end.r as f32 / 255f32 - r0) as f32 / (num as f32 - 1f32);
        let g_inc = (end.g as f32 / 255f32 - g0) as f32 / (num as f32 - 1f32);
        let b_inc = (end.b as f32 / 255f32 - b0) as f32 / (num as f32 - 1f32);

        (0..num)
            .map(|i| {
                RGBAColor::new(
                    (saturate(r0 + (i as f32 * r_inc)) * 255f32) as u8,
                    (saturate(g0 + (i as f32 * g_inc)) * 255f32) as u8,
                    (saturate(b0 + (i as f32 * b_inc)) * 255f32) as u8,
                )
            })
            .collect::<Vec<_>>()
    } else {
        vec![start]
    }
}
pub struct ColorMap {}

impl ColorMap {
    // pub fn write_to_file(file: &std::path::Path, colormap: &[RGBAColor], color_blk_size: u32) {
    //     use crate::image::write_png;

    //     let mut img_pixels = vec![
    //         RGBAColor::from(StdColors::BLACK);
    //         colormap.len() * (color_blk_size * color_blk_size) as usize
    //     ];

    //     (0..color_blk_size).for_each(|y| {
    //         colormap.iter().enumerate().for_each(|(icol, rgba)| {
    //             (0..color_blk_size).for_each(|x| {
    //                 let idx = y as usize * colormap.len() * color_blk_size as usize
    //                     + (icol * color_blk_size as usize)
    //                     + x as usize;
    //                 img_pixels[idx] = *rgba;
    //             });
    //         });
    //     });

    //     let bytes = unsafe {
    //         std::slice::from_raw_parts(
    //             img_pixels.as_ptr() as *const u8,
    //             img_pixels.len() * std::mem::size_of::<RGBAColor>(),
    //         )
    //     };

    //     write_png(
    //         file,
    //         (colormap.len() * color_blk_size as usize) as u32,
    //         color_blk_size,
    //         bytes,
    //     );
    // }

    pub fn create_linear(start: RGBAColor, end: RGBAColor, num: u32) -> Vec<RGBAColor> {
        let r0 = start.r as f32 / 255f32;
        let g0 = start.g as f32 / 255f32;
        let b0 = start.b as f32 / 255f32;
        if num > 1 {
            let r_inc = (end.r as f32 / 255f32 - r0) as f32 / (num as f32 - 1f32);
            let g_inc = (end.g as f32 / 255f32 - g0) as f32 / (num as f32 - 1f32);
            let b_inc = (end.b as f32 / 255f32 - b0) as f32 / (num as f32 - 1f32);
            (0..num)
                .map(|i| {
                    RGBAColor::new(
                        (saturate(r0 + (i as f32 * r_inc)) * 255f32) as u8,
                        (saturate(g0 + (i as f32 * g_inc)) * 255f32) as u8,
                        (saturate(b0 + (i as f32 * b_inc)) * 255f32) as u8,
                    )
                })
                .collect::<Vec<_>>()
        } else {
            vec![start]
        }
    }

    /// Creates a palette : Dark Blue -> Cyan -> Green
    pub fn pf1() -> Vec<RGBAColor> {
        std::iter::once(RGBAColor::from(StdColors::BLACK))
            .chain(
                ColorMap::create_linear(
                    RGBAColor::from(StdColors::DK_BLUE),
                    RGBAColor::from(StdColors::CYAN),
                    128,
                )
                .into_iter()
                .take(127),
            )
            .chain(
                ColorMap::create_linear(
                    RGBAColor::from(StdColors::CYAN),
                    RGBAColor::from(StdColors::DK_GREEN),
                    128,
                )
                .into_iter(),
            )
            .collect()
    }

    /// Creates a palette: Dark Red -> Yellow -> Blue
    pub fn pf2() -> Vec<RGBAColor> {
        std::iter::once(RGBAColor::from(StdColors::BLACK))
            .chain(
                ColorMap::create_linear(
                    RGBAColor::from(StdColors::DK_RED),
                    RGBAColor::from(StdColors::YELLOW),
                    128,
                )
                .into_iter()
                .take(127),
            )
            .chain(
                ColorMap::create_linear(
                    RGBAColor::from(StdColors::DK_BLUE),
                    RGBAColor::from(StdColors::CYAN),
                    128,
                )
                .into_iter(),
            )
            .collect()
    }

    /// Drak blue -> cyan for the star drawing algo.
    pub fn pf3() -> Vec<RGBAColor> {
        std::iter::once(RGBAColor::from(StdColors::BLACK))
            .chain(
                ColorMap::create_linear(
                    RGBAColor::from(StdColors::CYAN),
                    RGBAColor::from(StdColors::DK_BLUE),
                    255,
                )
                .into_iter(),
            )
            .collect()
    }

    /// Increments red and green in 15 steps, used by the bands coloring scheme.
    pub fn pf4() -> Vec<RGBAColor> {
        let mut palette = vec![RGBAColor::from(StdColors::BLACK); 256];

        (0..15).for_each(|i| {
            let green = 255 - 16 * i;

            (0..15).for_each(|j| {
                let red = 255 - 12 * j;
                let idx = 16 * i + j + 17;
                palette[idx as usize] = RGBAColor::new(red as u8, green as u8, 0);
            });
        });

        palette
    }

    /// Increments blue and green in 15 steps, used by the bands coloring scheme.
    pub fn pf5() -> Vec<RGBAColor> {
        let mut palette = vec![RGBAColor::from(StdColors::BLACK); 256];

        (0..15).for_each(|i| {
            let green = 255 - 16 * i;

            (0..15).for_each(|j| {
                let blue = 255 - 12 * j;
                let idx = 16 * i + j + 17;
                palette[idx as usize] = RGBAColor::new(0, green as u8, blue as u8);
            });
        });

        palette
    }

    /// 4 colors for the quadrants scheme, 256 colors total
    pub fn pf6() -> Vec<RGBAColor> {
        let mut palette = vec![RGBAColor::from(StdColors::BLACK); 256];

        (1u32..129u32).for_each(|i| {
            palette[i as usize] = RGBAColor::new(0, 255.min(2 * i) as u8, 255.min(128 + i) as u8);
        });

        (1..128).for_each(|i| {
            palette[(128 + i) as usize] =
                RGBAColor::new((2 * i + 1) as u8, (255 - 2 * i) as u8, (255 - 2 * i) as u8);
        });

        palette
    }
    /// Yellow and blue bands, 256, colors
    pub fn pf7() -> Vec<RGBAColor> {
        let yellow_blue = [
            RGBAColor::from(StdColors::YELLOW),
            RGBAColor::from(StdColors::DK_BLUE),
        ];
        std::iter::once(RGBAColor::from(StdColors::BLACK))
            .chain((1..256).map(|i| yellow_blue[i % 2]))
            .collect()
    }

    /// Thin red bands, 256 colors
    pub fn pf8() -> Vec<RGBAColor> {
        let mut palette = vec![RGBAColor::from(StdColors::WHITE); 256];
        palette[0] = RGBAColor::from(StdColors::BLACK);

        (0..4).for_each(|i| {
            (0..3).for_each(|j| {
                palette[64 * i + j + 1] = RGBAColor::new(128, 0, 0);
                palette[64 * i + j + 4] = RGBAColor::new(192, 0, 0);
                palette[64 * i + j + 7] = RGBAColor::new(255, 0, 0);
                palette[64 * i + j + 10] = RGBAColor::new(255, 64, 64);
                palette[64 * i + j + 13] = RGBAColor::new(255, 128, 128);
                palette[64 * i + j + 16] = RGBAColor::new(255, 192, 192);
                palette[64 * i + j + 33] = RGBAColor::new(128, 64, 0);
                palette[64 * i + j + 36] = RGBAColor::new(192, 96, 0);
                palette[64 * i + j + 39] = RGBAColor::new(255, 128, 0);
                palette[64 * i + j + 42] = RGBAColor::new(255, 160, 64);
                palette[64 * i + j + 44] = RGBAColor::new(255, 192, 128);
                palette[64 * i + j + 47] = RGBAColor::new(255, 224, 192);
            });
        });

        palette
    }
}
