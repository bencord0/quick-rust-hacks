use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use crate::circleci::{
    jobs::*,
    orbs::*,
    parameters::*,
    workflows::*,
};

#[derive(Debug, Serialize)]
pub struct Pipeline {
    version: String,
    orbs: Orbs,
    parameters: Parameters,
    jobs: Jobs,
    workflows: Workflows,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder {
            orbs: BTreeMap::new(),
            parameters: BTreeMap::new(),
            workflows: BTreeMap::new(),
        }
    }
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

pub struct PipelineBuilder {
    orbs: BTreeMap<String, Orb>,
    parameters: BTreeMap<String, Parameter>,
    workflows: BTreeMap<String, Workflow>,
}

impl PipelineBuilder {
    pub fn orb<R, O>(mut self, reference: R, orb: O) -> Self
    where
        R: Into<String>,
        O: Into<Orb>,
    {
        self.orbs.insert(reference.into(), orb.into());
        self
    }

    pub fn string_parameter<N, D, DESC>(mut self, name: N, default: D, description: Option<DESC>) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        DESC: Into<String>,
    {
        self.parameters.insert(name.into(), Parameter {
            value: ParameterType::String(default.into()),
            description: description.map(|d| d.into()),
        });
        self
    }

    fn bool_parameter(self) -> Self {
        unimplemented!();
    }

    pub fn workflow<N>(self, name: N) -> WorkflowBuilder
    where
        N: Into<String>,
    {
        WorkflowBuilder {
            name: name.into(),
            jobs: Vec::new(),
            pipeline: self,
        }
    }

    pub fn build(self) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let pipeline = Pipeline {
            version: "2.1".into(),
            orbs: Orbs { orbs: self.orbs},
            parameters: Parameters { parameters: self.parameters },
            jobs: Jobs {},
            workflows: Workflows { workflows: self.workflows },
        };
        Ok(pipeline)
    }
}

pub struct WorkflowBuilder {
    name: String,
    jobs: Vec<WorkflowJob>,
    pipeline: PipelineBuilder,
}

impl WorkflowBuilder {
    pub fn job<N>(self, name: N) -> WorkflowJobBuilder
    where
        N: Into<WorkflowJobName>,
    {
        WorkflowJobBuilder {
            name: name.into(),
            parameters: BTreeMap::new(),
            workflow: self,
        }
    }

    pub fn finish(self) -> PipelineBuilder {
        let mut pipeline = self.pipeline;
        pipeline.workflows.insert(self.name, Workflow { jobs: self.jobs });
        pipeline
    }
}

pub struct WorkflowJobBuilder {
    name: WorkflowJobName,
    parameters: BTreeMap<String, ParameterValue>,
    workflow: WorkflowBuilder,
}

impl WorkflowJobBuilder {
    fn string_literal_parameter<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.parameters.insert(name.into(), ParameterValue::Literal(value.into()));
        self
    }

    pub fn string_pipeline_parameter<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.parameters.insert(name.into(), ParameterValue::PipelineTemplate(value.into()));
        self
    }

    pub fn finish(self) -> WorkflowBuilder {
        let mut workflow = self.workflow;
        workflow.jobs.push(WorkflowJob {
            name: self.name,
            parameters: self.parameters,
        });

        workflow
    }
}
