use bit_vec::*;

use std::{io,env};
use std::path::Path;

use image;
use image::GenericImage;

pub trait GetBit { fn get_bit(&self, bit:u8) -> bool; }
impl GetBit for u8 { fn get_bit(&self, bit:u8) -> bool { (self >> 7-bit%8) & 1 == 1 } }

pub fn read(img_path:&Path)
{
    let img = image::open(img_path).unwrap();
    let rgb = img.as_rgb8().unwrap();
    let channel = 0; // 0:R,1:G,2:B;

    let bits = rgb.pixels().map(|rgb|rgb[channel]%2 == 1).take(128*8).collect::<BitVec<u8>>();
    println!("{:?}", String::from_utf8_lossy(&bits.to_bytes()));
}

pub fn write(img_path:&Path, out_path:&Path, message:&str) -> io::Result<()>
{
    let message_bits = BitVec::from_bytes(message.as_bytes());
    let mut img = image::open(img_path).unwrap();
    let mut rgb = img.as_mut_rgb8().unwrap();

    let channel = 0; // 0:R,1:G,2:B;

    for bp in message_bits.iter().zip(rgb.pixels_mut()){
        let (m,p) = bp;
        if m && p[channel]%2 == 0 {
            p[channel] += 1;
        }
        else if !m && p[channel]%2 == 1 {
            p[channel] += 1;
        }
    }

    image::save_buffer(out_path,
                       rgb,
                       rgb.width(), rgb.height(),
                       image::ColorType::RGB(8))
}
