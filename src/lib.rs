extern crate rustc_serialize;

use rustc_serialize::json;
use rustc_serialize::Encodable;
use rustc_serialize::Encoder;

#[derive(RustcEncodable)]
struct BuildDetailAttributes {
    name: &'static str,
    version: &'static str,
    homepage: &'static str,
    support: &'static str,
    description: &'static str
}

struct BuildBody {
    _type: String,
    id: i16,
    attributes: BuildDetailAttributes
}
impl Encodable for BuildDetail {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match * self {
            BuildDetail { _type: ref p_type, id: ref p_id, attributes: ref p_attributes } =>
                encoder.emit_struct("BuildDetail", 3usize, |enc| -> _ {
                    try!(enc.emit_struct_field( "type", 0usize, |enc| p_type.encode(enc)));
                    try!(enc.emit_struct_field( "id", 1usize, |enc| p_id.encode(enc)));
                    return enc.emit_struct_field("attributes", 2usize, |enc| -> _ { (* p_attributes).encode(enc) });
                }),
        }
    }
}

#[derive(RustcEncodable)]
struct Build {
    data: Vec<BuildBOdy>
}

fn get_build_attributes() -> BuildDetailAttributes {
    return BuildDetailAttributes {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        homepage: env!("CARGO_PKG_HOMEPAGE"),
        support: env!("CARGO_PKG_AUTHORS"),
        description: env!("CARGO_PKG_DESCRIPTION")
    };
}

fn get_build_body(attributes : BuildDetailAttributes) -> BuildDetail {
    return BuildBody {
        _type: "build".to_string(),
        id: 1,
        attributes: attributes
    };
}

fn get_build(body : BuildBody ) -> Build {
    return  Build {
        data: vec![body]
    };
}

pub mod fn build_as_json() -> String {
    let attributes = get_build_attributes();
    let body = get_build_body(attributes);
    let json = get_build(body);
    let payload = json::encode(&json).unwrap();

    return payload;
}

// Lets deliver an HTML template that request JSON.
// For now just deliver a String.
fn build_as_html() -> String {
    return "build statement goes here".to_string();
}
