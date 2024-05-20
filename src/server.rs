use std::sync::Arc;

use crate::{
    command::{
        command_handler::CommandHandler,
        interface::circle_repository_interface::{CircleRepositoryInterface, HasCircleRepository},
    }
};

pub fn new() -> anyhow::Result<()> {
    Ok(())
}