#[derive(Copy, Clone)]
pub enum TileSlot {
    Wall,
    Grass,
    TallGrass,
    LowGrass,
    Empty,
    Poster,
    Door,
    LeftHouseCorner,
    MainHouseFront,
    RightHouseCorner,
    LeftRiverBank,
    RightRiverBank,
    LeftRiverBankCorner,
    RightRiverBankCorner,
    TopRiverCenter,
    River,
    Fence
}

impl TileSlot {
    pub fn texture_index(&self) -> usize {
        match self {
            TileSlot::Grass => {
                0
            },
            TileSlot::Wall => {
                1
            },
            TileSlot::Fence => {
                2
            },
            TileSlot::TallGrass => {
                3
            },
            TileSlot::LowGrass => {
                4
            },
            TileSlot::Poster => {
                5
            },
            TileSlot::Door => {
                6
            },
            TileSlot::LeftHouseCorner => {
                7
            },
            TileSlot::MainHouseFront => {
                8
            },
            TileSlot::RightHouseCorner => {
                9
            },
            TileSlot::LeftRiverBank => {
                10
            },
            TileSlot::RightRiverBank => {
                11
            },
            TileSlot::LeftRiverBankCorner => {
                12
            },
            TileSlot::RightRiverBankCorner => {
                13
            },
            TileSlot::TopRiverCenter => {
                14
            },
            TileSlot::River => {
                15
            },
            TileSlot::Empty => {
                20
            }
        }
    }
}


#[derive(Clone, Copy)]
pub struct Tile {
    slots: [TileSlot; 5],
    number_of_tiles: usize
}

impl Tile {
    
    pub fn new() -> Tile {
        Tile {
            slots: [TileSlot::Empty; 5],
            number_of_tiles: 0
        }
    }

    pub fn add_tile_slot(&mut self, slot: TileSlot) -> bool {
        if self.number_of_tiles < 5 {

            self.slots[self.number_of_tiles] = slot;
            self.number_of_tiles += 1;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn get_top_tile_slot(&self) -> &TileSlot {
        if self.number_of_tiles == 0 {
            return &self.slots[0];
        }
        return &self.slots[self.number_of_tiles - 1];
    }

    pub fn remove_top_tile(&mut self) -> TileSlot {
        self.number_of_tiles -= 1;
        let removed_tiles = self.slots[self.number_of_tiles];
        self.slots[self.number_of_tiles] = TileSlot::Empty;
        return removed_tiles;
    }

    pub fn from_usize(index: usize) -> Tile {
        let mut tile = Tile::new();
        match index {
            0 => {
                tile.add_tile_slot(TileSlot::Grass);
            },
            1 => {
                tile.add_tile_slot(TileSlot::Fence);
            },
            2 => {
                tile.add_tile_slot(TileSlot::Wall);
            },
            3 => {
                tile.add_tile_slot(TileSlot::TallGrass);
            },
            4 => {
                tile.add_tile_slot(TileSlot::Wall);
            },
            5 => {
                tile.add_tile_slot(TileSlot::LowGrass);
            },
            6 => {
                tile.add_tile_slot(TileSlot::Poster);
            },
            7 => {
                tile.add_tile_slot(TileSlot::Door);
            },
            8 => {
                tile.add_tile_slot(TileSlot::LeftHouseCorner);
            },
            9 => {
                tile.add_tile_slot(TileSlot::MainHouseFront);
            },
            10 => {
                tile.add_tile_slot(TileSlot::RightHouseCorner);
            },
            11 => {
                tile.add_tile_slot(TileSlot::LeftRiverBank);
            },
            12 => {
                tile.add_tile_slot(TileSlot::RightRiverBank);
            },
            13 => {
                tile.add_tile_slot(TileSlot::LeftRiverBankCorner);
            },
            14 => {
                tile.add_tile_slot(TileSlot::RightRiverBankCorner);
            },
            15 => {
                tile.add_tile_slot(TileSlot::TopRiverCenter);
            },
            16 => {
                tile.add_tile_slot(TileSlot::River);
            }
            _ => {
                tile.add_tile_slot(TileSlot::Empty);
            }
        }

        return tile;
    }

}