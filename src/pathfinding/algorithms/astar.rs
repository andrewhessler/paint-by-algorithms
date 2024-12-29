use std::{collections::BinaryHeap, isize};

use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{PathfindingEventType, PathfindingNode},
};

use super::node::Node;

pub fn setup_and_run_astar(
    grid: &[&Tile],
    current_tile_id: usize,
    is_aggressive: bool,
) -> Vec<PathfindingNode> {
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);

    // This looks weird, probably want it dynamic some day.
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];

    for tile in grid {
        if tile.tile_type == TileType::End {
            end_tile_pos = Some((tile.row, tile.col));
        }

        if tile.id == current_tile_id {
            current_tile_pos = (tile.row, tile.col);
        }

        let row = tile.row as usize;
        let col = tile.col as usize;
        nodes[row][col].from_tile(tile);

        if tile.tile_type == TileType::Wall {
            nodes[row][col].visited = true;
            nodes[row][col].is_wall = true;
        }
    }

    return astar(nodes, current_tile_pos, end_tile_pos, is_aggressive);
}

fn astar(
    mut nodes: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    is_aggressive: bool,
) -> Vec<PathfindingNode> {
    let mut heap = BinaryHeap::new();
    let mut event_order = vec![];
    heap.push(Node {
        distance: 0,
        ..nodes[current_tile_pos.0][current_tile_pos.1]
    });

    let directions = [
        (-1, -1),
        (1, -1),
        (1, 1),
        (-1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];
    let end_pos = end_tile_pos.unwrap_or((0, 0));

    while let Some(mut node) = heap.pop() {
        if node.visited == true || node.is_wall {
            continue;
        }

        if (node.row, node.col) == end_pos {
            break;
        }

        node.visited = true;
        event_order.push(PathfindingNode {
            tile_id: node.tile_id,
            event_type: PathfindingEventType::Visited,
        });

        for (row_offset, col_offset) in directions {
            let visit_row = ((node.row + ROW_COUNT) as isize + row_offset) as usize % ROW_COUNT; // add row count to avoid negative index >.> <.<
            let visit_col = ((node.col + COL_COUNT) as isize + col_offset) as usize % COL_COUNT;

            if nodes[visit_row][visit_col].is_wall {
                continue;
            }

            let mut directional_distance = if row_offset.abs() + col_offset.abs() == 2 {
                14
            } else {
                10
            };

            let mut dx = end_pos.0 as isize - visit_row as isize;
            let mut dy = end_pos.1 as isize - visit_col as isize;
            if dx.abs() > COL_COUNT as isize / 2 {
                dx = COL_COUNT as isize - dx.abs();
            }
            if dy.abs() > ROW_COUNT as isize / 2 {
                dy = ROW_COUNT as isize - dy.abs();
            }
            let distance_between_checked_and_end = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();

            directional_distance += distance_between_checked_and_end as usize;

            let checked_node = &mut nodes[visit_row][visit_col];
            let new_distance = node.distance
                + if is_aggressive {
                    directional_distance.pow(10)
                } else {
                    directional_distance * 10
                };
            // event_order.push(PathfindingEvent {
            //     tile_id: checked_node.tile_id,
            //     event_type: PathfindingEventType::Checked,
            // });

            if new_distance < checked_node.distance {
                checked_node.distance = new_distance;
                checked_node.previous_node = Some((node.row, node.col));
                checked_node.visited = false;
                heap.push(Node { ..*checked_node });
            }
        }
    }
    return event_order;
}
