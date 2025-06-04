use std::{env, path::Path};

use charming::{theme::Theme, ImageRenderer};
use charming_gallery::CHARTS;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <output-dir>", args[0]);
        return;
    }

    let output_dir = Path::new(&args[1]);
    let mut renderer = ImageRenderer::new(600, 450);

    for (key, value) in CHARTS.iter() {
        // Not yet supported by SSR.
        if let "aria" = *key {
            continue;
        }

        let dir = output_dir.join(key);
        std::fs::create_dir_all(&dir).unwrap();
        for (name, chart) in value.iter() {
            println!("Rendering {key}/{name}");
            let path = dir.join(format!("{name}.svg"));
            renderer.save(&chart(), &path).unwrap();
            // let path = dir.join(format!("{}.png", name));
            // renderer
            //     .save_format(ImageFormat::Png, &chart(), &path)
            //     .unwrap();
        }
    }

    let rainfall_chart = charming_gallery::CHARTS["line"]["rainfall"]();
    let theme_dir = output_dir.join("theme");
    std::fs::create_dir_all(&theme_dir).unwrap();
    let themes = [
        Theme::Default,
        Theme::Dark,
        Theme::Vintage,
        Theme::Westeros,
        Theme::Essos,
        Theme::Wonderland,
        Theme::Walden,
        Theme::Chalk,
        Theme::Infographic,
        Theme::Macarons,
        Theme::Roma,
        Theme::Shine,
        Theme::PurplePassion,
        Theme::Halloween,
    ];

    for theme in themes {
        let mut theme_renderer = ImageRenderer::new(650, 400).theme(theme.clone());
        let theme_name = match theme {
            Theme::Default => "default",
            Theme::Dark => "dark",
            Theme::Vintage => "vintage",
            Theme::Westeros => "westeros",
            Theme::Essos => "essos",
            Theme::Wonderland => "wonderland",
            Theme::Walden => "walden",
            Theme::Chalk => "chalk",
            Theme::Infographic => "infographic",
            Theme::Macarons => "macarons",
            Theme::Roma => "roma",
            Theme::Shine => "shine",
            Theme::PurplePassion => "purple-passion",
            Theme::Halloween => "halloween",
            Theme::Custom(_, _) => todo!(),
        };

        println!("Rendering theme/{theme_name}");
        theme_renderer
            .save(&rainfall_chart, theme_dir.join(format!("{theme_name}.svg")))
            .unwrap();
    }
}
