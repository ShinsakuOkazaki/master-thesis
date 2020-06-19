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
use std::io::LineWriter;
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

fn write_serialized(vec_serialized: Vec<Vec<u8>>, method: &str) {
    let file = File::create(format!("serialized_with_{}.txt", method)).unwrap();
    let mut file = LineWriter::new(file);
    for serialized in vec_serialized {
        file.write_all(&serialized[..]).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn read_serialized(file_name: &str) -> Vec<String> {
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().collect::<Result<Vec<String>, std::io::Error>>().unwrap();
    lines
}

fn do_serialize_handcoded(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let mut vec_serialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let part = &parts[i];
        let serialized_with_handcoded = part.write_byte_buffer();
        vec_serialized.push(serialized_with_handcoded.as_ref().to_vec());
    }
    write_serialized(vec_serialized, "handcoded");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_handcoded(file_name: &str) -> u128 {
    let vec_serialized = read_serialized(file_name);
    let size = vec_serialized.len();
    let mut vec_deserialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let deserialized_with_handcoded = Part::read_byte_buffer(&vec_serialized[i].as_bytes());
        vec_deserialized.push(deserialized_with_handcoded);
    }
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_json(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let mut vec_serialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let part = &parts[i];
        let serialized_with_json = part.serialize_to_json();
        vec_serialized.push(serialized_with_json.into_bytes());
    }
    write_serialized(vec_serialized, "json");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_json(file_name: &str) -> u128 {
    let vec_serialized = read_serialized(file_name);
    let size = vec_serialized.len();
    let mut vec_deserialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let deserialized_with_json = Part::deserialze_from_json(&vec_serialized[i]);
        vec_deserialized.push(deserialized_with_json);
    }
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_bincode(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let mut vec_serialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let part = &parts[i];
        let serialized_with_bincode = part.serialize_to_bincode();
        vec_serialized.push(serialized_with_bincode);
    }
    write_serialized(vec_serialized, "bincode");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_bincode(file_name: &str) -> u128 {
    let vec_serialized = read_serialized(file_name);
    let size = vec_serialized.len();
    let mut vec_deserialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let deserialized_with_bincode = Part::deserialized_from_bincode(&vec_serialized[i].as_bytes());
        vec_deserialized.push(deserialized_with_bincode);
    }
    let end = start.elapsed().as_millis();
    end
}

fn do_serialize_avro(part_file: &str, size: usize) -> u128 {
    let parts = create_part_vector(part_file, size);
    let mut vec_serialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let part = &parts[i];
        let serialized_with_avro = part.serialize_to_avro();
        vec_serialized.push(serialized_with_avro);
    }
    write_serialized(vec_serialized, "avro");
    let end = start.elapsed().as_millis();
    end
}

fn do_deserialize_avro(file_name: &str) -> u128 {
    let vec_serialized = read_serialized(file_name);
    let size = vec_serialized.len();
    let mut vec_deserialized = Vec::with_capacity(size);
    let start = Instant::now();
    for i in 0..size {
        let deserialized_with_avro = Part::deserialize_from_avro(&vec_serialized[i].as_bytes(),);
        vec_deserialized.push(deserialized_with_avro);
    }
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
        schema
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

    pub fn deserialze_from_json(serialized: &str) -> Part {
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

    pub fn deserialized_from_bincode(serialized: &[u8]) -> Part {
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
    pub fn deserialize_from_avro(input: &[u8]) -> Vec<Part> {
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