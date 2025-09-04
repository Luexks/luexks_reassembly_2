use crate::shapes::ports::Ports;
use crate::shapes::vertices::Vertices;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Default)]
pub struct Scale {
    pub verts: Vertices,
    pub ports: Ports,
    pub name: String,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", self.verts, self.ports)
    }
}
