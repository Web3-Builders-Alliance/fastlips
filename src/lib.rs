use rayon::prelude::*;
use std::{fs::{File, self}, io::Read, path::Path, env, collections::HashMap,};
use serde::Deserialize;
use image::{io::Reader, DynamicImage, GenericImage, GenericImageView, Pixel};

#[derive(Debug, Deserialize, Clone)]
struct Attribute {
    value: String,
}

#[derive(Debug, Deserialize)]
struct TokenMetadata {
    edition: usize,
    attributes: Vec<Attribute>
}

fn generate(layer_path: Option<&str>, output_path: Option<&str>, metadata_path: Option<&str>) {
    let layer_path = match layer_path {
        Some(p) => p,
        None => "input/layers/"
    }.to_string();
    let output_path = match output_path {
        Some(p) => p,
        None => "output/"
    }.to_string();
    let metadata = match metadata_path {
        Some(p) => p,
        None => "input/metadata.json"
    }.to_string();

    let mut file = File::open(&metadata).unwrap();
    let mut file_buf: Vec<u8> = vec![];
    file.read_to_end(&mut file_buf).unwrap();

    // Load all images into memory
    let mut layers: HashMap<String, DynamicImage> = HashMap::new();
    fs::read_dir(&layer_path).unwrap().for_each(|r| {
        let f = r.unwrap().file_name().to_str().unwrap().to_string();
        let i: DynamicImage = Reader::open(format!("{}{}", layer_path, f)).unwrap().decode().expect("Opening image failed");
        layers.insert(f.split_once(".").unwrap().0.to_string(), i);
    });

    let tokens: Vec<TokenMetadata> = serde_json::from_slice(file_buf.as_slice()).expect("JSON was not well-formatted");
    tokens.par_iter().for_each(|t| {
        let images= t.attributes.clone().into_iter().map(|v| {
            layers.get(&v.value.to_string()).unwrap()
        }).collect::<Vec<&DynamicImage>>();
        let img = flatten_unchecked::<DynamicImage>(&images);
        img.save(format!("{}{}.png", output_path, &t.edition)).ok();
        println!("{:?}", t.edition);
    });
}

pub fn flatten_unchecked<I>(images: &Vec<&DynamicImage>) -> DynamicImage {
    let mut i = images[0].clone();
    let (width, height) = i.dimensions();

    for y in 0..height {
        for x in 0..width {
            let mut px = images[images.len()-1].get_pixel(x, y);
            for z in (0..images.len()-1).rev() {
                match px.channels()[3] {
                    255 => {
                        break;
                    },
                    0 => {
                        px = images[z].get_pixel(x, y)
                    },
                    _ => {
                        let mut p = images[z].get_pixel(x,y);
                        p.blend(&px);
                        px = p;
                    }
                }
            }
            i.put_pixel(x, y, px);
        }
    }
    return i
}

#[cfg(test)]
mod tests {
    use crate::generate;

    #[test]
    fn moon() {
        generate(None, None, None);
    }
}