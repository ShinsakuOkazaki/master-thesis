extern crate rand;
extern crate bytes;
extern crate serde_json;
extern crate bincode;
extern crate avro_rs;
use avro_rs::{Schema, Writer, Reader, from_value};
use std::io::{BufRead, BufReader};
use std::fs::File;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::io::Result;
use serde::{Serialize, Deserialize};
use std::str;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let part_file: &str = args.get(1).unwrap();
    let size : usize = args[2].parse::<usize>().unwrap();

    let parts = create_part_vector(part_file, size);

    for i in 0..size {
        let part = &parts[i];

        let serialized_with_handcoded = part.write_byte_buffer();
        let deserialized_with_handcoded = part.read_byte_buffer(&serialized_with_handcoded);

        let serialized_with_json = part.serialize_to_json();
        let deserialized_with_json = part.deserialze_from_json(&serialized_with_json);
        
        let serialized_with_bincode = part.serialize_to_bincode();
        let deserialized_with_bincode = part.deserialized_from_bincode(&serialized_with_bincode[..]);

        let (serialized_with_avro, schema) = part.serialize_to_avro();
        let deserialized_with_avro = part.deserialize_from_avro(&serialized_with_avro[..], &schema);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    part_id: i32,
    name: String,
    mfgr: String,
    brand: String, 
    style: String,
    size: i32,
    container: String,
    retail_price: f64,
    comment: String,
}

impl Part {
    pub fn new(part_id: i32, name: String, mfgr: String, 
        brand: String, style: String, size: i32, 
        container: String, retail_price: f64, comment: String) -> Part {
        Part {
            part_id: part_id,
            name: name,
            mfgr: mfgr,
            brand: brand, 
            style: style,
            size: size,
            container: container,
            retail_price: retail_price,
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
        let temp_container = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        string_size = byte_buffer.get_i32();
        let temp_name = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        string_size = byte_buffer.get_i32();
        let temp_mfgr = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        string_size = byte_buffer.get_i32();
        let temp_brand = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        string_size = byte_buffer.get_i32();
        let temp_style = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        string_size = byte_buffer.get_i32();
        let temp_comment = extract_string(&mut byte_buffer, string_size as usize).unwrap();

        let object = Part::new(temp_part_id, temp_name, temp_mfgr, 
                               temp_brand, temp_style, temp_size, 
                               temp_container, temp_retail_price, temp_comment);
        Ok(object)
    }

    pub fn serialize_to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        serialized
        // let mut file = OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .open("part.json")
        //     .unwrap();
        // file.write_all(serialized.as_bytes()).expect("Fail to write file.");
    }

    pub fn deserialze_from_json(&self, serialized: &str) -> Part {
        // path= Path::new(&file_name);
        // let file = File::open(path).unwrap();
        // let buf_reader = BufReader::new(file);
        // let buffer = buf_reader.fill_buf().unwrap();
        // let json_string = str::from_utf8(buffer).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        deserialized
    }

    pub fn serialize_to_bincode(&self) -> Vec<u8> {
        let serialized = bincode::serialize(&self).unwrap();
        serialized
    }

    pub fn deserialized_from_bincode(&self, serialized: &[u8]) -> Part {
        let deserialized = bincode::deserialize(&serialized).unwrap();
        deserialized
    }

    pub fn serialize_to_avro(&self) -> (Vec<u8>, Schema) {
        let raw_schema = r#"
        {
            "type: "record,
            "name: "part",
            "fields": [
                {"name": "part_id", "type": "int"}, 
                {"name": "name", "type": "string"}, 
                {"name": "mfgr", "type": "string"}, 
                {"name": "brand", "type": "string"}, 
                {"name": "style", "type": "string"}, 
                {"name": "size", "type": "int"}, 
                {"name": "container", "type": "string"}, 
                {"name": "retail_price", "type": "float"}, 
                {"name": "comment", "type": "string"}, 
            ]
        }"#;

        let schema = Schema::parse_str(raw_schema).unwrap();
        let mut writer = Writer::new(&schema, Vec::new());
        writer.append_ser(self).unwrap();
        let serialized = writer.into_inner();
        (serialized, schema)
    }
    pub fn deserialize_from_avro(&self, input: &[u8], schema: &Schema) -> Vec<Part> {
        let reader = Reader::with_schema(&schema, input).unwrap();
        let mut deserialized = Vec::new();
        for value in reader {
            deserialized.push(from_value::<Part>(&value.unwrap()).unwrap());
        }
        deserialized
    }
}

fn extract_string(byte_buffer: &mut Bytes, size: usize) -> Result<String> {
    let mut dst: Vec<u8> = vec![0; size];
    byte_buffer.copy_to_slice(&mut dst[..]);
    let string = String::from_utf8(dst).unwrap();
    Ok(string)
}

fn create_part_vector(file_name: &str, size: usize) -> Vec<Part> {
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut parts = Vec::new();
    for (i, line) in lines.enumerate() {
        if i == size {
            break;
        }
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let part_id: i32 = row[0].parse::<i32>().unwrap();
        let name: String = row[1].to_string();
        let mfgr: String = row[2].to_string();
        let brand: String = row[3].to_string(); 
        let style: String = row[4].to_string();
        let size:i32 = row[5].parse::<i32>().unwrap();
        let container: String = row[6].to_string();
        let retail_price: f64 = row[7].parse::<f64>().unwrap();
        let comment: String = row[8].to_string();

        let part = Part::new(part_id, name, mfgr, brand, style, size, container, retail_price, comment);
        parts.push(part);
    }
    parts
}