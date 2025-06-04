use resvg::{
    tiny_skia::{self, Pixmap},
    usvg::{self, Options},
};
use std::path::{Path, PathBuf};
use test_each_file::test_each_path;

test_each_path! { in "./img" => image_snapshot }
fn image_snapshot(old_path: &Path) {
    let test_path_elements = old_path.iter().map(|e| {
        if e.to_str().unwrap() == "img" {
            "img_test"
        } else {
            e.to_str().unwrap()
        }
    });
    let mut test_path = PathBuf::new();
    for e in test_path_elements {
        test_path.push(e);
    }

    if !test_path.exists() {
        panic!("Unable to find {test_path:?}, did you run \n`cargo r --bin generate_images img_test` \nto create the images for comparison?")
    };

    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();

    let old_pixmap = get_rgba_data(old_path, &opt);
    let test_pixmap = get_rgba_data(&test_path, &opt);

    assert_eq!(old_pixmap, test_pixmap);
}

fn get_rgba_data(path: &Path, opt: &Options) -> Pixmap {
    let svg_data = std::fs::read(path).unwrap();
    let tree = usvg::Tree::from_data(&svg_data, opt).unwrap();

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    // let mut png_path = path.with_extension("png");
    // pixmap.save_png(png_path).unwrap();
    pixmap
}
