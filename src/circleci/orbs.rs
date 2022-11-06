use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Orbs {
    pub(crate) orbs: BTreeMap<String, Orb>,
}

#[derive(Debug)]
pub struct Orb {
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

impl<NS, N, V> From<(NS, N, V)> for Orb
where
    NS: Into<String>,
    N: Into<String>,
    V: Into<String>,
{
    fn from((namespace, name, version): (NS, N, V)) -> Self
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            version: version.into(),
        }
    }
}
