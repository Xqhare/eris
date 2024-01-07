use std::{path::{Path, PathBuf}, fs::File, io::Read};

use json::{JsonValue, parse};

use crate::proc::Proc;

pub fn write_state<P>(new_data: Vec<Proc>, filename: P) where P: AsRef<Path> + Copy, PathBuf: From<P> {
    let file_path = PathBuf::from(filename);
    if file_path.exists() {
        let old_data = read_json(filename);
        write_json(old_data, new_data, filename);
    } else {
        let old_data = JsonValue::new_object();
        write_json(old_data, new_data, filename);
    }
}

/// This cannot panic, it will return an empty `JsonValue` instead.
fn read_json<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let input = File::open(filename);
    if input.is_ok() {
        let mut buffer: String = Default::default();
        let _ = input.unwrap().read_to_string(&mut buffer);
        let out = parse(&buffer);
        if out.is_ok() {
            return out.unwrap();
        } else {
            return JsonValue::new_object();
        }
    } else {
        return JsonValue::new_object();
    }
}
/// This function cannot panic, it will just not write anything if it does.
fn write_json<P>(json_file: JsonValue, new_data: Vec<Proc>, filename: P) where P: AsRef<Path> {
    let mut json_obj_out = JsonValue::new_object();
    for value in json_file.entries() {
        let _ = json_obj_out.insert(value.0, value.1.clone());
    }
    for entry in new_data {
        let name = entry.name;
        let pid = entry.pid.as_u32();
        let parent_name = entry.parent_name;
        let parent_pid = entry.parent_pid.as_u32();
        let cpu_usage_per = entry.cpu_usage_per;
        let date = entry.date;
        let new_json_data = json::object!{
            name: name,
            pid: pid,
            parent_name: parent_name,
            parent_pid: parent_pid,
            cpu_usage_percent: cpu_usage_per,
            date: date.clone()
        };
        let _ = json_obj_out.insert(format!("process {} at {}", pid, date).as_str(), new_json_data);
    }
    let file = File::create(filename);
    if file.is_ok() {
        let _ = json_obj_out.write(&mut file.expect("UNABLE TO CREATE FILE!"));
    }
}
