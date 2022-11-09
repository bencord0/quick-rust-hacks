use quick_rust_hacks::circleci::Pipeline;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_conditionals: bool = env::var("PARAM_CONDITIONALS")
        .as_deref().unwrap_or("true") == "true";
    let use_loops: bool = env::var("PARAM_LOOPS")
        .as_deref().unwrap_or("true") == "true";

    let mut pipeline = Pipeline::builder()
        .orb("rusty", ("bencord0", "rusty-orb", "1.1.0"))
        .string_parameter("to", "World", Some("Whom to greet?"));

    if use_conditionals {
        // Can build up the pipeline conditionally
        pipeline = pipeline
        .workflow("hello-1")
            .job(("rusty", "hello"))
                .string_pipeline_parameter("to", "to")
                .finish()
            .finish();
    }

    if use_loops {
        // Can also build the pipeline incrementally
        for workflow_name in ["hello-2", "hello-3"] {
            pipeline = pipeline
            .workflow(workflow_name)
                .job(("rusty", "hello"))
                    .string_pipeline_parameter("to", "to")
                    .finish()
                .finish();
        }
    }

    let yaml = serde_yaml::to_string(&pipeline.build()?)?;
    println!("{}", yaml);

    Ok(())
}
