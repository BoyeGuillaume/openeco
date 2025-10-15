use bevy_ecs::{component::Component, entity::Entity};

/// Represents a share or stock of a business owned by another entity, all share of a business sum to 100%
#[derive(Component, Clone, Copy)]
pub struct Share {
    /// Must be a [`Business`]
    pub target: Entity,

    /// Current owner, can be a [`Human`], [`Business`] or [`Government`]
    pub owner: Entity,

    /// By default, shares must sum [`u64::MAX`] (100%)
    pub fraction: u64,
}
