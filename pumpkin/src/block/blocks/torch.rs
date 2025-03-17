use async_trait::async_trait;
use log::error;
use pumpkin_data::block::{Block, BlockProperties, EnumVariants, FurnaceLikeProperties, HorizontalFacing, WallTorchLikeProperties};
use pumpkin_data::block::BlockFace::Wall;
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::block::BlockDirection;

use crate::{block::pumpkin_block::PumpkinBlock, server::Server, world::World};

#[pumpkin_block("minecraft:torch")]
pub struct TorchBlock;
#[pumpkin_block("minecraft:redstone_torch")]
pub struct RedstoneTorchBlock;
#[pumpkin_block("minecraft:soul_torch")]
pub struct SoulTorchBlock;
#[async_trait]
impl PumpkinBlock for TorchBlock {
    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        block: &Block,
        face: &BlockDirection,
        block_pos: &BlockPos,
        _use_item_on: &SUseItemOn,
        player_direction: &HorizontalFacing,
        _other: bool,
    ) -> u16 {
        match face {
            BlockDirection::Down => {
                block.default_state_id
            }
            _ => {
                let props = get_wall_block_props(world, block, block_pos, face, player_direction, WallTorchLikeProperties::default(&Block::WALL_TORCH).to_props()).await;
                WallTorchLikeProperties::from_props(props, &Block::WALL_TORCH).to_state_id(&Block::WALL_TORCH)
            }
        }
    }
}

#[async_trait]
impl PumpkinBlock for RedstoneTorchBlock {
    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        block: &Block,
        face: &BlockDirection,
        block_pos: &BlockPos,
        _use_item_on: &SUseItemOn,
        player_direction: &HorizontalFacing,
        _other: bool,
    ) -> u16 {
        match face {
            BlockDirection::Down => {
                block.default_state_id
            }
            _ => {
                let props = get_wall_block_props(world, block, block_pos, face, player_direction, FurnaceLikeProperties::default(&Block::REDSTONE_WALL_TORCH).to_props()).await;
                FurnaceLikeProperties::from_props(props, &Block::REDSTONE_WALL_TORCH).to_state_id(&Block::REDSTONE_WALL_TORCH)
            }
        }
    }
}

#[async_trait]
impl PumpkinBlock for SoulTorchBlock {
    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        block: &Block,
        face: &BlockDirection,
        block_pos: &BlockPos,
        _use_item_on: &SUseItemOn,
        player_direction: &HorizontalFacing,
        _other: bool,
    ) -> u16 {
        match face {
            BlockDirection::Down => {
                block.default_state_id
            }
            _ => {
                let props = get_wall_block_props(world, block, block_pos, face, player_direction, WallTorchLikeProperties::default(&Block::SOUL_WALL_TORCH).to_props()).await;
                WallTorchLikeProperties::from_props(props, &Block::SOUL_WALL_TORCH).to_state_id(&Block::SOUL_WALL_TORCH)
            }
        }
    }
}
async fn get_wall_block_props(world: &World, block: &Block, block_pos: &BlockPos, _face: &BlockDirection, player_direction:&HorizontalFacing, mut block_properties: Vec<(String, String)>) -> Vec<(String, String)> {

    let contains_facing = block_properties.iter().any(|(key, _)| key == "facing");

    if !contains_facing{
        error!("Cannot find facing property in block {}", block.name);
        return Block::AIR.properties(Block::AIR.default_state_id).unwrap().to_props()
    }

    let facing_index = block_properties.iter().position(|(key, _)| key == "facing").unwrap();

    match _face {
        BlockDirection::North
        | BlockDirection::South
        | BlockDirection::West
        | BlockDirection::East => {
            block_properties[facing_index].1 = _face.to_cardinal_direction().opposite().to_value().to_string();
            return block_properties;
        }
        BlockDirection::Up => {
            let mut possible_directions = vec![];
            for bd in BlockDirection::horizontal() {
                let block_offset = bd.to_offset();
                let base_block_pos = block_pos.offset(block_offset);
                let base_block = world.get_block(&base_block_pos).await.unwrap();
                if base_block.id != 0 {
                    possible_directions.push(bd.to_cardinal_direction());
                }
            }

            return if possible_directions.contains(player_direction) {
                block_properties[facing_index].1 = player_direction.opposite().to_value().to_string();
                block_properties
            } else if possible_directions.is_empty() {
                block_properties[facing_index].1 = possible_directions[0].opposite().to_value().to_string();
                block_properties
            } else {
                Block::AIR.properties(Block::AIR.default_state_id).unwrap().to_props()
            };
        }
        BlockDirection::Down => {}
    }

    block_properties
}
