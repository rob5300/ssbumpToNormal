use image::{io::Reader as ImageReader, GenericImageView, RgbImage};
use std::{env, io::{self, ErrorKind}, path::Path, ffi::OsStr, error::Error};
use cgmath::{Vector3, InnerSpace};

//From source 2013: https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/materialsystem/stdshaders/common_fxc.h#L41
const OO_SQRT_3: f32 = 0.57735025882720947f32;
static BUMP_BASIS_TRANSPOSE: [Vector3<f32>; 3] = [
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
        match stdin.read_line(&mut buffer) {
            Err(e) => println!("Input not valid"),
            Ok(_) => (),
        }
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
        println!("Starting conversion of '{}'...", arg);
        match convert_image(&arg) {
            Ok(()) => println!("File '{}' converted successfully.", arg),
            Err(e) => println!("Image conversion error for '{}': {}", arg, e)
        }
    }

    println!("Done!");
}

fn path_error() -> io::Error
{
    io::Error::new(ErrorKind::Other, "Path of image was malformed or contained invalid characters")
}

fn convert_image(path: &String) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let width = img.width();
    let height = img.height();
    let pixels = img.pixels();
    let mut new_image = RgbImage::new(width, height);

    //Adjust pixels on image
    for pixel in pixels {
        //Convert pixel colour to a vector
        let rgb = pixel.2;
        let pixel_vector = Vector3::new(rgb[0] as f32 / 255f32, rgb[1] as f32 / 255f32, rgb[2] as f32 / 255f32);
        //Convert normal vector back to traditional tangent normal
        let new_rgb = new_image.get_pixel_mut(pixel.0, pixel.1);
        new_rgb[0] = convert_vector(&pixel_vector, 0);
        new_rgb[1] = convert_vector(&pixel_vector, 1);
        new_rgb[2] = convert_vector(&pixel_vector, 2);
    }

    let original_path = Path::new(path);
    //Add _normal into filename
    let mut new_file_path = original_path.to_owned();
    let old_name = original_path.file_stem().ok_or_else(path_error)?.to_str().ok_or_else(path_error)?;
    let new_file_name = [old_name, "_normal.", "png"].concat();
    new_file_path.set_file_name(OsStr::new(&new_file_name));

    //Save new image to disk
    if new_file_path.exists() {
        println!("Overriding existing image...");
    }
    new_image.save_with_format(&new_file_path, image::ImageFormat::Png)?;
    println!("Wrote converted normal map to '{}'.", new_file_path.to_str().ok_or_else(path_error)?);
    Ok(())
}

fn convert_vector(pixel: &Vector3<f32>, index: usize) -> u8 {
    return (((pixel.dot(BUMP_BASIS_TRANSPOSE[index]) * 0.5f32) + 0.5f32) * 255f32) as u8;
}