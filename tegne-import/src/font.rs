use msdfgen::Bitmap;
use msdfgen::FontExt;
use msdfgen::Gray;
use msdfgen::Range;
use msdfgen::EDGE_THRESHOLD;
use msdfgen::OVERLAP_SUPPORT;
use msdfgen_lib as _;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ttf_parser::Font;

pub fn import_font(in_path: &Path, out_path: &Path) {
    println!("Compiling {:?}", in_path);

    let width = 32;
    let height = 32;

    let font_src = fs::read(in_path).expect("cannot read input file");
    let font = Font::from_data(&font_src, 0).expect("invalid ttf");

    let A = CharData::new(&font, 'A', width, height);
}

struct CharData {}

impl CharData {
    pub fn new(font: &Font, c: char, width: u32, height: u32) -> Self {
        let glyph = font
            .glyph_index(c)
            .expect(&format!("font does not include '{}'", c));
        let mut shape = font.glyph_shape(glyph).expect("cannot get shape");

        let bounds = shape.get_bounds();
        let framing = bounds
            .autoframe(width, height, Range::Px(4.0), None)
            .expect("cannot autoframe");

        let mut bitmap = Bitmap::new(width, height);

        shape.edge_coloring_simple(3.0, 0);
        shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);
        shape.correct_sign(&mut bitmap, &framing, Default::default());

        let error = shape.estimate_error(&mut bitmap, &framing, 5, Default::default());

        println!("Estimated error: {}", error);

        bitmap.flip_y();

        let mut output = File::create(format!("{}-msdf.png", c)).expect("cannot create file");
        bitmap.write_png(&mut output).expect("cannot write bitmap");

        let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);
        bitmap.render(&mut preview, Default::default());

        let mut output = File::create(format!("{}-preview.png", c)).unwrap();
        preview.write_png(&mut output).unwrap();

        Self {}
    }
}
