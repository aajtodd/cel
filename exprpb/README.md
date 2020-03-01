
This is a wrapper crate for generating Rust code from the required CEL protobuf definitions. We are using rust-protobuf for the time being only because `prost` doesn't have support the introspection capabilities a CEL implementation will require. See https://github.com/danburkert/prost/issues/235 
