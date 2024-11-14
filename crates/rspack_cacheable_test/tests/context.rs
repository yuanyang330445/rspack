use rspack_cacheable::{cacheable, from_bytes, to_bytes};

#[cacheable]
#[derive(Debug, PartialEq, Eq)]
struct Module {
  name: String,
}

#[test]
fn test_context() {
  let module = Module {
    name: String::from("a"),
  };

  let bytes = to_bytes(&module, &()).unwrap();

  let new_module: Module = from_bytes(&bytes, &()).unwrap();
  assert_eq!(module, new_module);
}
