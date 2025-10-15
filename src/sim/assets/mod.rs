use strum::{EnumIs, EnumTryAs};

pub mod goods;
pub mod money;

#[derive(Debug, Clone, EnumIs, EnumTryAs)]
pub enum Asset {
    Goods(goods::Goods),
    Money(money::Money),
}
