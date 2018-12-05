use std::fs;
use std::path::Path;

extern crate image;
use image::GenericImageView;
use image::DynamicImage;
use std::path::PathBuf;


fn main() {

    let v = find_all_files(Path::new("images"));

    let v = load_images(v);

    let (x,y) = v.iter().next().unwrap().dimensions();

    let mut result = image::RgbImage::new(x,y);

    result.enumerate_pixels_mut().for_each( |(px,py, pix)| {
        let mut acc_r :u64 =0;
        let mut acc_g :u64 =0;
        let mut acc_b :u64 =0;
        let mut cnt =0;
        for img in &v{
            acc_r+=img.get_pixel(px,py).data[0] as u64;
            acc_g+=img.get_pixel(px,py).data[1] as u64;
            acc_b+=img.get_pixel(px,py).data[2] as u64;
            cnt+=1;
        }
        let res_r = (acc_r/cnt) as u8;
        let res_g = (acc_g/cnt) as u8;
        let res_b = (acc_b/cnt) as u8;
        *pix = image::Rgb([res_r,res_g,res_b])
    }
    );
    result.save("out.jpg").unwrap();
}


fn find_all_files(dir: &Path) -> Vec<PathBuf> {

    let mut v = vec![];

    if !dir.is_dir() {
        panic!("passed dir is no directory")
    }

    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file) = file {
                println!("{:?}", file.path());
                v.push(file.path());
            }
        }
    }
    return v;
}

fn load_images(paths: Vec<PathBuf>) -> Vec<DynamicImage> {
    let mut v = vec![];
    for path in paths{
        let img = image::open(&path);
        if let Ok(img) = img {
            v.push(img);
        }
        else{
            panic!("Could not read image: {:?}",path);
        }
    }
    return v;
}