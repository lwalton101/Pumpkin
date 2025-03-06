use pumpkin_util::{math::vector3::Vector3, random::RandomDeriver};
use serde::Deserialize;

pub mod rule;

pub struct MaterialRuleContext<'a> {
    pub min_y: i8,
    pub height: u16,
    pub random_deriver: &'a RandomDeriver,
    pub block_pos: Vector3<i32>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MaterialCondition {
    #[serde(rename = "minecraft:biome")]
    Biome,
    #[serde(rename = "minecraft:noise_threshold")]
    NoiseThreshold,
    #[serde(rename = "minecraft:vertical_gradient")]
    VerticalGradient(VerticalGradientMaterialCondition),
    #[serde(rename = "minecraft:y_above")]
    YAbove,
    #[serde(rename = "minecraft:water")]
    Water,
    #[serde(rename = "minecraft:temperature")]
    Temperature,
    #[serde(rename = "minecraft:steep")]
    Steep,
    #[serde(rename = "minecraft:not")]
    Not,
    #[serde(rename = "minecraft:hole")]
    Hole,
    #[serde(rename = "minecraft:above_preliminary_surface")]
    AbovePreliminarySurface,
    #[serde(rename = "minecraft:stone_depth")]
    StoneDepth,
}

impl MaterialCondition {
    pub fn test(&self, context: &MaterialRuleContext) -> bool {
        match self {
            MaterialCondition::Biome => todo!(),
            MaterialCondition::NoiseThreshold => todo!(),
            MaterialCondition::VerticalGradient(vertical_gradient_material_condition) => {
                vertical_gradient_material_condition.test(context)
            }
            MaterialCondition::YAbove => todo!(),
            MaterialCondition::Water => todo!(),
            MaterialCondition::Temperature => todo!(),
            MaterialCondition::Steep => todo!(),
            MaterialCondition::Not => todo!(),
            MaterialCondition::Hole => todo!(),
            MaterialCondition::AbovePreliminarySurface => todo!(),
            MaterialCondition::StoneDepth => todo!(),
        }
    }
}

#[derive(Deserialize)]
pub struct VerticalGradientMaterialCondition {
    random_name: String,
    true_at_and_below: YOffsetCodec,
    false_at_and_above: YOffsetCodec,
}

impl VerticalGradientMaterialCondition {
    pub fn test(&self, context: &MaterialRuleContext) -> bool {
        let true_at = self.true_at_and_below.get_y(context);
        let false_at = self.false_at_and_above.get_y(context);
        let splitter = context
            .random_deriver
            .split_string(&self.random_name)
            .next_splitter();
        let block_y = context.block_pos.y;
        if block_y <= true_at as i32 {
            return true;
        }
        if block_y >= false_at as i32 {
            return false;
        }
        let maped =
            pumpkin_util::math::map(block_y as f32, true_at as f32, false_at as f32, 1.0, 0.0);
        let mut random = splitter.split_pos(context.block_pos.x, block_y, context.block_pos.z);
        return random.next_f32() < maped;
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum YOffsetCodec {
    Absolute(Absolute),
    AboveBottom(AboveBottom),
    BelowTop(BelowTop),
}

impl YOffsetCodec {
    pub fn get_y(&self, context: &MaterialRuleContext) -> i16 {
        match self {
            YOffsetCodec::AboveBottom(above_bottom) => {
                context.min_y as i16 + above_bottom.above_bottom as i16
            }
            YOffsetCodec::BelowTop(below_top) => {
                context.height as i16 - 1 + context.min_y as i16 - below_top.below_top as i16
            }
            YOffsetCodec::Absolute(absolute) => absolute.absolute as i16,
        }
    }
}

#[derive(Deserialize)]
pub struct Absolute {
    absolute: i8,
}

#[derive(Deserialize)]
pub struct AboveBottom {
    above_bottom: i8,
}
#[derive(Deserialize)]
pub struct BelowTop {
    below_top: i8,
}
