//! Regression tests proving that schema-defined names are never special-cased.

use mono_tl_codegen::{Options, generate};
use mono_tl_schema::Source;

fn generate_schema(input: &str, options: &Options) -> String {
    let schema = mono_tl_schema::parse(input, Source::default()).unwrap();
    generate(&schema, options).unwrap()
}

fn assert_compiles(code: &str) {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("generated.rs");
    std::fs::write(&source, code).unwrap();
    let output = std::process::Command::new("rustc")
        .arg("--edition=2024")
        .arg("--crate-type=lib")
        .arg("--out-dir")
        .arg(directory.path())
        .arg(source)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Generated Rust did not compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn discovers_unfamiliar_types_and_generic_applications() {
    let code = generate_schema(
        "octet value:int = Octet;\
         sequence {element:Type} count:# values:count*[element] = Sequence element;\
         envelope payload:Sequence<Octet> = Envelope;",
        &Options::default(),
    );

    assert!(code.contains("pub enum Octet"));
    assert!(code.contains("pub enum Sequence<Element>"));
    assert!(code.contains("pub values: Vec<Element>"));
    assert!(code.contains("pub payload: Box<Sequence<Box<Octet>>>"));
    assert!(!code.contains("Vector"));
    assert!(!code.contains("Int128"));
    assert_compiles(&code);
}

#[test]
fn primitive_representation_is_caller_policy() {
    let mut options = Options::default();
    options
        .type_mappings
        .insert("int".into(), "crate::wire::SignedWord".into());
    let code = generate_schema("word value:int = Word;", &options);
    assert!(code.contains("pub value: crate::wire::SignedWord"));
}

#[test]
fn natural_expressions_drive_repetition_shape() {
    let code = generate_schema(
        "fixed values:4*[ item:int ] = Fixed;\
         dynamic n:# values:(n + 1)*[ item:int ] = Dynamic;",
        &Options::default(),
    );
    assert!(code.contains("pub values: [i32; 4]"));
    assert!(code.contains("pub values: Vec<i32>"));
}

#[test]
fn constructor_local_generic_names_are_not_type_identity() {
    let code = generate_schema(
        "nil {X:Type} = List X;\
         cons {Y:Type} head:Y tail:List<Y> = List Y;",
        &Options::default(),
    );
    assert!(code.contains("pub enum List<X>"));
    assert!(code.contains("Nil(NilConstructor<X>)"));
    assert!(code.contains("Cons(ConsConstructor<X>)"));
    assert!(code.contains("pub struct ConsConstructor<Y>"));
    assert_compiles(&code);
}

#[test]
fn natural_generics_and_addition_remain_in_the_generated_type() {
    let code = generate_schema(
        "octet value:int = Octet;\
         tuple {t:Type} {n:#} values:n*[item:t] = Tuple t n;\
         wrapped value:Tuple<Octet,(2 + 3)> = Wrapped;",
        &Options::default(),
    );
    assert!(code.contains("pub enum Tuple<T, N>"));
    assert!(code.contains("pub enum Wrapped"));
    assert!(code.contains("__tl::Add<__tl::Nat<2>, __tl::Nat<3>>"));
    assert_compiles(&code);
}

#[test]
fn emits_only_schema_derived_layer_source_and_flag_constants() {
    let schema = mono_tl_schema::parse(
        "// LAYER 42\n// @introduced 40\nitem#01020304 flags:# payload:flags.7?bytes = Item;",
        Source {
            name: "fixture".into(),
            url: Some("https://example.invalid/schema".into()),
        },
    )
    .unwrap();
    let code = generate(&schema, &Options::default()).unwrap();

    assert!(code.contains("pub const LAYER: u32 = 42"));
    assert!(code.contains("pub const SOURCE_URL: &str = \"https://example.invalid/schema\""));
    assert!(code.contains("pub const PAYLOAD_FLAG: u32 = 1_u32 << 7"));
    assert!(code.contains("pub const INTRODUCED_LAYER: Option<u32> = Some(40)"));
}
