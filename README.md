# ic-captcha

![License](https://img.shields.io/crates/l/ic-captcha.svg)
[![Crates.io](https://img.shields.io/crates/d/ic-captcha.svg)](https://crates.io/crates/ic-captcha)
[![Codecov](https://codecov.io/gh/ldclabs/ic-captcha/branch/main/graph/badge.svg)](https://codecov.io/gh/ldclabs/ic-captcha)
[![CI](https://github.com/ldclabs/ic-captcha/actions/workflows/ci.yml/badge.svg)](https://github.com/ldclabs/ic-captcha/actions/workflows/ci.yml)
[![Docs.rs](https://img.shields.io/docsrs/ic-captcha?label=docs.rs)](https://docs.rs/ic-captcha)
[![Latest Version](https://img.shields.io/crates/v/ic-captcha.svg)](https://crates.io/crates/ic-captcha)

**ic-captcha** is a library that generating CAPTCHAs with given random bytes for the Internet Computer.

It is inspired by [captcha-rs](https://github.com/samirdjelal/captcha-rs).

## Usage

See examples and the [API documentation] for more.

### Using mode method

| CaptchaBuilder::mode       | Captcha Preview                                    |
| -------------------------- | -------------------------------------------------- |
| mode(0): dark on light     | ![captcha-mode-0.jpeg](images/captcha-mode-0.jpeg) |
| mode(1): colorful on light | ![captcha-mode-0.jpeg](images/captcha-mode-1.jpeg) |
| mode(2): colorful on dark  | ![captcha-mode-0.jpeg](images/captcha-mode-2.jpeg) |

### Using complexity method

| CaptchaBuilder::complexity | Captcha Preview                                                  |
| -------------------------- | ---------------------------------------------------------------- |
| complexity(1)              | ![captcha-complexity-1.jpeg](images/captcha-complexity-1.jpeg)   |
| complexity(5)              | ![captcha-complexity-5.jpeg](images/captcha-complexity-5.jpeg)   |
| complexity(10)             | ![captcha-complexity-10.jpeg](images/captcha-complexity-10.jpeg) |

### Using compression

| Captcha::to_base64 | Captcha Preview                                                    |
| ------------------ | ------------------------------------------------------------------ |
| to_base64(10)      | ![captcha-compression-10.jpeg](images/captcha-compression-10.jpeg) |
| to_base64(40)      | ![captcha-compression-40.jpeg](images/captcha-compression-40.jpeg) |
| to_base64(80)      | ![captcha-compression-80.jpeg](images/captcha-compression-80.jpeg) |

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
ic-captcha = "0.2"
```

And then get started in your `main.rs`:

```rust
use ic_captcha::CaptchaBuilder;

fn main() {
    {
        let builder = CaptchaBuilder::new();

        let captcha = builder.generate(b"random seed 0", None);
        println!("text: {}", captcha.text());
        println!("base_img: {}", captcha.to_base64(0));

        let captcha = builder.generate(b"random seed 1", None);
        println!("text: {}", captcha.text());
        println!("base_img: {}", captcha.to_base64(0));
    }

    {
        // same as default
        let builder = CaptchaBuilder::new()
            .length(4)
            .width(140)
            .height(60)
            .mode(1)
            .complexity(4);

        let captcha = builder.generate(b"random seed 0", None);
        println!("text: {}", captcha.text());
        println!("base_img: {}", captcha.to_base64(30));
    }
}

```

[API documentation]: https://docs.rs/ic-captcha

## License

Copyright © 2024-present [LDC Labs](https://github.com/ldclabs).

`ldclabs/ic-captcha` is licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a>.
