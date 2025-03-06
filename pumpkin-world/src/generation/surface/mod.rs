use noise::Vector3;
use pumpkin_util::random::RandomDeriver;
use serde::Deserialize;

pub mod rule;

#[derive(Deserialize)]
pub enum MaterialCondition {
    Biome,
    NoiseThreshold,
    VerticalGradient(VerticalGradientMaterialCondition),
    YAbove,
    Water,
    Temperature,
    Steep,
    Not,
    Hole,
    AbovePreliminarySurface,
    StoneDepth,
}

#[derive(Deserialize)]
pub struct VerticalGradientMaterialCondition {
    random_name: String,
    true_at_and_below: YOffsetCodec,
    false_at_and_above: YOffsetCodec,
}

impl VerticalGradientMaterialCondition {
    pub fn test(&self, min_y: u32, random_deriver: RandomDeriver, block_pos: Vector3<i32>) -> bool {
        let true_at = self.true_at_and_below.get_y(min_y);
        let false_at = self.false_at_and_above.get_y(min_y);
        let splitter = random_deriver
            .split_string(&self.random_name)
            .next_splitter();
        let block_y = block_pos.y;
        if block_y <= true_at as i32 {
            return true;
        }
        if block_y >= false_at as i32 {
            return false;
        }
        let maped =
            pumpkin_util::math::map(block_y as f32, true_at as f32, false_at as f32, 1.0, 0.0);
        let mut random = splitter.split_pos(block_pos.x, block_y, block_pos.z);
        return random.next_f32() < maped;
    }
}

#[derive(Deserialize)]
pub enum YOffsetCodec {
    Top(AboveBottom),
    Bottom(BelowTop),
}

impl YOffsetCodec {
    pub fn get_y(&self, min_y: u32) -> u32 {
        match self {
            YOffsetCodec::Top(above_bottom) => min_y + above_bottom.above_bottom,
            YOffsetCodec::Bottom(below_top) => below_top.below_top,
        }
    }
}

#[derive(Deserialize)]
pub struct AboveBottom {
    above_bottom: u32,
}
#[derive(Deserialize)]
pub struct BelowTop {
    below_top: u32,
}
