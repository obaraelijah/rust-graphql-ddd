use crate::domain::aggregate::value_object::member_id::MemberId;

use super::value_object::{grade::Grade, major::Major};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId, // Value object
    pub name: String,
    pub age: usize,
    pub grade: Grade,
    pub major: Major,
}
impl Member {
    pub fn new(name: String, age: usize, grade: Grade, major: Major) -> Self {
        Member {
            id: MemberId::gen(),
            name,
            age,
            grade,
            major,
        }
    }

    pub fn reconstruct(id: MemberId, name: String, age: usize, grade: Grade, major: Major) -> Self {
        Member {
            id,
            name,
            age,
            grade,
            major,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 20
    }
}
