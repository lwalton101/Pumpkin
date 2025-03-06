use serde::Deserialize;

use crate::block::BlockStateCodec;

use super::MaterialCondition;

#[derive(Deserialize)]
pub enum MaterialRule {
    Badlands,
    Block(BlockMaterialRule),
    Sequence(SequenceMaterialRule),
    Condition(ConditionMaterialRule),
}

impl MaterialRule {}

#[derive(Deserialize)]
pub struct BlockMaterialRule {
    result_state: BlockStateCodec,
}

#[derive(Deserialize)]
pub struct SequenceMaterialRule {
    sequence: Vec<MaterialRule>,
}

#[derive(Deserialize)]
pub struct ConditionMaterialRule {
    if_true: MaterialCondition,
    then_run: Box<MaterialRule>,
}
