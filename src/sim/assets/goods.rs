use bevy_ecs::component::Component;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Component)]
pub struct Goods {
    /// Owner of the good, can be a [`Human`], [`Business`] or [`Government`]
    pub owner: Uuid,

    /// Unique identifier of the good
    pub name: Uuid,

    /// Unit of the good (e.g., kg, liter, piece)
    pub unit: u64,
}
