use serde::Serialize;
use zod_gen::ZodSchema as _;
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema)]
#[allow(dead_code)]
struct TestStruct {
    a: String,
    b: i32,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
enum TestEnum {
    Foo,
    Bar,
}

#[test]
fn test_struct_schema() {
    let schema = TestStruct::zod_schema();
    assert!(schema.contains("a: z.string()"));
    assert!(schema.contains("b: z.number()"));
}

#[test]
fn test_enum_schema() {
    let schema = TestEnum::zod_schema();
    assert!(schema.contains("z.literal('Foo')"));
    assert!(schema.contains("z.literal('Bar')"));
}

#[derive(ZodSchema, Serialize)]
#[allow(dead_code)]
struct TestStructWithRename {
    #[serde(rename = "FOOBAR")]
    foobar: String,
}

#[test]
fn test_struct_rename() {
    let schema = TestStructWithRename::zod_schema();
    assert!(schema.contains("FOOBAR: z.string()"));
}
