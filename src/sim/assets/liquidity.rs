use bevy_ecs::{component::Component, entity::Entity};

use crate::currency::MonetaryValue;

/// Represents liquid assets (cash or equivalents) owned by an entity
#[derive(Component, Clone, Copy, Debug)]
pub struct Liquidity {
    /// Current owner, can be a [`Human`], [`Business`] or [`Government`]
    pub owner: Entity,

    /// Amount of liquid money available
    pub amount: MonetaryValue,
}
