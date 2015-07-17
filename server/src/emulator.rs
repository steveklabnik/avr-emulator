use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct Emulator {
    stack: String,
    registers: Vec<u8>,
}

pub fn serialize() -> String {
  let object = Emulator {
    stack: "homura".to_string(),
    registers: vec![2,3,4,5],
  };

  // Serialize using `json::encode`
  json::encode(&object).unwrap()
}

#[test]
fn it_works() {
  assert_eq!("{\"stack\":\"homura\",\"registers\":[2,3,4,5]}", serialize());
}
