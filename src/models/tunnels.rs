use sea_orm::entity::prelude::*;
use super::_entities::tunnels::{ActiveModel, Entity};
pub type Tunnels = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
