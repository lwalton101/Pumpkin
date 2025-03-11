use async_trait::async_trait;
use pumpkin_data::block::{Block, BlockProperties, HorizontalFacing, WallTorchLikeProperties};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_world::block::registry;
use pumpkin_world::block::{BlockDirection};

use crate::{
    block::{pumpkin_block::PumpkinBlock},
    server::Server,
    world::World,
};

#[pumpkin_block("minecraft:torch")]
pub struct TorchBlock;

#[async_trait]
impl PumpkinBlock for TorchBlock {
    async fn on_place(&self, _server: &Server, _world: &World, block: &Block, _face: &BlockDirection, _block_pos: &BlockPos, _use_item_on: &SUseItemOn, _player_direction: &HorizontalFacing, _other: bool) -> u16 {
        let standing_block = Block::WALL_TORCH;
        let mut properties = WallTorchLikeProperties::default(&standing_block);

        match _face {
            BlockDirection::North | BlockDirection::South | BlockDirection::West | BlockDirection::East => {
                properties.facing = _face.to_cardinal_direction().opposite();
                return properties.to_state_id(&standing_block);
            }
            _ => {}
        }


        block.default_state_id
    }
}
