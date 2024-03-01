use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, ImageOutputFormat::Jpeg, Rgb};
use imageproc::{
    drawing::{draw_cubic_bezier_curve_mut, draw_hollow_ellipse_mut, draw_text_mut, text_size},
    noise::{gaussian_noise_mut, salt_and_pepper_noise_mut},
};
use rusttype::{Font, Scale};
use std::io::Cursor;

// Define the verification code characters.
// Remove 0, O, I, L and other easily confusing letters
const BASIC_CHAR: [char; 54] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
    'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Define a random color for a string
const LIGHT_BASIC_COLOR: [[u8; 3]; 5] = [
    [0, 140, 8],
    [5, 50, 250],
    [18, 18, 18],
    [180, 120, 60],
    [224, 44, 24],
];
const DARK_BASIC_COLOR: [[u8; 3]; 5] = [
    [248, 248, 248],
    [255, 255, 0],
    [255, 0, 255],
    [0, 255, 255],
    [0, 255, 0],
];

// Define background color
const LIGHT: [u8; 3] = [248, 248, 248];
const DARK: [u8; 3] = [18, 18, 18];

// Define font size
const SCALE_SM: Scale = Scale { x: 38.0, y: 35.0 };
const SCALE_MD: Scale = Scale { x: 45.0, y: 42.0 };
const SCALE_LG: Scale = Scale { x: 53.0, y: 50.0 };

/// A captcha should be created using the [`CaptchaBuilder`].
pub struct Captcha {
    mode: u8, // 0: dark on light, 1: colorful on light, 2: colorful on dark
    chars: Vec<char>,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Captcha {
    /// Retures the verification code string
    pub fn text(&self) -> String {
        self.chars.iter().collect()
    }

    /// Returns the verification code image in base64 format
    /// params `compression` - specify image quality, range 10-80, default is 30
    pub fn to_base64(&self, compression: u8) -> String {
        let compression = if compression > 80 {
            80
        } else if compression < 10 {
            30
        } else {
            compression
        };
        let mut buf = Cursor::new(Vec::new());
        self.image.write_to(&mut buf, Jpeg(compression)).unwrap();
        let res_base64 = general_purpose::STANDARD.encode(buf.into_inner());
        format!("data:image/jpeg;base64,{}", res_base64)
    }

    // Create a new captcha instance with the given text, width, height and dark mode
    pub(crate) fn new(text: String, width: u32, height: u32, mode: u8) -> Self {
        Captcha {
            chars: text.chars().collect(),
            image: ImageBuffer::from_fn(width, height, |_, _| {
                if mode > 1 {
                    return image::Rgb(DARK);
                }
                image::Rgb(LIGHT)
            }),
            mode,
        }
    }

    // Create a new captcha instance with random text, width, height and dark mode
    pub(crate) fn random<R>(get_rnd: &mut R, num: u8, width: u32, height: u32, mode: u8) -> Self
    where
        R: FnMut(u32) -> u32,
    {
        let mut chars: Vec<char> = Vec::with_capacity(num as usize);
        for _ in 0..num {
            chars.push(BASIC_CHAR[get_rnd(BASIC_CHAR.len() as u32) as usize])
        }

        let text: String = chars.iter().collect();
        Self::new(text, width, height, mode)
    }

    // Draw characters with given font on the captcha image.
    pub(crate) fn draw_characters<R>(&mut self, get_rnd: &mut R, font: &Font)
    where
        R: FnMut(u32) -> u32,
    {
        let x = (self.image.width() - 10) as i32 / self.chars.len() as i32;
        let h = self.image.height() as i32;

        let scale = match self.chars.len() {
            1..=4 => SCALE_LG,
            5..=6 => SCALE_MD,
            _ => SCALE_SM,
        };

        for (i, cs) in self.chars.iter().enumerate() {
            let c = cs.to_string();
            let (_, ch) = text_size(scale, font, c.as_str());
            draw_text_mut(
                &mut self.image,
                get_color(get_rnd, self.mode),
                5 + (i as i32 * x),
                rnd_between(get_rnd, 0 - (ch / 8), h + (ch / 8) - ch),
                scale,
                font,
                c.as_str(),
            );
        }
    }

    // Draw interference lines on the captcha image
    pub(crate) fn draw_interference_line<R>(&mut self, get_rnd: &mut R)
    where
        R: FnMut(u32) -> u32,
    {
        let width = self.image.width();
        let height = self.image.height();
        let x1: i32 = 5;
        let y1 = rnd_between(get_rnd, -5, height as i32);

        let x2 = width as i32 - 5;
        let y2 = rnd_between(get_rnd, -5, height as i32 + 5);

        let span = width as i32 / 10;
        let ctrl_x = rnd_between(get_rnd, span, width as i32 / 2);
        let ctrl_y = rnd_between(get_rnd, 0, height as i32);

        let ctrl_x2 = rnd_between(get_rnd, width as i32 / 2 + span, width as i32 - span);
        let ctrl_y2 = rnd_between(get_rnd, 0, height as i32);
        // Randomly draw bezier curves
        let color = get_color(get_rnd, self.mode);
        draw_cubic_bezier_curve_mut(
            &mut self.image,
            (x1 as f32, y1 as f32),
            (x2 as f32, y2 as f32),
            (ctrl_x as f32, ctrl_y as f32),
            (ctrl_x2 as f32, ctrl_y2 as f32),
            color,
        );
        draw_cubic_bezier_curve_mut(
            &mut self.image,
            (x1 as f32, y1 as f32 + 2.0),
            (x2 as f32, y2 as f32 + 2.0),
            (ctrl_x as f32, ctrl_y as f32 + 2.0),
            (ctrl_x2 as f32, ctrl_y2 as f32 + 2.0),
            color,
        );
    }

    // Draw interference circle on the captcha image
    pub(crate) fn draw_interference_ellipse<R>(&mut self, get_rnd: &mut R)
    where
        R: FnMut(u32) -> u32,
    {
        let w = rnd_between(get_rnd, 5, self.image.height() as i32 / 3);
        let x = rnd_between(get_rnd, 5, self.image.width() as i32 - 5);
        let y = rnd_between(get_rnd, 5, self.image.height() as i32 - 5);
        let color = get_color(get_rnd, self.mode);
        draw_hollow_ellipse_mut(&mut self.image, (x, y), w * 2, w, color);
        draw_hollow_ellipse_mut(&mut self.image, (x, y), w * 2 + 2, w + 2, color);
    }

    // Draw interference noise on the captcha image
    pub(crate) fn draw_interference_noise<R>(&mut self, get_rnd: &mut R, complexity: u32)
    where
        R: FnMut(u32) -> u32,
    {
        if complexity > 1 {
            gaussian_noise_mut(
                &mut self.image,
                (complexity - 1) as f64,
                (4 * complexity) as f64,
                get_rnd(u32::MAX) as u64,
            );
            salt_and_pepper_noise_mut(
                &mut self.image,
                (0.002 * complexity as f64) - 0.002,
                get_rnd(u32::MAX) as u64,
            );
        }
    }
}

// Return a random color with given mode
fn get_color<R>(get_rnd: &mut R, mode: u8) -> Rgb<u8>
where
    R: FnMut(u32) -> u32,
{
    match mode {
        0 => Rgb(DARK),
        1 => Rgb(LIGHT_BASIC_COLOR[get_rnd(LIGHT_BASIC_COLOR.len() as u32) as usize]),
        _ => Rgb(DARK_BASIC_COLOR[get_rnd(DARK_BASIC_COLOR.len() as u32) as usize]),
    }
}

// Return a random number between two numbers
fn rnd_between<R>(get_rnd: &mut R, min: i32, max: i32) -> i32
where
    R: FnMut(u32) -> u32,
{
    if min >= max {
        return min;
    }

    min + get_rnd((max - min) as u32) as i32
}
