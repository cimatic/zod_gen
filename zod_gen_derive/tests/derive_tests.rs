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
struct TestSmallIntegerStruct {
    byte: u8,
    layer: u16,
    offset: i8,
    temperature: i16,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
enum TestEnum {
    Foo,
    Bar,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
enum TestSmallIntegerEnum {
    Byte(u8),
    Temperature(i16),
}

#[test]
fn test_struct_schema() {
    let schema = TestStruct::zod_schema();
    assert!(schema.contains("a: z.string()"));
    assert!(schema.contains("b: z.number()"));
}

#[test]
fn test_small_integer_struct_schema() {
    let schema = TestSmallIntegerStruct::zod_schema();
    assert!(schema.contains("byte: z.number()"), "schema: {schema}");
    assert!(schema.contains("layer: z.number()"), "schema: {schema}");
    assert!(schema.contains("offset: z.number()"), "schema: {schema}");
    assert!(
        schema.contains("temperature: z.number()"),
        "schema: {schema}"
    );
}

#[test]
fn test_enum_schema() {
    let schema = TestEnum::zod_schema();
    assert!(schema.contains("z.literal('Foo')"));
    assert!(schema.contains("z.literal('Bar')"));
}

#[test]
fn test_small_integer_enum_schema() {
    let schema = TestSmallIntegerEnum::zod_schema();
    assert!(schema.contains("Byte: z.number()"), "schema: {schema}");
    assert!(
        schema.contains("Temperature: z.number()"),
        "schema: {schema}"
    );
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
