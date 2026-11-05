#![cfg(all(not(target_arch = "wasm32"), feature = "ssr-raster"))]

use charming::{ImageFormat, ImageRenderer};

#[path = "../../gallery/src/echarts6.rs"]
mod examples;

#[test]
fn new_series_and_registered_extensions_render_foreground_pngs() {
    let mut renderer = ImageRenderer::new(600, 450).locale("EN");
    renderer.register_extension(examples::EXTENSIONS).unwrap();
    assert!(
        renderer
            .register_extension("throw new Error('extension failure')")
            .is_err()
    );
    for (name, chart) in [
        ("matrix", examples::matrix()),
        ("chord", examples::chord()),
        ("violin", examples::violin()),
        ("bar-range", examples::bar_range()),
    ] {
        let png = renderer.render_format(ImageFormat::Png, &chart).unwrap();
        let image = image::load_from_memory(&png).unwrap().to_rgba8();
        assert_eq!(image.dimensions(), (600, 450), "{name}");
        // Exclude the title and axes: count saturated pixels from the data marks.
        let foreground = image
            .enumerate_pixels()
            .filter(|(x, y, p)| {
                let channels = &p.0[..3];
                *x > 75
                    && *x < 560
                    && *y > 85
                    && *y < 380
                    && p.0[3] > 100
                    && channels.iter().max().unwrap() - channels.iter().min().unwrap() > 40
            })
            .count();
        assert!(
            foreground > 300,
            "{name} has only {foreground} foreground pixels"
        );
    }
}
