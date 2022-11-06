use quick_rust_hacks::circleci::Pipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline::builder()
        .orb("rusty", ("bencord0", "rusty-orb", "1.1.0"))
        .string_parameter("to", "World", Some("Whom to greet?"))

        .workflow("hello")
            .job(("rusty", "hello"))
                .string_pipeline_parameter("to", "to")
                .finish()
            .finish()

        .build()?;

    let yaml = serde_yaml::to_string(&pipeline)?;
    println!("{}", yaml);

    Ok(())
}
