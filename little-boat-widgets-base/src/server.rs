use std::cmp::Ordering;
use std::sync::Arc;
use std::{cmp, fmt, str};

use serde::{Deserialize, Serialize};

use crate::bouncer::BouncerNetwork;

pub type ServerName = Arc<str>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct Server {
    pub name: ServerName,
    pub network: Option<Arc<BouncerNetwork>>,
}

impl Server {
    pub fn is_bouncer_network(&self) -> bool {
        self.network.is_some()
    }

    pub fn bouncer_netid(&self) -> Option<&str> {
        self.network.as_ref().map(|network| &*network.id)
    }

    pub fn parent(&self) -> Option<Server> {
        if self.network.is_some() {
            Some(Self {
                network: None,
                ..self.clone()
            })
        } else {
            None
        }
    }
}

impl From<ServerName> for Server {
    fn from(name: ServerName) -> Self {
        Self {
            name,
            network: None,
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(network) = &self.network {
            network.name.fmt(f)
        } else {
            self.name.fmt(f)
        }
    }
}


// here is a machine-readable (not-friendly) representation of a server, which
// can be hashed and used for history. Due to existing constraints this must be
// the server name if no bouncer network exists (due to the constraint of
// existing history).
impl fmt::Binary for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(network) = &self.network {
            // we insert a NUL to make sure no conflicts exist with existing networks
            write!(f, "\0{}", network.name)?;
        }
        Ok(())
    }
}

// Use case-insensitive comparison first, falling back to case-sensitive
// only when server names are equal (in a case-insensitive context).
impl Ord for Server {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let case_insensitive_ordering =
            self.name.to_lowercase().cmp(&other.name.to_lowercase());

        if !matches!(case_insensitive_ordering, cmp::Ordering::Equal) {
            return case_insensitive_ordering;
        }

        self.network.cmp(&other.network)
    }
}

impl PartialOrd for Server {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'de> Deserialize<'de> for Server {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NameNetwork {
            name: ServerName,
            network: Option<Arc<BouncerNetwork>>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Data {
            String(String),
            NameNetwork(NameNetwork),
        }

        let data = Data::deserialize(deserializer)?;

        Ok(match data {
            Data::String(name) => Server {
                name: name.into(),
                network: None,
            },
            Data::NameNetwork(NameNetwork { name, network }) => {
                Server { name, network }
            }
        })
    }
}

