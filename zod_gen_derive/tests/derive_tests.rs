use zod_gen_derive::ZodSchema;
use zod_gen::ZodSchema as _;

#[derive(ZodSchema)]
struct TestStruct {
    a: String,
    b: i32,
}

#[derive(ZodSchema)]
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