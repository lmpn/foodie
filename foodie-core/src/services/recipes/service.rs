#[derive(Clone, Debug)]
pub struct RecipeService<Storage>
where
    Storage: Send + Sync,
{
    pub(crate) storage: Storage,
}

impl<Storage> RecipeService<Storage>
where
    Storage: Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
