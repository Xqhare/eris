use std::{path::{Path, PathBuf}, fs::File, io::Read};

use json::{JsonValue, parse};

use crate::proc::Proc;

pub fn move_to_storage_file<P>(filename_to_move: P, filename_to_move_to: P) where P: AsRef<Path> + Copy, PathBuf: From<P> {
    // Filename_to_move exists, to_move_to is not proven yet. This is handled by File::create,
    // however the data I write to the file changes!
    if PathBuf::from(filename_to_move_to).exists() {
        let data_to_move_to = read_json(filename_to_move_to);
        let data_to_move = read_json(filename_to_move);
        let combinded_data = {
            let mut combinded_json = JsonValue::new_object();
            for entry in data_to_move_to.entries() {
                let _ = combinded_json.insert(entry.0, entry.1.clone());
            }
            for entry in data_to_move.entries() {
                let _ = combinded_json.insert(entry.0, entry.1.clone());
            }
            combinded_json
        };
        write_json(combinded_data, filename_to_move_to);
        // deleting contents in work file
        reset_work_file(filename_to_move);
    } else {
        let json_file = read_json(filename_to_move);
        write_json(json_file, filename_to_move_to);
        // deleting contents in work file
        reset_work_file(filename_to_move);
    }
}

pub fn write_state<P>(new_data: Vec<Proc>, filename: P) where P: AsRef<Path> + Copy, PathBuf: From<P> {
    let file_path = PathBuf::from(filename);
    if file_path.exists() {
        let old_data = read_json(filename);
        encode_write_json(old_data, new_data, filename);
    } else {
        let old_data = JsonValue::new_object();
        encode_write_json(old_data, new_data, filename);
    }
}

fn reset_work_file<P>(filename: P) where P: AsRef<Path> {
    let empty_data = JsonValue::new_object();
    write_json(empty_data, filename);
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
fn encode_write_json<P>(json_file: JsonValue, new_data: Vec<Proc>, filename: P) where P: AsRef<Path> {
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
        let vir_mem = entry.vir_mem;
        let total_disc_read = entry.total_disc_read;
        let total_disc_write = entry.total_disc_write;
        let run_time = entry.run_time;
        let usr_id = entry.usr_id;
        let new_json_data = json::object!{
            name: name,
            pid: pid,
            parent_name: parent_name,
            parent_pid: parent_pid,
            cpu_usage_percent: cpu_usage_per,
            date: date.clone(),
            vir_mem: vir_mem,
            total_disc_read: total_disc_read,
            total_disc_write: total_disc_write,
            run_time: run_time,
            usr_id: usr_id
        };
        let _ = json_obj_out.insert(format!("process {} at {}", pid, date).as_str(), new_json_data);
    }
    write_json(json_obj_out, filename);
}

fn write_json<P>(json_file: JsonValue, filename: P) where P: AsRef<Path> {
    let file = File::create(filename);
    if file.is_ok() {
        let _ = json_file.write_pretty(&mut file.expect("UNABLE TO CREATE FILE!"), 2);
    }
}
