use std::fs;
use std::path::Path;

extern crate image;
use image::GenericImageView;
use image::DynamicImage;
use std::path::PathBuf;

/**
    This program loads all jpeg images in the images folder and merges them pixel per pixel
    This is by far not the best way but it scales up to about 256 Jpegs with about 16Mp on my laptop.
    This program was written because I could not find any solutions to merge such a number of images.
    Imagemagic was able to merge about 15 images before it hogged all my RAM and Swap.
*/
fn main() {

    let v = find_all_files(Path::new("images"));

    let v = load_images(v);

    let (x,y) = v.iter().next().unwrap().dimensions();

    // The resulting, merged image. It has the same dimensions as all other images.
    let mut result = image::RgbImage::new(x,y);

    // accumulators for the pixel data u64 might be overkill but who cares?
    let mut acc_r :u64 =0;
    let mut acc_g :u64 =0;
    let mut acc_b :u64 =0;

    let mut res_r :u8 =0;
    let mut res_g :u8 =0;
    let mut res_b :u8 =0;

    // the count of images, used to average them
    let cnt = v.len();

    // iterate over the result image and set its pixels as the average of the pixel of the
    // other images at the exact same position
    result.enumerate_pixels_mut().for_each( |(px,py, pix)| {

        // average the colors separately
        for img in &v{
            acc_r+=img.get_pixel(px,py).data[0] as u64;
            acc_g+=img.get_pixel(px,py).data[1] as u64;
            acc_b+=img.get_pixel(px,py).data[2] as u64;
        }

        res_r = (acc_r/cnt) as u8;
        res_g = (acc_g/cnt) as u8;
        res_b = (acc_b/cnt) as u8;

        // reset the accumulators for the next pixel
        acc_r=0;
        acc_g=0;
        acc_b=0;

        *pix = image::Rgb([res_r,res_g,res_b])
    }
    );
    // finally write out the pixels
    result.save("out.jpg").unwrap();
}


/**
    Find all jpeg files in the passed directory and add the files object to a resulting vector.
*/
fn find_all_files(dir: &Path) -> Vec<PathBuf> {

    let mut v = vec![];

    // we only traverse directories
    if !dir.is_dir() {
        panic!("Directory {:?} is no directory, abort.",dir)
    }

    // only process valid directories
    if let Ok(files) = fs::read_dir(dir) {

        // process all files in directory, not recursive
        for file in files {

            //only process readabe files
            if let Ok(file) = file {

                // only add files with valid, reaqdable names
                if let Ok(name) = file.file_name().into_string(){

                    // only add supported jpeg files
                    if name.ends_with("JPG")
                        || name.ends_with("jpg")
                        || name.ends_with("jpeg"){

                        // add the file to the result vector
                        v.push(file.path());
                    }
                    else{
                        println!("Skipping {} due to wrong file ending.",name)
                    }
                }
                else{
                    println!("Illegal characters in file string.")
                }
            }
            else{
                println!("Cannot access file")
            }
        }
    }
    else{
        println!("Cannot read dir: {:?}",dir);
    }
    return v;
}

/**
    Load all images in the path vector.
    If one image fails to load, we abort.
*/
fn load_images(paths: Vec<PathBuf>) -> Vec<DynamicImage> {

    let count = paths.len();
    let mut counter = 1;

    let mut v = vec![];
    let mut dimension: Option<(u32,u32)>  = None;
    for path in paths{
        let img = image::open(&path);
        if let Ok(img) = img {

            // we only handle images with the same dimensions
            if let Some(dim) =dimension{
                let (x,y) = img.dimensions();
                if!(dim.0 == x && dim.1==y){
                    panic!("Images need to have the same dimenstion! Abortind due to {:?}",path);
                }
            }
            else{
                // in this case the dimenstions have not been set yet,
                // so we can use the ones from the first image
                dimension = Some(img.dimensions());
            }
            println!("[{}/{}]\tLoaded image: {:?}",counter,count,path);
            counter +=1;

            v.push(img);
        }
        else{
            panic!("Could not read image: {:?}",path);
        }
    }
    return v;
}