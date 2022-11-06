use serde::{Serialize, Serializer, ser::SerializeMap};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Workflows {
    pub(crate) workflows: BTreeMap<String, Workflow>,
}

#[derive(Debug, Serialize)]
pub struct Workflow {
    pub(crate) jobs: Vec<WorkflowJob>,
}

#[derive(Debug)]
pub struct WorkflowJob {
    pub(crate) name: WorkflowJobName,
    pub(crate) parameters: BTreeMap<String, ParameterValue>,
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
pub enum WorkflowJobName {
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

impl From<WorkflowJobName> for String {
    fn from(name: WorkflowJobName) -> String {
        match name {
            WorkflowJobName::Local(name) => name,
            WorkflowJobName::Orb(namespace, name) => format!("{namespace}/{name}"),
        }
    }
}

impl<NS, N> From<(NS, N)> for WorkflowJobName
where
    NS: Into<String>,
    N: Into<String>,
{
    fn from((namespace, name): (NS, N)) -> WorkflowJobName {
        WorkflowJobName::Orb(namespace.into(), name.into())
    }
}

#[derive(Debug)]
pub enum ParameterValue {
    Literal(String),
    PipelineTemplate(String),
}


