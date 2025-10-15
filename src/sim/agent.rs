use std::time::Duration;

use bevy_ecs::{component::Component, entity::Entity};

use crate::sim::assets::money::Money;

/// Represents a business entity in the simulation
#[derive(Component)]
pub struct Business {}

/// Represents a government entity in the simulation
#[derive(Component)]
pub struct Government {}

/// Represents a bank entity in the simulation
#[derive(Component)]
pub struct Bank {
    /// The bank's total reserves (current liquidity) in various currencies
    pub reserves: Vec<Money>,
}

/// Represents a loan taken by an entity
#[derive(Component)]
pub struct Loan {
    /// The lender, must be a [`Bank`]
    pub lender: Entity,

    /// The borrower, must be a [`Human`] or a [`Business`]
    pub borrower: Entity,

    /// The amount borrowed
    pub amount: Money,

    /// The interest rate as a percentage (e.g., 5.0 for 5%)
    pub interest_rate: f64,

    /// The duration of the loan
    pub duration_months: Duration,
}

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

/// Represents a human agent in the simulation
#[derive(Component)]
pub struct Human {
    pub assets: Money,
}
