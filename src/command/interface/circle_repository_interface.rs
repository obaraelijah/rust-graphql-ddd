use std::sync::Arc;

use anyhow::Error;

use crate::domain::aggregate::circle::Circle;
use crate::domain::aggregate::value_object::circle_id::CircleId;

pub trait CircleRepositoryInterface {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    fn create(&self, circle: &Circle) -> Result<(), Error>;
    fn update(&self, circle: &Circle) -> Result<Circle, Error>;
    fn delete(&self, circle: &Circle) -> Result<(), Error>;
}

pub trait HasCircleRepository {
    fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync>;
}
