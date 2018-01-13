use blocks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockRecipe {
    Rekey,
    Info(Vec<u8>),
}

impl BlockRecipe {
    pub fn info(buf: Vec<u8>) -> Result<BlockRecipe, blocks::Error> {
        blocks::validate_block_size(buf.len())?;
        Ok(BlockRecipe::Info(buf))
    }
}
