use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(usize);

impl CircleId {
    pub fn gen() -> Self {
        Self(rand::random::<usize>())
    }
}