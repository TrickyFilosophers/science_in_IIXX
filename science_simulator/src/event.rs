use std::path::Path;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum Operation {
    Pause,
    CreateDir,
    AppendFile,
    AppendMany,
    DeleteFile,
    DeleteMany,
    Commit
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Event {
     pub operation: Operation,
     pub path: Option<String>,
     pub name: Option<String>,
     pub branch: Option<String>,
     pub msg: Option<String>,
     pub repeat: Option<i32>,
     pub from: Option<i32>,
     pub to: Option<i32>
}


