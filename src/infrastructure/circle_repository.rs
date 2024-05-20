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
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => Err(Error::msg("Circle already exists")),
            None => {
                self.db
                    .set(circle.id.to_string(), &CircleData::from(circle.clone()))?;
                Ok(())
            }
        }
    }

    fn update(&self, circle: &Circle) -> Result<Circle, Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => self
                .db
                .set(circle.id.to_string(), &CircleData::from(circle.clone()))
                .and_then(|_| self.db.get::<CircleData, _>(&circle.id.to_string()))
                .map(|data| match data {
                    Some(data) => Circle::try_from(data),
                    None => Err(Error::msg("Failed to convert circle data")),
                })?,
            None => Err(Error::msg("Circle not found")),
        }
    }

    fn delete(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => self.db.remove(circle.id.to_string()),
            None => Err(Error::msg("Circle not found")),
        }
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

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        Ok(Circle::reconstruct(
            CircleId::from(data.id),
            data.name,
            Member::reconstruct(
                MemberId::from(data.owner.id),
                data.owner.name,
                data.owner.age,
                Grade::try_from(data.owner.grade)?,
                Major::from(data.owner.major.as_str()),
            ),
            data.capacity,
            data.members
                .into_iter()
                .map(Member::try_from)
                .collect::<Result<Vec<Member>, Error>>()?,
        ))
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

impl std::convert::From<Member> for MemberData {
    fn from(value: Member) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            age: value.age,
            grade: value.grade.into(),
            major: value.major.into(),
        }
    }
}

impl std::convert::TryFrom<MemberData> for Member {
    type Error = Error;

    fn try_from(value: MemberData) -> Result<Self, Self::Error> {
        Ok(Member::reconstruct(
            MemberId::from(value.id),
            value.name,
            value.age,
            Grade::try_from(value.grade)?,
            Major::from(value.major.as_str()),
        ))
    }
}