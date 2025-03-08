use serde::Deserialize;

use crate::block::{BlockState, BlockStateCodec};

use super::{MaterialCondition, MaterialRuleContext};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MaterialRule {
    #[serde(rename = "minecraft:bandlands")]
    Badlands,
    #[serde(rename = "minecraft:block")]
    Block(BlockMaterialRule),
    #[serde(rename = "minecraft:sequence")]
    Sequence(SequenceMaterialRule),
    #[serde(rename = "minecraft:condition")]
    Condition(ConditionMaterialRule),
}

impl MaterialRule {
    pub fn try_apply(&self, context: &mut MaterialRuleContext) -> Option<BlockState> {
        match self {
            MaterialRule::Badlands => todo!(),
            MaterialRule::Block(block) => block.try_apply(),
            MaterialRule::Sequence(sequence) => sequence.try_apply(context),
            MaterialRule::Condition(condition) => condition.try_apply(context),
        }
    }
}

#[derive(Deserialize)]
pub struct BlockMaterialRule {
    result_state: BlockStateCodec,
}

impl BlockMaterialRule {
    pub fn try_apply(&self) -> Option<BlockState> {
        BlockState::new(&self.result_state.name)
    }
}

#[derive(Deserialize)]
pub struct SequenceMaterialRule {
    sequence: Vec<MaterialRule>,
}

impl SequenceMaterialRule {
    pub fn try_apply(&self, context: &mut MaterialRuleContext) -> Option<BlockState> {
        for seq in &self.sequence {
            if let Some(state) = seq.try_apply(context) {
                return Some(state);
            }
        }
        None
    }
}

#[derive(Deserialize)]
pub struct ConditionMaterialRule {
    if_true: MaterialCondition,
    then_run: Box<MaterialRule>,
}

impl ConditionMaterialRule {
    pub fn try_apply(&self, context: &mut MaterialRuleContext) -> Option<BlockState> {
        if self.if_true.test(context) {
            return self.then_run.try_apply(context);
        }
        None
    }
}
