mod layers;
mod metadata;
mod nft_trait;

use std::{collections::HashSet, fs, num::NonZeroUsize};

use anyhow::Result;
use image::{imageops::FilterType, ColorType, DynamicImage, GenericImage, GenericImageView, Rgba};
use indexmap::IndexMap;
use layers::{
    Animal, AnimalColor, Background, BackgroundColor, Foreground, ForegroundColor, Overlay,
};
use metadata::{AttributeValue, Chip0007Metadata, Collection, CollectionAttribute, NftAttribute};
use nft_trait::Trait;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Traits {
    foreground: Foreground,
    foreground_color: ForegroundColor,
    animal: Animal,
    animal_color: AnimalColor,
    background: Background,
    background_color: BackgroundColor,
    overlay: Overlay,
}

fn main() -> Result<()> {
    let mut rng = ChaCha20Rng::seed_from_u64(1337);
    let mut images = Vec::new();
    let mut seen_traits = HashSet::new();

    while images.len() < 1000 {
        let traits = Traits {
            foreground: Foreground::random(&mut rng),
            foreground_color: ForegroundColor::random(&mut rng),
            animal: Animal::random(&mut rng),
            animal_color: AnimalColor::random(&mut rng),
            background: Background::random(&mut rng),
            background_color: BackgroundColor::random(&mut rng),
            overlay: Overlay::random(&mut rng),
        };

        if traits.animal_color == AnimalColor::Alpha
            && matches!(traits.overlay, Overlay::Lasers | Overlay::Xch)
        {
            continue;
        }

        if !seen_traits.insert(traits) {
            continue;
        }

        let mut foreground = custom_foreground(traits.foreground, traits.foreground_color.rgba())?;
        let animal = custom_animal(traits.animal, traits.animal_color.rgba())?;

        copy_non_transparent_pixels(&mut foreground, &animal, 0, 0);

        let (primary_color, secondary_color) = traits.background_color.rgba();
        let mut image = custom_background(traits.background, primary_color, secondary_color)?;
        copy_non_transparent_pixels(&mut image, &foreground, 0, 0);

        let (x, y) = traits.overlay.position(traits.animal);
        let overlay = custom_overlay(traits.overlay)?;
        copy_non_transparent_pixels(&mut image, &overlay, x, y);

        images.push((image, traits));
    }

    let mut animals = IndexMap::new();
    let mut animal_colors = IndexMap::new();
    let mut backgrounds = IndexMap::new();
    let mut background_colors = IndexMap::new();
    let mut foregrounds = IndexMap::new();
    let mut foreground_colors = IndexMap::new();
    let mut overlays = IndexMap::new();

    for traits in seen_traits {
        *animals.entry(traits.animal).or_insert(0) += 1;
        *animal_colors.entry(traits.animal_color).or_insert(0) += 1;
        *backgrounds.entry(traits.background).or_insert(0) += 1;
        *background_colors
            .entry(traits.background_color)
            .or_insert(0) += 1;
        *foregrounds.entry(traits.foreground).or_insert(0) += 1;
        *foreground_colors
            .entry(traits.foreground_color)
            .or_insert(0) += 1;
        *overlays.entry(traits.overlay).or_insert(0) += 1;
    }

    println!("Animals and their probabilities");
    println!("{:?}", animals);
    println!(
        "{:?}",
        animals
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    println!("\nAnimal colors and their probabilities");
    println!("{:?}", animal_colors);
    println!(
        "{:?}",
        animal_colors
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    println!("\nBackgrounds and their probabilities");
    println!("{:?}", backgrounds);
    println!(
        "{:?}",
        backgrounds
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    println!("\nBackground colors and their probabilities");
    println!("{:?}", background_colors);
    println!(
        "{:?}",
        background_colors
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    println!("\nForegrounds and their probabilities");
    println!("{:?}", foregrounds);
    println!(
        "{:?}",
        foregrounds
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    println!("\nOverlays and their probabilities");
    println!("{:?}", overlays);
    println!(
        "{:?}",
        overlays
            .keys()
            .map(|k| (k, k.probability()))
            .collect::<IndexMap<_, _>>()
    );

    let mut collage = DynamicImage::new(32 * 32, 32 * 32, ColorType::Rgba8);
    let mut banner = DynamicImage::new(32 * 8 * 8, 32 * 8 * 4, ColorType::Rgba8);

    let mut x = 0;
    let mut y = 0;

    fs::create_dir_all("images")?;
    fs::create_dir_all("metadata")?;

    let mut image_hashes = Vec::new();
    let mut metadata_hashes = Vec::new();

    let mut banner_x = 0;
    let mut banner_y = 0;

    for (i, (image, traits)) in images.iter().enumerate() {
        let bigger_image = image.resize(32 * 32, 32 * 32, FilterType::Nearest);
        bigger_image.save(format!("images/image_{}.png", i + 1))?;

        let mut hasher = Sha256::new();
        hasher.update(fs::read(format!("images/image_{}.png", i + 1))?);
        let hash = hasher.finalize();
        image_hashes.push(hex::encode(hash));

        let metadata = Chip0007Metadata {
            format: "CHIP-0007".to_string(),
            name: format!("Fancy Fauna #{}", i + 1),
            description: "1,000 unique NFTs on the Chia blockchain with a variety of colorful pixel-art creatures!".to_string(),
            minting_tool: Some("MintGarden's Secure the Mint".to_string()),
            series_number: Some(NonZeroUsize::new(i + 1).unwrap()),
            series_total: Some(NonZeroUsize::new(1000).unwrap()),
            attributes: Some(vec![
                NftAttribute {
                    trait_type: AttributeValue::String("Animal".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.animal)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Animal Color".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.animal_color)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Background".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.background)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Background Color".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.background_color)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Foreground".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.foreground)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Foreground Color".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.foreground_color)),
                    min_value: None,
                    max_value: None,
                },
                NftAttribute {
                    trait_type: AttributeValue::String("Overlay".to_string()),
                    value: AttributeValue::String(format!("{:?}", traits.overlay)),
                    min_value: None,
                    max_value: None,
                },
            ]),
            collection: Some(Collection {
                id: "1efd5e73-fada-6140-b8ef-fa84fe808a6f".parse()?,
                name: "Fancy Fauna".to_string(),
                attributes: Some(vec![
                    CollectionAttribute {
                        kind: AttributeValue::String("description".to_string()),
                        value: AttributeValue::String("1,000 unique NFTs on the Chia blockchain with a variety of colorful pixel-art creatures!".to_string()),
                    },
                    CollectionAttribute {
                        kind: AttributeValue::String("icon".to_string()),
                        value: AttributeValue::String("https://fancyfauna.com/icon.png".to_string()),
                    },
                    CollectionAttribute {
                        kind: AttributeValue::String("banner".to_string()),
                        value: AttributeValue::String("https://fancyfauna.com/banner.png".to_string()),
                    },
                    CollectionAttribute {
                        kind: AttributeValue::String("twitter".to_string()),
                        value: AttributeValue::String("@fancy_fauna".to_string()),
                    },
                    CollectionAttribute {
                        kind: AttributeValue::String("website".to_string()),
                        value: AttributeValue::String("https://fancyfauna.com".to_string()),
                    },
                ]),
            }),
        };

        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(format!("metadata/metadata_{}.json", i + 1), metadata_json)?;

        let mut hasher = Sha256::new();
        hasher.update(fs::read(format!("metadata/metadata_{}.json", i + 1))?);
        let hash = hasher.finalize();
        metadata_hashes.push(hex::encode(hash));

        collage.copy_from(image, x * 32, y * 32)?;

        if banner_y < 4 {
            banner.copy_from(
                &image.resize(32 * 8, 32 * 8, FilterType::Nearest),
                banner_x * 32 * 8,
                banner_y * 32 * 8,
            )?;
            banner_x += 1;
            if banner_x == 8 {
                banner_x = 0;
                banner_y += 1;
            }
        }

        x += 1;
        if x == 32 {
            x = 0;
            y += 1;
        }
    }

    fs::write("image_hashes.txt", image_hashes.join("\n"))?;
    fs::write("metadata_hashes.txt", metadata_hashes.join("\n"))?;

    collage.save("collage.png")?;
    banner.save("banner.png")?;

    Ok(())
}

fn custom_animal(animal: Animal, color: Rgba<u8>) -> Result<DynamicImage> {
    let mut image = image::open(match animal {
        Animal::Cat => "Animals/Cat.png",
        Animal::Dog => "Animals/Dog.png",
        Animal::Fox => "Animals/Fox.png",
        Animal::Rabbit => "Animals/Rabbit.png",
        Animal::Budgie => "Animals/Budgie.png",
        Animal::Duck => "Animals/Duck.png",
    })?;

    for rgba in image.as_mut_rgba8().unwrap().pixels_mut() {
        if rgba.0[3] == 0 || is_black(rgba) {
            continue;
        }

        if is_white(rgba) {
            if color.0[3] == 255 {
                continue;
            }

            rgba.0 = [0, 0, 0, 0];
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
        Background::Frame => "Backgrounds/Frame.png",
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

fn custom_overlay(overlay: Overlay) -> Result<DynamicImage> {
    Ok(image::open(match overlay {
        Overlay::None => return Ok(DynamicImage::new(32, 32, ColorType::Rgba8)),
        Overlay::Halo => "Overlays/Halo.png",
        Overlay::Sunglasses => "Overlays/Sunglasses.png",
        Overlay::Lasers => "Overlays/Lasers.png",
        Overlay::Heart => "Overlays/Heart.png",
        Overlay::Sprout => "Overlays/Sprout.png",
        Overlay::Rust => "Overlays/Rust.png",
        Overlay::Xch => "Overlays/XCH.png",
    })?)
}

fn is_white(pixel: &Rgba<u8>) -> bool {
    pixel.0 == [255, 255, 255, 255]
}

fn is_black(pixel: &Rgba<u8>) -> bool {
    pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 && pixel.0[3] > 0
}

fn copy_non_transparent_pixels(
    image: &mut DynamicImage,
    from: &DynamicImage,
    offset_x: u32,
    offset_y: u32,
) {
    for (x, y, pixel) in from.pixels() {
        let dest_x = x + offset_x;
        let dest_y = y + offset_y;

        // Skip if destination coordinates are out of bounds
        if dest_x >= image.width() || dest_y >= image.height() {
            continue;
        }

        if pixel.0[3] == 0 || (pixel.0[3] < 255 && image.get_pixel(dest_x, dest_y).0[3] == 0) {
            continue;
        }

        // If pixel has any opacity
        let background = image.get_pixel(dest_x, dest_y);
        let alpha = pixel.0[3] as f32 / 255.0;

        // Blend each color channel (RGB)
        let blended = Rgba([
            blend_channel(pixel.0[0], background.0[0], alpha),
            blend_channel(pixel.0[1], background.0[1], alpha),
            blend_channel(pixel.0[2], background.0[2], alpha),
            blend_opacity(pixel.0[3], background.0[3]),
        ]);

        image.put_pixel(dest_x, dest_y, blended);
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
