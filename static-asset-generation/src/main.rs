use clap::Parser;
use image::ImageFormat;
use ravif::{Encoder, Img};
use rgb::FromSlice;
use static_asset_generation::{convert_not_found_images_and_rebuild_sprite, download_images, get_assets_path, get_temp_path, Args};

#[tokio::main]
async fn main() {
    let args=  Args::parse();
    let (items_modified, profile_icons_modified, perks_modified, champion_modified, summoner_spells_modified) = download_images().await.unwrap();

    convert_not_found_images_and_rebuild_sprite(
        args.items || items_modified,
        args.profile_icons|| profile_icons_modified,
        args.perks||perks_modified,
        args.champions||champion_modified,
        args.summoner_spells||summoner_spells_modified,
    )
    .await
    .unwrap();

    if args.logo{
        let logo_path = get_temp_path().join("logo.webp");
        let dest_path = get_assets_path().join("logo.avif");
        let image_data=  tokio::fs::read(&logo_path).await.unwrap();
        let image = image::load_from_memory_with_format(&image_data, ImageFormat::WebP)
            .map_err(|e| format!("Failed to load image at {}: {}", logo_path.display(), e)).unwrap();

        tokio::fs::write(
            dest_path,
            Encoder::new()
                .with_quality(100.0)
                .with_speed(1)
                .encode_rgba(Img::new(
                    image.as_bytes().as_rgba(),
                    image.width() as usize,
                    image.height() as usize,
                ))
                .unwrap()
                .avif_file,
        )
            .await
            .map_err(|e| format!("Failed to write file: {}", e)).unwrap();
    }

}