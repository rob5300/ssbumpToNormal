use image::{io::Reader as ImageReader, ImageError};
use std::{env, io, path::Path, ffi::OsStr};
use cgmath::{Vector3, InnerSpace};

//From source 2013: https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/materialsystem/stdshaders/common_fxc.h#L41
const OO_SQRT_3: f32 = 0.57735025882720947f32;
static bumpBasisTranspose: [Vector3<f32>; 3] = [
    Vector3::new( 0.81649661064147949f32, -0.40824833512306213f32, -0.40824833512306213f32 ),
    Vector3::new(  0.0f32, 0.70710676908493042f32, -0.7071068286895752f32 ),
    Vector3::new(  OO_SQRT_3, OO_SQRT_3, OO_SQRT_3 )
]; 

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("[ssbump To Normal Converter by rob5300]\nhttps://github.com/rob5300");
        println!("Enter path of file to convert (or pass as launch arg):");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer);
        //Remove trailing new line chars
        if buffer.ends_with('\n') {
            buffer.pop();
            if buffer.ends_with('\r') {
                buffer.pop();
            }
        }
        buffer = buffer.replace("\"", "");
        //Check path is file
        let path = Path::new(&buffer);
        
        if path.is_file() {
            args.push(buffer);
        }
        else {
            println!("Provided path '{}' is not a file!", buffer);
            return;
        }
    }

    for x in 1..args.len() {
        let arg = &args[x];
        match ConvertImage(&arg) {
            Ok(result) => {
                if result {
                    println!("File {} converted successfully.", arg);
                }
                else {
                    println!("File {} failed to convert.", arg);
                }
            }
            Err(e) => println!("Img error for '{}': {}", arg, e)
        }
    }

    println!("Done!");
}

fn ConvertImage(path: &String) -> Result<bool, ImageError> {
    let mut img = ImageReader::open(path)?.decode()?;
    let width = img.width();
    let height = img.height();
    let rgb_img = img.as_mut_rgb8().expect("Failed to get rgb image");

    //Adjust pixels on image
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb_img.get_pixel_mut(x, y);
            //Convert pixel colour to a vector
            let pixel_vector = Vector3::new(pixel[0] as f32 / 255f32, pixel[1] as f32 / 255f32, pixel[2] as f32 / 255f32);
            //Convert normal vector back to traditional tangent normal
            pixel[0] = ConvertVector(&pixel_vector, 0);
            pixel[1] = ConvertVector(&pixel_vector, 1);
            pixel[2] = ConvertVector(&pixel_vector, 2);
        }
    }

    let originalPath = Path::new(path);
    //Add _normal into filename, hope there is a better way to do this
    let mut newFilePath = originalPath.to_owned();
    let oldName = originalPath.file_stem().unwrap().to_str().unwrap();
    let oldExt = originalPath.extension().unwrap().to_str().unwrap();
    let newFileName = [oldName, "_normal.", oldExt].concat();
    newFilePath.set_file_name(OsStr::new(&newFileName));
    img.save(&newFilePath);
    println!("Wrote new file to '{}'.", newFilePath.to_str().unwrap());

    return Ok(true);
}

fn ConvertVector(pixel: &Vector3<f32>, index: usize) -> u8 {
    return (((pixel.dot(bumpBasisTranspose[index]) * 0.5f32) + 0.5f32) * 255f32) as u8;
}