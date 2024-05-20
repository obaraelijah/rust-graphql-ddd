use anyhow::Error;

use crate::domain::aggregate::member::Member;
use crate::domain::aggregate::value_object::circle_id::CircleId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId, // Value object
    pub name: String,
    pub capacity: usize,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    pub fn new(name: String, capacity: usize, owner: Member) -> Result<Self, Error> {
        if capacity < 5 {
            return Err(Error::msg("Circle capacity must be 5 or more"));
        }

        Ok(Circle {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
        })
    }
}