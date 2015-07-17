use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct Emulator {
    dataSpace: String,
    registerFile: Vec<u8>,
    ioMemory: Vec<u8>,
    program: Vec<u8>,
    stack: u16,
    stackPointer: u16
}

pub fn serialize() -> String {
  let object = Emulator {
    stack: "homura".to_string(),
    registers: vec![2,3,4,5],
  };

  // Serialize using `json::encode`
  json::encode(&object).unwrap()
}
//#[test]
//fn can_add() {
  //let emulator = Emulator {
    //registerFile: vec![0,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
  //};
  //let operation = "add r1,r2";
  //let result = add(emulator, "r1,r2");
  //assert_eq!(5, emulator.registerFile[1]);
  //assert_eq!(3, emulator.registerFile[2]);
//}

#[test]
fn it_works() {
  assert_eq!("{\"stack\":\"homura\",\"registers\":[2,3,4,5]}", serialize());
}
