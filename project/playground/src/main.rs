extern crate rand;
extern crate bytes;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::io::Result;
use serde::{Serializem, Deserialize};

fn main() -> Result<()> {
    let part_id = 14;
    let size = 30;
    let retail_price = 3.14;
    let container = String::from("Basic");
    let name = String::from("Okazaki");
    let mfgr = String::from("MFGR");
    let brand = String::from("Nike");
    let style = String::from("Standard");
    let comment = String::from("Good");

    let before_part = Part::new(part_id, size, retail_price, container, name, mfgr, brand, style, comment);
    
    let in_buf = before_part.write_byte_buffer();
    {
        let mut file = File::create("buffer.txt")?;
        file.write(in_buf.bytes())?;
    }
    let file = File::open("buffer.txt")?;
    let mut reader = BufReader::new(file);
    let out_buf = reader.fill_buf().unwrap();
    assert_eq!(in_buf, out_buf);
    let after_part = before_part.read_byte_buffer(out_buf)?;
    println!("part_id: {}, size: {}, retail_price: {}, 
                container: {:?}, name: {:?}, mfgr: {:?}, 
                brand: {:?}, style: {:?}, comment: {:?}",
                after_part.part_id, after_part.size, after_part.retail_price, 
                after_part.container, after_part.name, after_part.mfgr, 
                after_part.brand, after_part.style, after_part.comment);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    part_id: i32,
    size: i32,
    retail_price: f64,

    container: String,
    name: String,
    mfgr: String,
    brand: String, 
    style: String,
    comment: String,
}

impl Part {
    pub fn new(part_id: i32, size: i32, retail_price: f64,
                container: String, name: String, mfgr: String, 
                brand: String, style: String, comment: String) -> Part {
        Part {
            part_id: part_id,
            size: size,
            retail_price: retail_price,
            container: container,
            name: name,
            mfgr: mfgr,
            brand: brand, 
            style: style,
            comment: comment,
        }
    }

    pub fn write_byte_buffer(&self) -> Bytes {
        let container_bytes = self.container.as_bytes();
        let name_bytes = self.name.as_bytes();
        let mfgr_bytes = self.mfgr.as_bytes();
        let brand_bytes = self.brand.as_bytes();
        let style_bytes = self.style.as_bytes();
        let comment_bytes = self.comment.as_bytes();
        
        let mut byte_buffer = BytesMut::with_capacity(40 + container_bytes.len() + name_bytes.len()
                                                         + mfgr_bytes.len() + brand_bytes.len()
                                                         + style_bytes.len() + comment_bytes.len());
        byte_buffer.put_i32(self.part_id);
        byte_buffer.put_i32(self.size);
        byte_buffer.put_f64(self.retail_price);

        byte_buffer.put_i32(container_bytes.len() as i32);
        byte_buffer.put_slice(container_bytes);

        byte_buffer.put_i32(name_bytes.len() as i32);
        byte_buffer.put_slice(name_bytes);

        byte_buffer.put_i32(mfgr_bytes.len() as i32);
        byte_buffer.put_slice(mfgr_bytes);

        byte_buffer.put_i32(brand_bytes.len() as i32);
        byte_buffer.put_slice(brand_bytes);

        byte_buffer.put_i32(style_bytes.len() as i32);
        byte_buffer.put_slice(style_bytes);

        byte_buffer.put_i32(comment_bytes.len() as i32);
        byte_buffer.put_slice(comment_bytes);

        return byte_buffer.freeze()
    }

    pub fn read_byte_buffer(& self, buf: &[u8]) -> Result<Part> {
        let mut byte_buffer_init = BytesMut::with_capacity(buf.len());
        byte_buffer_init.extend_from_slice(buf);
        let mut byte_buffer = byte_buffer_init.freeze();

        let temp_part_id = byte_buffer.get_i32();
        let temp_size = byte_buffer.get_i32();
        let temp_retail_price = byte_buffer.get_f64();

        let mut string_size;

        string_size = byte_buffer.get_i32();
        let temp_container = extract_string(&mut byte_buffer, string_size as usize)?;

        string_size = byte_buffer.get_i32();
        let temp_name = extract_string(&mut byte_buffer, string_size as usize)?;

        string_size = byte_buffer.get_i32();
        let temp_mfgr = extract_string(&mut byte_buffer, string_size as usize)?;

        string_size = byte_buffer.get_i32();
        let temp_brand = extract_string(&mut byte_buffer, string_size as usize)?;

        string_size = byte_buffer.get_i32();
        let temp_style = extract_string(&mut byte_buffer, string_size as usize)?;

        string_size = byte_buffer.get_i32();
        let temp_comment = extract_string(&mut byte_buffer, string_size as usize)?;

        let object = Part::new(temp_part_id, temp_size, temp_retail_price, 
                    temp_container, temp_name , temp_mfgr, 
                    temp_brand, temp_style , temp_comment);
        Ok(object)
    }
}

fn extract_string(byte_buffer: &mut Bytes, size: usize) -> Result<String> {
    let mut dst: Vec<u8> = vec![0; size];
    byte_buffer.copy_to_slice(&mut dst[..]);
    let string = String::from_utf8(dst).unwrap();
    Ok(string)
}
