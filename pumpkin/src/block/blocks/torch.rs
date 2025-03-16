use async_trait::async_trait;
use pumpkin_data::block::{Block, BlockProperties, FurnaceLikeProperties, HorizontalFacing, WallTorchLikeProperties};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::block::BlockDirection;

use crate::{
    block::pumpkin_block::PumpkinBlock,
    server::Server,
    world::World,
};

#[pumpkin_block("minecraft:torch")]
pub struct TorchBlock;
#[pumpkin_block("minecraft:redstone_torch")]
pub struct RedstoneTorchBlock;
#[pumpkin_block("minecraft:soul_torch")]
pub struct SoulTorchBlock;
#[async_trait]
impl PumpkinBlock for TorchBlock {
    async fn on_place(&self, _server: &Server, _world: &World, block: &Block, _face: &BlockDirection, _block_pos: &BlockPos, _use_item_on: &SUseItemOn, _player_direction: &HorizontalFacing, _other: bool) -> u16 {
        let wall_block = Block::WALL_TORCH;
        let mut wall_block_properties = WallTorchLikeProperties::default(&wall_block);
        match _face {
            BlockDirection::North | BlockDirection::South | BlockDirection::West | BlockDirection::East => {
                wall_block_properties.facing = _face.to_cardinal_direction().opposite();
                return wall_block_properties.to_state_id(&wall_block);
            }
            BlockDirection::Up => {
                let mut possible_directions = vec![];
                for bd in BlockDirection::horizontal(){
                    let block_offset = bd.to_offset();
                    let base_block_pos = _block_pos.offset(block_offset);
                    let base_block = _world.get_block(&base_block_pos).await.unwrap();
                    if base_block.id != 0{
                        possible_directions.push(bd.to_cardinal_direction());
                    }
                }

                return if possible_directions.contains(&_player_direction) {
                    wall_block_properties.facing = _player_direction.opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else if possible_directions.len() > 0 {
                    wall_block_properties.facing = possible_directions[0].opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else {
                    0
                }
            }
            _ => {}
        }

        block.default_state_id
    }
}
#[async_trait]
impl PumpkinBlock for RedstoneTorchBlock {
    async fn on_place(&self, _server: &Server, _world: &World, block: &Block, _face: &BlockDirection, _block_pos: &BlockPos, _use_item_on: &SUseItemOn, _player_direction: &HorizontalFacing, _other: bool) -> u16 {
        let wall_block = Block::REDSTONE_WALL_TORCH;
        let mut wall_block_properties = FurnaceLikeProperties::default(&wall_block);
        match _face {
            BlockDirection::North | BlockDirection::South | BlockDirection::West | BlockDirection::East => {
                wall_block_properties.facing = _face.to_cardinal_direction().opposite();
                return wall_block_properties.to_state_id(&wall_block);
            }
            BlockDirection::Up => {
                let mut possible_directions = vec![];
                for bd in BlockDirection::horizontal(){
                    let block_offset = bd.to_offset();
                    let base_block_pos = _block_pos.offset(block_offset);
                    let base_block = _world.get_block(&base_block_pos).await.unwrap();
                    if base_block.id != 0{
                        possible_directions.push(bd.to_cardinal_direction());
                    }
                }

                return if possible_directions.contains(&_player_direction) {
                    wall_block_properties.facing = _player_direction.opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else if possible_directions.len() > 0 {
                    wall_block_properties.facing = possible_directions[0].opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else {
                    0
                }
            }
            _ => {}
        }

        block.default_state_id
    }
}

#[async_trait]
impl PumpkinBlock for SoulTorchBlock {
    async fn on_place(&self, _server: &Server, _world: &World, block: &Block, _face: &BlockDirection, _block_pos: &BlockPos, _use_item_on: &SUseItemOn, _player_direction: &HorizontalFacing, _other: bool) -> u16 {
        let wall_block = Block::SOUL_WALL_TORCH;
        let mut wall_block_properties = WallTorchLikeProperties::default(&wall_block);
        match _face {
            BlockDirection::North | BlockDirection::South | BlockDirection::West | BlockDirection::East => {
                wall_block_properties.facing = _face.to_cardinal_direction().opposite();
                return wall_block_properties.to_state_id(&wall_block);
            }
            BlockDirection::Up => {
                let mut possible_directions = vec![];
                for bd in BlockDirection::horizontal(){
                    let block_offset = bd.to_offset();
                    let base_block_pos = _block_pos.offset(block_offset);
                    let base_block = _world.get_block(&base_block_pos).await.unwrap();
                    if base_block.id != 0{
                        possible_directions.push(bd.to_cardinal_direction());
                    }
                }

                return if possible_directions.contains(&_player_direction) {
                    wall_block_properties.facing = _player_direction.opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else if possible_directions.len() > 0 {
                    wall_block_properties.facing = possible_directions[0].opposite();
                    wall_block_properties.to_state_id(&wall_block)
                } else {
                    0
                }
            }
            _ => {}
        }

        block.default_state_id
    }
}



