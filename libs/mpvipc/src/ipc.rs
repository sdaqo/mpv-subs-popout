use super::*;
use log::{debug, warn};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Iterator;

#[derive(Debug)]
pub struct PlaylistEntry {
    pub id: usize,
    pub filename: String,
    pub title: String,
    pub current: bool,
}

pub trait TypeHandler: Sized {
    fn get_value(value: Value) -> Result<Self, Error>;
    fn as_string(&self) -> String;
}

impl TypeHandler for String {
    fn get_value(value: Value) -> Result<String, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::String(ref s) = map["data"] {
                        Ok(s.to_string())
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainString))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for bool {
    fn get_value(value: Value) -> Result<bool, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Bool(ref b) = map["data"] {
                        Ok(*b)
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainBool))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }
    fn as_string(&self) -> String {
        if *self {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

impl TypeHandler for f64 {
    fn get_value(value: Value) -> Result<f64, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Number(ref num) = map["data"] {
                        Ok(num.as_f64().unwrap())
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainF64))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for usize {
    fn get_value(value: Value) -> Result<usize, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Number(ref num) = map["data"] {
                        Ok(num.as_u64().unwrap() as usize)
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainUsize))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for HashMap<String, MpvDataType> {
    fn get_value(value: Value) -> Result<HashMap<String, MpvDataType>, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Object(ref inner_map) = map["data"] {
                        Ok(json_map_to_hashmap(inner_map))
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainHashMap))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl TypeHandler for Vec<PlaylistEntry> {
    fn get_value(value: Value) -> Result<Vec<PlaylistEntry>, Error> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Array(ref playlist_vec) = map["data"] {
                        Ok(json_array_to_playlist(playlist_vec))
                    } else {
                        Err(Error(ErrorCode::ValueDoesNotContainPlaylist))
                    }
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedValue))
            }
        } else {
            Err(Error(ErrorCode::UnexpectedValue))
        }
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn get_mpv_property<T: TypeHandler>(instance: &mut Mpv, property: &str) -> Result<T, Error> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);

    match serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string)) {
        Ok(val) => T::get_value(val),
        Err(why) => Err(Error(ErrorCode::JsonParseError(why.to_string()))),
    }
}

pub fn get_mpv_property_string(instance: &mut Mpv, property: &str) -> Result<String, Error> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);
    let val = serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string))
        .map_err(|why| Error(ErrorCode::JsonParseError(why.to_string())))?;

    let map = if let Value::Object(map) = val {
        Ok(map)
    } else {
        Err(Error(ErrorCode::UnexpectedValue))
    }?;

    let error = if let Value::String(ref error) = map["error"] {
        Ok(error)
    } else {
        Err(Error(ErrorCode::UnexpectedValue))
    }?;

    let data = if error == "success" {
        Ok(&map["data"])
    } else {
        Err(Error(ErrorCode::MpvError(error.to_string())))
    }?;

    match data {
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(ref n) => Ok(n.to_string()),
        Value::String(ref s) => Ok(s.to_string()),
        Value::Array(ref array) => Ok(format!("{:?}", array)),
        Value::Object(ref map) => Ok(format!("{:?}", map)),
        Value::Null => Err(Error(ErrorCode::MissingValue)),
    }
}

pub fn set_mpv_property<T: TypeHandler>(
    instance: &mut Mpv,
    property: &str,
    value: T,
) -> Result<(), Error> {
    let ipc_string = format!(
        "{{ \"command\": [\"set_property\", \"{}\", {}] }}\n",
        property,
        value.as_string()
    );
    match serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string)) {
        Ok(_) => Ok(()),
        Err(why) => Err(Error(ErrorCode::JsonParseError(why.to_string()))),
    }
}

pub fn run_mpv_command(instance: &mut Mpv, command: &str, args: &[&str]) -> Result<(), Error> {
    let mut ipc_string = format!(r#"{{ "command": ["{}""#, command);
    for arg in args {
        ipc_string.push_str(&format!(r#", "{}""#, arg));
    }
    ipc_string.push_str("] }\n");
    match serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string)) {
        Ok(feedback) => {
            if let Value::String(ref error) = feedback["error"] {
                if error == "success" {
                    Ok(())
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedResult))
            }
        }
        Err(why) => Err(Error(ErrorCode::JsonParseError(why.to_string()))),
    }
}

pub fn observe_mpv_property(instance: &mut Mpv, id: &isize, property: &str) -> Result<(), Error> {
    let ipc_string = format!(
        "{{ \"command\": [\"observe_property\", {}, \"{}\"] }}\n",
        id, property
    );
    match serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string)) {
        Ok(feedback) => {
            if let Value::String(ref error) = feedback["error"] {
                if error == "success" {
                    Ok(())
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedResult))
            }
        }
        Err(why) => Err(Error(ErrorCode::JsonParseError(why.to_string()))),
    }
}

pub fn unobserve_mpv_property(instance: &mut Mpv, id: &isize) -> Result<(), Error> {
    let ipc_string = format!("{{ \"command\": [\"unobserve_property\", {}] }}\n", id);
    match serde_json::from_str::<Value>(&send_command_sync(instance, &ipc_string)) {
        Ok(feedback) => {
            if let Value::String(ref error) = feedback["error"] {
                if error == "success" {
                    Ok(())
                } else {
                    Err(Error(ErrorCode::MpvError(error.to_string())))
                }
            } else {
                Err(Error(ErrorCode::UnexpectedResult))
            }
        }
        Err(why) => Err(Error(ErrorCode::JsonParseError(why.to_string()))),
    }
}

fn try_convert_property(name: &str, id: usize, data: MpvDataType) -> Event {
    let property = match name {
        "path" => match data {
            MpvDataType::String(value) => Property::Path(Some(value)),
            MpvDataType::Null => Property::Path(None),
            _ => unimplemented!(),
        },
        "pause" => match data {
            MpvDataType::Bool(value) => Property::Pause(value),
            _ => unimplemented!(),
        },
        "playback-time" => match data {
            MpvDataType::Double(value) => Property::PlaybackTime(Some(value)),
            MpvDataType::Null => Property::PlaybackTime(None),
            _ => unimplemented!(),
        },
        "duration" => match data {
            MpvDataType::Double(value) => Property::Duration(Some(value)),
            MpvDataType::Null => Property::Duration(None),
            _ => unimplemented!(),
        },
        "metadata" => match data {
            MpvDataType::HashMap(value) => Property::Metadata(Some(value)),
            MpvDataType::Null => Property::Metadata(None),
            _ => unimplemented!(),
        },
        _ => {
            warn!("Property {} not implemented", name);
            Property::Unknown {
                name: name.to_string(),
                data,
            }
        }
    };
    Event::PropertyChange { id, property }
}

pub fn listen(instance: &mut Mpv) -> Result<Event, Error> {
    let mut e;
    // sometimes we get responses unrelated to events, so we read a new line until we receive one
    // with an event field
    let name = loop {
        let mut reader = BufReader::new(&mut instance.stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        response = response.trim_end().to_string();
        debug!("Event: {}", response);

        e = serde_json::from_str::<Value>(&response)
            .map_err(|why| Error(ErrorCode::JsonParseError(why.to_string())))?;

        match e["event"] {
            Value::String(ref name) => break name,
            _ => {
                // It was not an event - try again
                debug!("Bad response: {:?}", response)
            }
        }
    };

    let event = match name.as_str() {
        "shutdown" => Event::Shutdown,
        "start-file" => Event::StartFile,
        "file-loaded" => Event::FileLoaded,
        "seek" => Event::Seek,
        "playback-restart" => Event::PlaybackRestart,
        "idle" => Event::Idle,
        "tick" => Event::Tick,
        "video-reconfig" => Event::VideoReconfig,
        "audio-reconfig" => Event::AudioReconfig,
        "tracks-changed" => Event::TracksChanged,
        "track-switched" => Event::TrackSwitched,
        "pause" => Event::Pause,
        "unpause" => Event::Unpause,
        "metadata-update" => Event::MetadataUpdate,
        "chapter-change" => Event::ChapterChange,
        "end-file" => Event::EndFile,
        "property-change" => {
            let name = match e["name"] {
                Value::String(ref n) => Ok(n.to_string()),
                _ => Err(Error(ErrorCode::JsonContainsUnexptectedType)),
            }?;

            let id: usize = match e["id"] {
                Value::Number(ref n) => n.as_u64().unwrap() as usize,
                _ => 0,
            };

            let data: MpvDataType = match e["data"] {
                Value::String(ref n) => MpvDataType::String(n.to_string()),

                Value::Array(ref a) => {
                    if name == "playlist".to_string() {
                        MpvDataType::Playlist(Playlist(json_array_to_playlist(a)))
                    } else {
                        MpvDataType::Array(json_array_to_vec(a))
                    }
                }

                Value::Bool(b) => MpvDataType::Bool(b),

                Value::Number(ref n) => {
                    if n.is_u64() {
                        MpvDataType::Usize(n.as_u64().unwrap() as usize)
                    } else if n.is_f64() {
                        MpvDataType::Double(n.as_f64().unwrap())
                    } else {
                        return Err(Error(ErrorCode::JsonContainsUnexptectedType));
                    }
                }

                Value::Object(ref m) => MpvDataType::HashMap(json_map_to_hashmap(m)),

                Value::Null => MpvDataType::Null,
            };

            try_convert_property(name.as_ref(), id, data)
        }
        _ => Event::Unimplemented,
    };
    Ok(event)
}

pub fn listen_raw(instance: &mut Mpv) -> String {
    let mut reader = BufReader::new(&mut instance.stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response.trim_end().to_string()
}

fn send_command_sync(instance: &mut Mpv, command: &str) -> String {
    let stream = &mut instance.stream;

    match stream.write_all(command.as_bytes()) {
        Err(why) => panic!("Error: Could not write to socket: {}", why),
        Ok(_) => {
            debug!("Command: {}", command.trim_end());
            let mut response = String::new();
            {
                let mut reader = BufReader::new(stream);
                while !response.contains("\"error\":") {
                    response.clear();
                    reader.read_line(&mut response).unwrap();
                }
            }
            debug!("Response: {}", response.trim_end());
            response
        }
    }
}

fn json_map_to_hashmap(map: &serde_json::map::Map<String, Value>) -> HashMap<String, MpvDataType> {
    let mut output_map: HashMap<String, MpvDataType> = HashMap::new();
    for (ref key, ref value) in map.iter() {
        match **value {
            Value::Array(ref array) => {
                output_map.insert(
                    key.to_string(),
                    MpvDataType::Array(json_array_to_vec(array)),
                );
            }
            Value::Bool(ref b) => {
                output_map.insert(key.to_string(), MpvDataType::Bool(*b));
            }
            Value::Number(ref n) => {
                if n.is_u64() {
                    output_map.insert(
                        key.to_string(),
                        MpvDataType::Usize(n.as_u64().unwrap() as usize),
                    );
                } else if n.is_f64() {
                    output_map.insert(key.to_string(), MpvDataType::Double(n.as_f64().unwrap()));
                } else {
                    panic!("unimplemented number");
                }
            }
            Value::String(ref s) => {
                output_map.insert(key.to_string(), MpvDataType::String(s.to_string()));
            }
            Value::Object(ref m) => {
                output_map.insert(
                    key.to_string(),
                    MpvDataType::HashMap(json_map_to_hashmap(m)),
                );
            }
            Value::Null => {
                unimplemented!();
            }
        }
    }
    output_map
}

fn json_array_to_vec(array: &Vec<Value>) -> Vec<MpvDataType> {
    let mut output: Vec<MpvDataType> = Vec::new();
    if array.len() > 0 {
        match array[0] {
            Value::Array(_) => {
                for entry in array {
                    if let Value::Array(ref a) = *entry {
                        output.push(MpvDataType::Array(json_array_to_vec(a)));
                    }
                }
            }

            Value::Bool(_) => {
                for entry in array {
                    if let Value::Bool(ref b) = *entry {
                        output.push(MpvDataType::Bool(*b));
                    }
                }
            }

            Value::Number(_) => {
                for entry in array {
                    if let Value::Number(ref n) = *entry {
                        if n.is_u64() {
                            output.push(MpvDataType::Usize(n.as_u64().unwrap() as usize));
                        } else if n.is_f64() {
                            output.push(MpvDataType::Double(n.as_f64().unwrap()));
                        } else {
                            panic!("unimplemented number");
                        }
                    }
                }
            }

            Value::Object(_) => {
                for entry in array {
                    if let Value::Object(ref map) = *entry {
                        output.push(MpvDataType::HashMap(json_map_to_hashmap(map)));
                    }
                }
            }

            Value::String(_) => {
                for entry in array {
                    if let Value::String(ref s) = *entry {
                        output.push(MpvDataType::String(s.to_string()));
                    }
                }
            }

            Value::Null => {
                unimplemented!();
            }
        }
    }
    output
}

fn json_array_to_playlist(array: &Vec<Value>) -> Vec<PlaylistEntry> {
    let mut output: Vec<PlaylistEntry> = Vec::new();
    for (id, entry) in array.iter().enumerate() {
        let mut filename: String = String::new();
        let mut title: String = String::new();
        let mut current: bool = false;
        if let Value::String(ref f) = entry["filename"] {
            filename = f.to_string();
        }
        if let Value::String(ref t) = entry["title"] {
            title = t.to_string();
        }
        if let Value::Bool(ref b) = entry["current"] {
            current = *b;
        }
        output.push(PlaylistEntry {
            id,
            filename,
            title,
            current,
        });
    }
    output
}
