use bevy_ecs::component::Component;

/// Represents a human agent in the simulation
#[derive(Component)]
pub struct Human {}

/// Represents a business entity in the simulation
#[derive(Component)]
pub struct Business {}

/// Represents a bank entity in the simulation
///
/// A bank is a special type of business that can issue loans
#[derive(Component)]
pub struct Bank {}

/// Represents a government entity in the simulation
///
/// A government can own shares in businesses, collect taxes, and issue currency
#[derive(Component)]
pub struct Government {}
