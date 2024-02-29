#![doc(html_root_url = "https://docs.rs/captcha-rs/latest")]

//! Generate a verification image.
//!
//! ```rust
//! use ic_captcha::CaptchaBuilder;
//!
//! let builder = CaptchaBuilder::new()
//!   .length(4)
//!   .width(140)
//!   .height(60)
//!   .mode(1)
//!   .complexity(4);
//!
//! let captcha = builder.generate(b"random seed 0", None);
//! println!("text: {}", captcha.text());
//! println!("base_img: {}", captcha.to_base64(30));
//! ```

mod captcha;

use captcha::Captcha;
use sha3::{Digest, Sha3_256};

/// The default font used to generate the captcha image.
pub static FONTS: &[u8] = include_bytes!("../fonts/arial-rounded-bold.ttf");

/// A builder struct for creating a [`Captcha`].
pub struct CaptchaBuilder {
    fonts: rusttype::Font<'static>,
    length: u8,
    width: u32,
    height: u32,
    mode: u8,
    complexity: u32,
}

impl Default for CaptchaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptchaBuilder {
    /// Returns a [`CaptchaBuilder`] with default configuration.
    pub fn new() -> Self {
        CaptchaBuilder {
            length: 4,
            fonts: rusttype::Font::try_from_bytes(FONTS).unwrap(),
            width: 140,
            height: 40,
            mode: 1u8,
            complexity: 4,
        }
    }

    /// Set the length of the verification code string, default is 4.
    pub fn length(mut self, length: u8) -> Self {
        self.length = if length > 0 { length } else { 4 };
        self
    }

    /// Set the font used to generate the captcha image, default is arial-rounded-bold.ttf.
    pub fn fonts(mut self, fonts: rusttype::Font<'static>) -> Self {
        self.fonts = fonts;
        self
    }

    /// Set the width of the verification code image, default is 140.
    pub fn width(mut self, width: u32) -> Self {
        self.width = if width > 60 { width } else { 140 };
        self
    }

    /// Set the height of the verification code image, default is 40.
    pub fn height(mut self, height: u32) -> Self {
        self.height = if height > 20 { height } else { 40 };
        self
    }

    /// Set the color mode of the verification code image, default is 1.
    /// 0: dark on light, 1: colorful on light, 2: colorful on dark.
    pub fn mode(mut self, mode: u8) -> Self {
        self.mode = mode;
        self
    }

    /// Set the complexity of the verification code image, default is 4.
    pub fn complexity(mut self, complexity: u32) -> Self {
        self.complexity = if complexity > 10 {
            10
        } else if complexity < 1 {
            1
        } else {
            complexity
        };
        self
    }

    /// Generate a [`Captcha`] with the given random seed and a optional text.
    /// If the text is not provided, a text will be generated from random seed.
    /// The random seed can be used only once. You should use a new seed for each new captcha.
    pub fn generate(&self, seed: &[u8], text: Option<String>) -> Captcha {
        let mut rnd = Rnd::new(seed);
        let mut get_rnd_32 = |num: u32| rnd.rnd_32(num);
        let mut captcha = match text {
            Some(text) => Captcha::new(text, self.width, self.height, self.mode),
            None => Captcha::random(
                &mut get_rnd_32,
                self.length,
                self.width,
                self.height,
                self.mode,
            ),
        };

        // Loop to write the verification code string into the background image
        captcha.cyclic_write_character(&mut get_rnd_32, &self.fonts);

        captcha.draw_interference_line(&mut get_rnd_32);
        captcha.draw_interference_line(&mut get_rnd_32);

        captcha.draw_interference_ellipse(&mut get_rnd_32);
        captcha.draw_interference_ellipse(&mut get_rnd_32);
        captcha.draw_interference_ellipse(&mut get_rnd_32);

        captcha.draw_interference_noise(&mut get_rnd_32, self.complexity);

        captcha
    }
}

// A simple random number generator with a fixed seed
struct Rnd {
    offset: usize,
    seed: [u8; 32],
}

impl Rnd {
    fn new(seed: &[u8]) -> Self {
        Rnd {
            offset: 0,
            seed: next_seed(seed),
        }
    }

    // Generate a random number between 0 and num with the given seed
    fn rnd_32(&mut self, num: u32) -> u32 {
        let mut d = [0u8; 4];
        d.copy_from_slice(&self.seed[self.offset..self.offset + 4]);
        self.offset += 4;
        if self.offset >= 32 {
            self.seed = next_seed(&self.seed);
            self.offset = 0;
        }
        u32::from_le_bytes(d) % num
    }
}

// Generate a new seed from the given seed using SHA3-256
fn next_seed(seed: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(seed);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use crate::CaptchaBuilder;

    #[test]
    fn it_generates_a_captcha() {
        let builder = CaptchaBuilder::new();

        let captcha = builder.generate(&[0u8, 32], None);
        assert_eq!(captcha.text().as_str(), "UmfU");
        let base_img = captcha.to_base64(0);
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
        println!("text: {}", captcha.text());
        println!("base_img: {}", base_img);

        let captcha2 = builder.generate(&[0u8, 32], None);
        assert_eq!(captcha2.text().as_str(), "UmfU");
        assert_eq!(base_img, captcha2.to_base64(0));

        let captcha2 = builder.generate(&[0u8, 32], Some("LDCLabs".to_string()));
        assert_eq!(captcha2.text().as_str(), "LDCLabs");
        assert_ne!(base_img, captcha2.to_base64(0));
    }

    #[test]
    fn it_generates_captcha_using_builder() {
        let captcha = CaptchaBuilder::new()
            .length(4)
            .width(120)
            .height(60)
            .mode(0)
            .complexity(8)
            .generate(&[1u8, 32], None);

        assert_eq!(captcha.text().len(), 4);
        let base_img = captcha.to_base64(10);
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
        println!("text: {}", captcha.text());
        println!("base_img: {}", base_img);
    }
}
