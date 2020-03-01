extern crate protoc_rust;
use protoc_rust::Customize;

use std::{env, fs, io};
use std::path::PathBuf;

fn protos() -> Result<Vec<String>, io::Error> {
    let curr_dir = env::current_dir()?;
    let prefix = PathBuf::from("googleapi-expr/google/api/expr/v1alpha1");
    let proto_dir = curr_dir.join(prefix.clone());

    let mut pb_files = Vec::new();
    for entry in fs::read_dir(proto_dir)? {
        let entry = entry?;
        let p = entry.file_name().into_string().expect("failed to convert OsString for proto path");
        if !p.ends_with(".proto") {
            continue;
        }
        let p = prefix.join(p);
        pb_files.push(p.into_os_string().into_string().unwrap());        
    }

    Ok(pb_files)
}

fn main() {
    let pb_files = protos().expect("failed to gather protos");
    let mut pb_files: Vec<&str> = pb_files.iter().map(AsRef::as_ref).collect();
    pb_files.push("googleapi-expr/google/rpc/status.proto");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        // full paths (googleapi-expr/google/api/expr/v1alpha1/blah.proto")
        input: pb_files.as_slice(),
        // NOTE: include path needs to be the root of whatever the IDL (.proto) files reference. In
        // this case the imports look like googe/blah/blah. Which is why the googleapi-expr repo
        // pulls in that entire path. This is a protobuf specific import behavior.
        includes: &["googleapi-expr"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
