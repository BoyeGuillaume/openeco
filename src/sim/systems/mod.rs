use bevy_ecs::{entity::Entity, query::Or, system::Query};

use crate::sim::{
    actor::{Bank, Human},
    assets::liquidity::Liquidity,
};

pub fn misc_merge_liquidity(
    liquidity: Query<(Entity, &Liquidity)>,
    entity: Query<(Entity, Or<(&Bank, &Human)>)>,
) {
}
