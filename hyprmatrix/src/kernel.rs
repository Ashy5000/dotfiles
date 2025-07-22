use crate::{random_char, Tile, TileType};
use rand::random_bool;

pub(crate) fn should_sink(a: f64, b: f64) -> bool {
    if a < b {
        false
    } else {
        random_bool((a - b) / 5.0)
    }
}

pub(crate) fn kernel(ll_p: Tile, lr_p: Tile, ul_p: Tile, ur_p: Tile) -> (Tile, Tile, Tile, Tile) {
    let mut ll = ll_p;
    let mut lr = lr_p;
    let mut ul = ul_p;
    let mut ur = ur_p;
    if (ll.empty() && ul.falls())
        || (ll.real() && ul.real() && should_sink(ul.density(), ll.density()))
    {
        let tmp = ll;
        ll = ul;
        ul = tmp;
    }
    if ll.tile_type == TileType::Sand && ul.tile_type == TileType::Water {
        ll.tile_type = TileType::WetSand;
        ul.tile_type = TileType::Empty;
    }
    if ll.tile_type == TileType::Water && lr.tile_type == TileType::Sand && random_bool(0.2) {
        lr.tile_type = TileType::WetSand;
        ll.tile_type = TileType::Empty
    }
    if lr.tile_type == TileType::Water && ll.tile_type == TileType::Sand && random_bool(0.2) {
        ll.tile_type = TileType::WetSand;
        lr.tile_type = TileType::Empty
    }
    if ll.tile_type == TileType::Magma && ul.tile_type == TileType::WetSand {
        ul.tile_type = TileType::Sand;
    }
    if ll.tile_type == TileType::Magma && ul.tile_type == TileType::Sand {
        ul.tile_type = TileType::Stone;
    }
    if ll.tile_type == TileType::WetSand && lr.tile_type == TileType::Empty && random_bool(0.2) {
        ll.tile_type = TileType::Sand;
        lr.tile_type = TileType::Water;
        lr.value = random_char();
    }
    if lr.tile_type == TileType::WetSand && ll.tile_type == TileType::Empty && random_bool(0.2) {
        lr.tile_type = TileType::Sand;
        ll.tile_type = TileType::Water;
        ll.value = random_char();
    }
    if random_bool((1.0 - ll.viscosity()) * (1.0 - lr.viscosity()) * 0.2) {
        let tmp = ll;
        ll = lr;
        lr = tmp;
    }
    if ll.slides() && ul.slides() && (lr.empty() || should_sink(ul.density(), lr.density())) {
        let tmp = lr;
        lr = ul;
        ul = tmp;
    }
    if lr.slides() && ur.slides() && (ll.empty() || should_sink(ur.density(), ll.density())) {
        let tmp = ll;
        ll = ur;
        ur = tmp;
    }
    (ll, lr, ul, ur)
}
