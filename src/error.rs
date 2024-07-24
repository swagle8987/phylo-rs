use std::fmt::{Display, Formatter, Result};
use std::error::Error;

/// A type for errors when parsing newick strings
#[derive(Debug)]
pub struct NewickErr(pub String);

impl Display for NewickErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NewickErr {}

/// A type for errors when parsing Nexus files
#[derive(Debug)]
pub struct NexusErr(pub String);

impl Display for NexusErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NexusErr {}

/// A type for errors when mutating trees
#[derive(Debug)]
pub struct TreeMutationErr(pub String);

impl Display for TreeMutationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TreeMutationErr {}

/// A type for errors when querying trees
#[derive(Debug)]
pub struct TreeQueryErr(pub String);

impl Display for TreeQueryErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TreeQueryErr {}

/// A type for errors when mutating Nodes
#[derive(Debug)]
pub struct NodeMutationErr(pub String);

impl Display for NodeMutationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NodeMutationErr {}

/// A type for errors when querying Nodes
#[derive(Debug)]
pub struct NodeQueryErr(pub String);

impl Display for NodeQueryErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NodeQueryErr {}