use anyhow::Error;

use crate::domain::aggregate::member::Member;
use crate::domain::aggregate::value_object::circle_id::CircleId;

use super::value_object::grade::Grade;

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
        if owner.grade != Grade::Third {
            return Err(Error::msg("Owner must be 3rd grade"));
        }
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

    pub fn reconstruct(id: CircleId, name: String, capacity: usize, owner: Member, members: Vec<Member>) -> Self {
        Circle {
            id,
            name,
            owner,
            capacity,
            members,
        }
    }

    pub fn add_member(&mut self, member: Member) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::msg("Circle member is full"));
        }

        if member.grade == Grade::Fourth {
            return Err(Error::msg("4th grade can't join circle"));
        }

        self.members.push(member);
        Ok(())
    }

    pub fn update(&mut self, name: Option<String>, capacity: Option<usize>) {
        if let Some(name) = name {
            self.name = name;
        }

        if let Some(capacity) = capacity {
            self.capacity = capacity;
        }
    }

    pub fn is_full(&self) -> bool {
        self.members.len() + 1 >= self.capacity 
    }

    pub fn is_drinkable_alcohol(member: &Member) -> bool {
        member.is_adult()
    }

    fn is_runnable(&self) -> bool {
        self.members.len() + 1 >= 5
    }

    pub fn remove_member(&mut self, member: &Member) -> Result<(), Error> {
        if self.owner.id == member.id {
            return Err(Error::msg("Owner can't be removed"));
        }
        self.members.retain(|m| m.id != member.id);
        Ok(())
    }

    pub fn graduate(&mut self) {
        self.members.retain(|m| m.grade != Grade::Fourth);
    }
}