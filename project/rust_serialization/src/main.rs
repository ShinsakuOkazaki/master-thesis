extern crate rand;
extern crate bytes;
extern crate serde_json;
extern crate bincode;
extern crate avro_rs;
use avro_rs::{Schema, Writer, Reader, from_value};
use std::io::{BufRead, BufReader};
use bytes::{Bytes, BytesMut, Buf, BufMut};
use serde::{Serialize, Deserialize};
use std::str;
use std::env;
use std::path::Path;
use std::time::Instant;
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let part_file: &str = args.get(1).unwrap();
    let size : usize = args[2].parse::<usize>().unwrap();
    let method: i32 = args[3].parse::<i32>().unwrap();
    run_experiment(part_file, size, method);
}

fn run_experiment(part_file: &str, size: usize, method: i32) {
    match method {
        1 => run_handcoded(part_file, size), 
        2 => run_json(part_file, size),
        3 => run_bincode(part_file, size),
        4 => run_avro(part_file, size), 
        _ => println!("Wrong input!!"), 
    }
}

fn run_handcoded(part_file: &str, size: usize) {
    let t_serialize = do_serialize_handcoded(part_file, size);
    let t_deserialize = do_deserialize_handcoded("serialized_with_handcoded.txt");
    write_to_file("handcoded", t_serialize, t_deserialize);
}

fn run_json(part_file: &str, size: usize) {
    let t_serialize = do_serialize_json(part_file, size);
    let t_deserialize = do_deserialize_json("serialized_with_json.txt");
    write_to_file("json", t_serialize, t_deserialize);
}

fn run_bincode(part_file: &str, size: usize) {
    let t_serialize = do_serialize_bincode(part_file, size);
    let t_deserialize = do_deserialize_bincode("serialized_with_bincode.txt");
    write_to_file("bincode", t_serialize, t_deserialize);
}

fn run_avro(part_file: &str, size: usize) {
    let t_serialize = do_serialize_avro(part_file, size);
    let t_deserialize = do_deserialize_avro("serialized_with_avro.txt");
    write_to_file("avro", t_serialize, t_deserialize);
}


fn write_to_file(method: &str, t_serialize: u128, t_deserialize: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}\n", method, t_serialize, t_deserialize);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();
    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

fn write_serialized(serialized: &[u8], method: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("serialized_with_{}.txt", method))
        .unwrap();
     file.write_all(&serialized[..]).unwrap();
}

fn read_serialized(file_name: &str) -> Vec<u8> {
    let path= Path::new(&file_name);
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    let n= file.read_to_end(&mut buffer);
    buffer
}

fn do_serialize_handcoded(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let start = Instant::now();
    let serialized = Part::write_byte_buffer_for_vec(&parts[..]);
    write_serialized(serialized.as_ref(), "handcoded");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_handcoded(file_name: &str) -> u128 {
    let serialized = read_serialized(file_name);
    let start = Instant::now();
    let deserialized = Part::read_byte_buffer_for_vec(&serialized[..]);
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_json(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let start = Instant::now();
    let serialized = Part::serialize_to_json_for_vec(&parts[..]);
    write_serialized(serialized.as_bytes(), "json");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_json(file_name: &str) -> u128 {
    let serialized = read_serialized(file_name);
    let start = Instant::now();
    let deserialized = Part::deserialize_from_json_for_vec(&str::from_utf8(&serialized[..]).unwrap());
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_bincode(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let start = Instant::now();
    let serialized = Part::serialize_to_bincode_for_vec(&parts[..]);
    write_serialized(&serialized[..], "bincode");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_bincode(file_name: &str) -> u128 {
    let serialized = read_serialized(file_name);
    let start = Instant::now();
    let deserialized = Part::deserialized_from_bincode_for_vec(&serialized[..]);
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_avro(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let start = Instant::now();
    let serialized = Part::serialize_to_avro_for_vec(&parts[..]);
    write_serialized(&serialized[..], "avro");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_avro(file_name: &str) -> u128 {
    let serialized = read_serialized(file_name);
    let start = Instant::now();
    let deserialized = Part::deserialize_from_avro(&serialized[..]);
    let end = start.elapsed().as_millis();
    end
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

    pub fn get_schema() -> Schema {
        let raw_schema = r#"
        {
            "type": "record",
            "name": "part",
            "fields": [
                {"name": "part_id", "type": "int"}, 
                {"name": "name", "type": "string"}, 
                {"name": "mfgr", "type": "string"}, 
                {"name": "brand", "type": "string"}, 
                {"name": "style", "type": "string"}, 
                {"name": "size", "type": "int"}, 
                {"name": "container", "type": "string"}, 
                {"name": "retail_price", "type": "double"}, 
                {"name": "comment", "type": "string"} 
            ]
        }"#;

        let schema = Schema::parse_str(raw_schema).unwrap();
        schema
    }

    pub fn write_byte_buffer(&self) -> Bytes {
        let container_bytes = self.container.as_bytes();
        // println!("Container bytes: {:?}", container_bytes);
        let name_bytes = self.name.as_bytes();
        // println!("Name bytes: {:?}", name_bytes);
        let mfgr_bytes = self.mfgr.as_bytes();
        // println!("Mfgr bytes: {:?}", mfgr_bytes);
        let brand_bytes = self.brand.as_bytes();
        // println!("Brand bytes: {:?}", brand_bytes);
        let style_bytes = self.style.as_bytes();
        // println!("Style bytes: {:?}", style_bytes);
        let comment_bytes = self.comment.as_bytes();
        // println!("Comment bytes: {:?}", comment_bytes);
        
        let mut byte_buffer = BytesMut::with_capacity(40 + container_bytes.len() + name_bytes.len()
                                                         + mfgr_bytes.len() + brand_bytes.len()
                                                         + style_bytes.len() + comment_bytes.len());
        byte_buffer.put_i32(self.part_id);
        byte_buffer.put_i32(self.size);
        byte_buffer.put_f64(self.retail_price);

        byte_buffer.put_i32(container_bytes.len() as i32);
        byte_buffer.put(container_bytes);

        byte_buffer.put_i32(name_bytes.len() as i32);
        byte_buffer.put(name_bytes);

        byte_buffer.put_i32(mfgr_bytes.len() as i32);
        byte_buffer.put(mfgr_bytes);

        byte_buffer.put_i32(brand_bytes.len() as i32);
        byte_buffer.put(brand_bytes);

        byte_buffer.put_i32(style_bytes.len() as i32);
        byte_buffer.put(style_bytes);

        byte_buffer.put_i32(comment_bytes.len() as i32);
        byte_buffer.put(comment_bytes);

        //println!("Byte: {:?}", byte_buffer);
        return byte_buffer.freeze()
    }

    pub fn write_byte_buffer_for_vec(parts: &[Part]) -> Bytes {
        let mut byte_buffer = BytesMut::new();
        let size = parts.len();
        for i in 0..size {
            let part = &parts[i];
            let temp_buf = part.write_byte_buffer();
            byte_buffer.put(&temp_buf[..]);
        }

        return byte_buffer.freeze();
    }

    pub fn read_byte_buffer(buf: &[u8]) -> std::io::Result<Part> {
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

    pub fn read_byte_buffer_for_vec(buf: &[u8]) -> std::io::Result<Vec<Part>> {
        let mut byte_buffer_init = BytesMut::with_capacity(buf.len());
        byte_buffer_init.extend_from_slice(buf);
        let mut byte_buffer = byte_buffer_init.freeze();
        let mut parts: Vec<Part> = Vec::new();

        while byte_buffer.has_remaining() {
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
            
            parts.push(object);
        }

        Ok(parts)
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

    pub fn serialize_to_json_for_vec(parts :&[Part]) -> String {
        let serialized = serde_json::to_string(parts).unwrap();
        serialized
    }

    pub fn deserialze_from_json(serialized: &str) -> Part {
        // path= Path::new(&file_name);
        // let file = File::open(path).unwrap();
        // let buf_reader = BufReader::new(file);
        // let buffer = buf_reader.fill_buf().unwrap();
        // let json_string = str::from_utf8(buffer).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        deserialized
    }

    pub fn deserialize_from_json_for_vec(serialized: &str) -> Vec<Part> {
        let deserialized = serde_json::from_str(&serialized).unwrap();
        deserialized 
    }

    pub fn serialize_to_bincode(&self) -> Vec<u8> {
        let serialized = bincode::serialize(&self).unwrap();
        serialized
    }

    pub fn serialize_to_bincode_for_vec(parts: &[Part]) -> Vec<u8>{
        let serialized = bincode::serialize(parts).unwrap();
        serialized
    }

    pub fn deserialized_from_bincode(serialized: &[u8]) -> Part {
        let deserialized = bincode::deserialize(&serialized).unwrap();
        deserialized
    }

    pub fn deserialized_from_bincode_for_vec(serialized: &[u8]) -> Vec<Part> {
        let deserialized = bincode::deserialize(&serialized).unwrap();
        deserialized 
    }

    pub fn serialize_to_avro(&self) -> Vec<u8> {
        let schema = Part::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        writer.append_ser(self).unwrap();
        let serialized = writer.into_inner();
        serialized
    }

    pub fn serialize_to_avro_for_vec(parts: &[Part]) -> Vec<u8>{
        let schema = Part::get_schema();
        let mut writer = Writer::new(&schema, Vec::new());
        for part in parts {
            writer.append_ser(part).unwrap();
        }
        let serialized = writer.into_inner();
        serialized
    }

    pub fn deserialize_from_avro(input: &[u8]) -> Part {
        let schema = Part::get_schema();
        let reader = Reader::with_schema(&schema, input).unwrap();
        let mut deserialized = Vec::new();
        for value in reader {
            deserialized.push(from_value::<Part>(&value.unwrap()).unwrap());
        }
        deserialized.pop().unwrap()
    }

    pub fn deserialize_from_avro_for_vec(input: &[u8]) -> Vec<Part> {
        let schema = Part::get_schema();
        let reader = Reader::with_schema(&schema, input).unwrap();
        let mut deserialized = Vec::new();
        for value in reader {
            deserialized.push(from_value::<Part>(&value.unwrap()).unwrap());
        }
        deserialized
    }
}

fn extract_string(byte_buffer: &mut Bytes, size: usize) -> std::io::Result<String> {
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