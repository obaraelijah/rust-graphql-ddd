use anyhow::Error;

use crate::{
    command::interface::circle_repository_interface::CircleRepositoryInterface,
    domain::aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
};

use super::db::Db;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: Db,
}

impl CircleRepository {
    //TODO: Chnage DB to mysql
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl CircleRepositoryInterface for CircleRepository {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        match self.db.get::<CircleData, _>(&circle_id.to_string())? {
           Some(data) => Ok(Circle::try_from(data)?),
           None => Err(Error::msg("Circle not found")),
        }
    }

    fn create(&self, circle: &Circle) -> Result<(), Error> {
        
    }

    fn update(&self, circle: &Circle) -> Result<Circle, Error> {
        
    }

    fn delete(&self, circle: &Circle) -> Result<(), Error> {
        
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CircleData {
    id: usize,
    name: String,
    owner: MemberData,
    capacity: usize,
    members: Vec<MemberData>,
}

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        CircleData {
            id: circle.id.into(),
            name: circle.name,
            owner: MemberData::from(circle.owner),
            capacity: circle.capacity,
            members: circle.members.into_iter().map(MemberData::from).collect(),
        }
    }
}    
#[derive(serde::Deserialize, serde::Serialize)]
struct MemberData {
    id: usize,
    name: String,
    age: usize,
    grade: usize,
    major: String,
}
