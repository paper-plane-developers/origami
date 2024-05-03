// Based on Mikhail Filimonov's code from
// https://github.com/overtake/TelegramSwift/blob/master/packages/TGUIKit/Sources/SoftwareGradientBackground.swift

pub type Color = gtk::graphene::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[doc(alias = "interpolatePoints")]
    pub fn interpolate(self, other: Self, factor: f32) -> Self {
        Self {
            x: self.x * (1.0 - factor) + other.x * factor,
            y: self.y * (1.0 - factor) + other.y * factor,
        }
    }
}

static BASE_POSITIONS: &[Point] = &[
    Point { x: 0.80, y: 0.10 },
    Point { x: 0.60, y: 0.20 },
    Point { x: 0.35, y: 0.25 },
    Point { x: 0.25, y: 0.60 },
    Point { x: 0.20, y: 0.90 },
    Point { x: 0.40, y: 0.80 },
    Point { x: 0.65, y: 0.75 },
    Point { x: 0.75, y: 0.40 },
];

/// In the TelegramSwift code
/// `gatherPositions` `shiftArray` and `basePositions` were always used in combination like that:
/// ```ignore
/// gatherPositions(shiftArray(array: AnimatedGradientBackgroundView.basePositions, offset: 0))
/// ```
/// so I combined them and it's only possible to change the offset.
#[doc(alias = "gatherPositions")]
pub fn gather_positions(offset: usize) -> Vec<Point> {
    BASE_POSITIONS
        .iter()
        .cycle()
        .skip(offset)
        .step_by(2)
        .take(BASE_POSITIONS.len() / 2)
        .copied()
        .collect()
}

/// Returns buffer for a texture with BGRA8 format.
#[doc(alias = "generateGradient")]
pub fn generate_gradient(
    Size { width, height }: Size,
    colors: &[Color],
    positions: &[Point],
) -> Box<[u8]> {
    let bytes_per_row = 4 * width as usize;
    let mut image_bytes = vec![0u8; bytes_per_row * height as usize].into_boxed_slice();

    for (y, row) in image_bytes.chunks_exact_mut(bytes_per_row).enumerate() {
        let direct_pixel_y = y as f32 / height as f32;
        let center_distance_y = direct_pixel_y - 0.5;
        let center_distance_y2 = center_distance_y * center_distance_y;

        for (x, pixel) in row.chunks_exact_mut(4).enumerate() {
            let direct_pixel_x = x as f32 / height as f32;
            let center_distance_x = direct_pixel_x - 0.5;

            let center_distance =
                (center_distance_x * center_distance_x + center_distance_y2).sqrt();

            let swirl_factor = 0.35 * center_distance;
            let theta = swirl_factor * swirl_factor * 0.8 * 8.0;

            // Note: sin_cos takes noticable amount of time.
            // It's possible to get better performance
            let (sin_theta, cos_theta) = theta.sin_cos();

            let pixel_x = (0.5 + center_distance_x * cos_theta - center_distance_y * sin_theta)
                .clamp(0.0, 1.0);

            let pixel_y = (0.5 + center_distance_x * sin_theta + center_distance_y * cos_theta)
                .clamp(0.0, 1.0);

            let mut distance_sum = 0.0;
            let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);

            for (color, pos) in colors.iter().zip(positions.iter()) {
                let distance_x = pixel_x - pos.x;
                let distance_y = pixel_y - pos.y;

                let mut distance =
                    (0.92 - (distance_x * distance_x + distance_y * distance_y).sqrt()).max(0.0);

                distance = distance * distance * distance;
                distance_sum += distance;

                r += distance * color.x();
                g += distance * color.y();
                b += distance * color.z();
            }

            pixel[0] = (b / distance_sum * 255.0) as u8;
            pixel[1] = (g / distance_sum * 255.0) as u8;
            pixel[2] = (r / distance_sum * 255.0) as u8;
            pixel[3] = 255;
        }
    }
    image_bytes
}
