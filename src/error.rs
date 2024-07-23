use std::fmt::{Display, Formatter, Result};
use std::error::Error;

/// A type to for errors when parsing newick strings
#[derive(Debug)]
pub struct NewickErr(pub String);

impl Display for NewickErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NewickErr {}

/// A type to for errors when parsing Nexus files
#[derive(Debug)]
pub struct NexusErr(pub String);

impl Display for NexusErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NexusErr {}