#[derive(Copy, Clone)]
pub enum TileSlot {
    Wall,
    Grass,
    Empty
}

impl TileSlot {
    pub fn texture_index(&self) -> usize {
        match self {
            TileSlot::Wall => {
                0
            },
            TileSlot::Grass => {
                1
            },
            TileSlot::Empty => {
                2
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
                tile.add_tile_slot(TileSlot::Wall);
            },
            1 => {
                tile.add_tile_slot(TileSlot::Grass);
            },
            2 => {
                tile.add_tile_slot(TileSlot::Empty);
            },
            _=> {
                tile.add_tile_slot(TileSlot::Empty);
            }
        }

        return tile;
    }

}