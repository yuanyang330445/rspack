use rspack_cacheable::{from_bytes, to_bytes};

#[derive(
  rspack_cacheable::__private::rkyv::Archive,
  rspack_cacheable::__private::rkyv::Deserialize,
  rspack_cacheable::__private::rkyv::Serialize,
)]
#[rkyv(crate=rspack_cacheable::__private::rkyv)]
struct Module {
  name: String,
}

#[test]
fn test_context() {
  let module = Module {
    name: String::from("a"),
  };

  let bytes = to_bytes(&module, &()).unwrap();
  println!("bytes {:?}", bytes);
  let bytes1: &[u8] = &[97, 255, 255, 255, 255, 255, 255, 255];
  /*  if &bytes == bytes1 {
    panic!("eq");
  } else {
    panic!("not eq");
  }*/
  //  let bytes = rspack_cacheable::__private::rkyv::to_bytes::<SerializeError>(&module).unwrap();
  //  println!("bytes {:?}", bytes.);
  from_bytes::<Module, ()>(&bytes, &()).unwrap();
  //  rspack_cacheable::__private::rkyv::from_bytes::<Module, DeserializeError>(&bytes).unwrap();
  //    assert!()
  //  assert_eq!(module, new_module);
}
