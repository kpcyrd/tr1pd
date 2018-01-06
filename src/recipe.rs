#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockRecipe {
    Rekey,
    Info(Vec<u8>),
}

impl BlockRecipe {
    pub fn info(buf: Vec<u8>) -> BlockRecipe {
        BlockRecipe::Info(buf)
    }
}
