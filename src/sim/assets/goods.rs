use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Goods {
    /// Unique identifier of the good
    pub name: Uuid,

    /// Unit of the good (e.g., kg, liter, piece)
    pub unit: u64,
}
