use std::{env, path::Path};

use charming::ImageRenderer;
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
            println!("Rendering {}/{}", key, name);
            let path = dir.join(format!("{}.svg", name));
            renderer.save(&chart(), &path).unwrap();
            // let path = dir.join(format!("{}.png", name));
            // renderer
            //     .save_format(ImageFormat::Png, &chart(), &path)
            //     .unwrap();
        }
    }
}
