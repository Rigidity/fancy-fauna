mod layers;
mod nft_trait;

use std::collections::HashSet;

use anyhow::Result;
use image::{imageops::FilterType, ColorType, DynamicImage, GenericImage, GenericImageView, Rgba};
use layers::{Animal, AnimalColor, Background, BackgroundColor, Foreground, ForegroundColor};
use nft_trait::Trait;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() -> Result<()> {
    let mut rng = ChaCha20Rng::seed_from_u64(1337);
    let mut images = Vec::new();
    let mut traits = HashSet::new();

    while images.len() < 1000 {
        let foreground = Foreground::random(&mut rng);
        let foreground_color = ForegroundColor::random(&mut rng);
        let animal = Animal::random(&mut rng);
        let animal_color = AnimalColor::random(&mut rng);
        let background = Background::random(&mut rng);
        let background_color = BackgroundColor::random(&mut rng);

        if !traits.insert((
            foreground,
            foreground_color,
            animal,
            animal_color,
            background,
            background_color,
        )) {
            continue;
        }

        let mut foreground = custom_foreground(foreground, foreground_color.rgba())?;
        let animal = custom_animal(animal, animal_color.rgba())?;

        copy_non_transparent_pixels(&mut foreground, &animal);

        let (primary_color, secondary_color) = background_color.rgba();
        let mut image = custom_background(background, primary_color, secondary_color)?;
        copy_non_transparent_pixels(&mut image, &foreground);

        images.push(image);
    }

    let mut collage = DynamicImage::new(32 * 32, 32 * 32, ColorType::Rgba8);

    let mut x = 0;
    let mut y = 0;

    for (i, image) in images.iter().take(1000).enumerate() {
        let bigger_image = image.resize(32 * 16, 32 * 16, FilterType::Nearest);
        bigger_image.save(format!("images/image_{}.png", i + 1))?;

        collage.copy_from(image, x * 32, y * 32)?;

        x += 1;
        if x == 32 {
            x = 0;
            y += 1;
        }
    }

    collage.save("collage.png")?;

    Ok(())
}

fn custom_animal(animal: Animal, color: Rgba<u8>) -> Result<DynamicImage> {
    let mut image = image::open(match animal {
        Animal::Cat => "Animals/Cat.png",
        Animal::Fox => "Animals/Fox.png",
        Animal::Rabbit => "Animals/Rabbit.png",
        Animal::Budgie => "Animals/Budgie.png",
    })?;

    for rgba in image.as_mut_rgba8().unwrap().pixels_mut() {
        if rgba.0[3] == 0 || is_white(rgba) || is_black(rgba) {
            continue;
        }
        *rgba = color;
    }

    Ok(image)
}

fn custom_background(
    background: Background,
    primary_color: Rgba<u8>,
    secondary_color: Rgba<u8>,
) -> Result<DynamicImage> {
    let mut image = image::open(match background {
        Background::Plain => "Backgrounds/Plain.png",
        Background::Vertical => "Backgrounds/Vertical.png",
        Background::Horizontal => "Backgrounds/Horizontal.png",
        Background::Radial => "Backgrounds/Radial.png",
        Background::Squares => "Backgrounds/Squares.png",
    })?;

    for rgba in image.as_mut_rgba8().unwrap().pixels_mut() {
        if rgba.0[3] == 0 || is_white(rgba) || is_black(rgba) {
            continue;
        }

        if rgba.0[0] == 0 && rgba.0[1] == 255 && rgba.0[2] == 0 {
            *rgba = primary_color;
        } else {
            *rgba = secondary_color;
        }
    }

    Ok(image)
}

fn custom_foreground(foreground: Foreground, color: Rgba<u8>) -> Result<DynamicImage> {
    let mut image = image::open(match foreground {
        Foreground::Ramp => "Foregrounds/Ramp.png",
        Foreground::Wall => "Foregrounds/Wall.png",
        Foreground::Wave => "Foregrounds/Wave.png",
    })?;

    for rgba in image.as_mut_rgba8().unwrap().pixels_mut() {
        if rgba.0[3] == 0 || is_white(rgba) || is_black(rgba) {
            continue;
        }

        *rgba = color;
    }

    Ok(image)
}

fn is_white(pixel: &Rgba<u8>) -> bool {
    pixel.0 == [255, 255, 255, 255]
}

fn is_black(pixel: &Rgba<u8>) -> bool {
    pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 && pixel.0[3] > 0
}

fn copy_non_transparent_pixels(image: &mut DynamicImage, from: &DynamicImage) {
    for (x, y, pixel) in from.pixels() {
        if pixel.0[3] == 0 || (pixel.0[3] < 255 && image.get_pixel(x, y).0[3] == 0) {
            continue;
        }

        // If pixel has any opacity
        let background = image.get_pixel(x, y);
        let alpha = pixel.0[3] as f32 / 255.0;

        // Blend each color channel (RGB)
        let blended = Rgba([
            blend_channel(pixel.0[0], background.0[0], alpha),
            blend_channel(pixel.0[1], background.0[1], alpha),
            blend_channel(pixel.0[2], background.0[2], alpha),
            blend_opacity(pixel.0[3], background.0[3]),
        ]);

        image.put_pixel(x, y, blended);
    }
}

// Helper function to blend a single color channel
fn blend_channel(foreground: u8, background: u8, alpha: f32) -> u8 {
    let fg = foreground as f32;
    let bg = background as f32;
    (fg * alpha + bg * (1.0 - alpha)) as u8
}

// Helper function to blend opacity values
fn blend_opacity(foreground: u8, background: u8) -> u8 {
    let alpha_f = foreground as f32 / 255.0;
    let alpha_b = background as f32 / 255.0;
    ((alpha_f + alpha_b * (1.0 - alpha_f)) * 255.0) as u8
}
