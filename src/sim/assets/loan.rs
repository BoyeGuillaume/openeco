use std::time::Duration;

use bevy_ecs::{component::Component, entity::Entity};

use crate::currency::MonetaryValue;

/// Represents a loan or a bond issued by a bank/lender to a borrower
#[derive(Component)]
pub struct Loan {
    /// The lender, must be a [`Bank`]
    pub lender: Entity,

    /// The borrower, must be a [`Human`] or a [`Business`]
    pub borrower: Entity,

    /// The amount borrowed
    pub amount: MonetaryValue,

    /// The interest rate as a percentage (e.g., 5.0 for 5%)
    pub interest_rate: f64,

    /// The duration of the loan
    pub duration_months: Duration,
}
