use serde::{Serialize, Serializer, ser::SerializeMap};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Parameters {
    pub(crate) parameters: BTreeMap<String, Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
    pub(crate) value: ParameterType,
    pub(crate) description: Option<String>,
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
pub enum ParameterType {
    String(String),
    Boolean(bool),
}


