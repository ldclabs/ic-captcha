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
