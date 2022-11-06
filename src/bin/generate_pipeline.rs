use serde::{Serialize, Serializer, ser::SerializeMap};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
struct Pipeline {
    version: String,
    orbs: Orbs,
    parameters: Parameters,
    jobs: Jobs,
    workflows: Workflows,
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct Orbs {
    orbs: BTreeMap<String, Orb>,
}

#[derive(Debug)]
struct Orb {
    namespace: String,
    name: String,
    version: String,
}

impl Serialize for Orb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let orb = format!("{}/{}@{}", self.namespace, self.name, self.version);
        serializer.serialize_str(&orb)
    }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct Parameters {
    parameters: BTreeMap<String, Parameter>,
}

#[derive(Debug)]
struct Parameter {
    value: ParameterType,
    description: Option<String>,
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let size = if self.description.is_some() { 3 } else { 2 };
        let mut p = serializer.serialize_map(Some(size))?;

        match &self.value {
            ParameterType::String(value) => {
                p.serialize_entry("type", "string")?;
                p.serialize_entry("default", value)?;
            },
            ParameterType::Boolean(value) => {
                p.serialize_entry("type", "bool")?;
                p.serialize_entry("default", value)?;
            }
        };

        if let Some(description) = &self.description {
            p.serialize_entry("description", description)?;
        };

        p.end()
    }
}

#[derive(Debug)]
enum ParameterType {
    String(String),
    Boolean(bool),
}

#[derive(Debug, Serialize)]
struct Jobs {}

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct Workflows {
    workflows: BTreeMap<String, Workflow>,
}

#[derive(Debug, Serialize)]
struct Workflow {
    jobs: Vec<WorkflowJob>,
}

#[derive(Debug)]
struct WorkflowJob {
    name: WorkflowJobName,
    parameters: BTreeMap<String, ParameterValue>,
}

impl Serialize for WorkflowJob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut job = serializer.serialize_map(Some(1))?;
        job.serialize_entry(&self.name, &self.parameters)?;

        job.end()
    }
}

#[derive(Debug)]
enum WorkflowJobName {
    Local(String),
    Orb(String, String),
}

impl Serialize for WorkflowJobName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let job_name = match self {
            WorkflowJobName::Local(name) => name.clone(),
            WorkflowJobName::Orb(orb, job) => format!("{orb}/{job}")
        };

        serializer.serialize_str(&job_name)
    }
}

#[derive(Debug)]
enum ParameterValue {
    Literal(String),
    PipelineTemplate(String),
}

impl Serialize for ParameterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            ParameterValue::Literal(value) => value.clone(),
            ParameterValue::PipelineTemplate(value) => {
                format!("<< pipeline.parameters.{value} >>")
            },
        };
        serializer.serialize_str(&value)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline {
        version: "2.1".to_string(),

        orbs: Orbs {
            orbs: {
                let mut orbs = BTreeMap::new();
                orbs.insert("rusty".to_string(), Orb {
                    namespace: "bencord0".to_string(),
                    name: "rusty-orb".to_string(),
                    version: "1.1.0".to_string(),
                });
                orbs
            },
        },

        parameters: Parameters {
            parameters: {
                let mut parameters = BTreeMap::new();
                parameters.insert("to".to_string(), Parameter {
                    value: ParameterType::String("World".to_string()),
                    description: Some("Whom to greet?".to_string()),
                });
                parameters
            },
        },

        jobs: Jobs {},

        workflows: Workflows {
            workflows: {
                let mut workflows = BTreeMap::new();
                workflows.insert("hello".to_string(), Workflow {
                    jobs: vec![
                        WorkflowJob {
                            name: WorkflowJobName::Orb("rusty".to_string(), "hello".to_string()),
                            parameters: {
                                let mut parameters = BTreeMap::new();
                                parameters.insert("to".to_string(), ParameterValue::PipelineTemplate("to".to_string()));
                                parameters
                            },
                        },
                    ],
                });
                workflows
            },
        }
    };

    let yaml = serde_yaml::to_string(&pipeline)?;
    println!("{}", yaml);

    Ok(())
}
