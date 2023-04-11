use crate::primitives::Rectangle;
use crate::primitives::Geometric;
use std::num;

pub struct Sky {
    region: Rectangle,
    tiles: Vec<Rectangle>,
    max_border: f64
}

impl Sky {

    pub(crate) fn new() -> Self {
        let reg = Rectangle::new(0.0, -90.0, 360.0, 90.0, None);
        let mut tiles:  Vec<Rectangle> = Vec::new();
        Sky {region: reg, tiles: tiles, max_border: 5.0}

    }

    pub fn tiles(&self) -> &Vec<Rectangle>{
        return &self.tiles
    }

    pub fn add_tile(&mut self, tile: Rectangle) -> bool {
        if !self.region.contains(&tile) {
            panic!("Tiles must be fully contained within the sky")
        }
        let reflection = self.reflect_tile(&tile);
        match reflection {
            Option::Some(mut obj) => self.tiles.append(&mut obj),
            _ => ()
        }
        self.tiles.push(tile);
        true
    
    }

    fn reflect_tile(&self, tile: &Rectangle) -> Option<Vec<Rectangle>> {
        let dbounds = self.region.dbounds(tile);
        let needs_reflection: Vec<bool> = dbounds
                                            .iter()
                                            .map(|db| db.abs() < self.max_border)
                                            .collect();
        if !needs_reflection.iter().any(|&val| val) {
            return None
        }
        let mut new_rectangles: Vec<Rectangle> = Vec::new();
        let mut new_bounds = [0.0, 0.0, 0.0, 0.0];
        for (i,&n) in needs_reflection.iter().enumerate() {
            if n {
                let reflection_bound = (i + 2) % 4;
                new_bounds[i] = self.region.bounds()[reflection_bound] - dbounds[i];
                new_bounds[reflection_bound] = new_bounds[i] - tile.bounds()[i] + tile.bounds()[reflection_bound];
                let obounda = (i+1)%4;
                let oboundb = (obounda + 2)%4;
                new_bounds[obounda] = tile.bounds()[obounda];
                new_bounds[oboundb] = tile.bounds()[oboundb];
                let new_rect = Rectangle::new(new_bounds[0], new_bounds[1], new_bounds[2], new_bounds[3], tile.get_idx());
                new_rectangles.push(new_rect)  
            }
        }
        let corner_reflections = self.reflect_tile_over_corners(&tile, &dbounds);
        match corner_reflections  {
            Option::Some(mut obj) => new_rectangles.append(&mut obj),
            _ => ()
        }
        return Option::Some(new_rectangles)
    }

    fn reflect_tile_over_corners(&self, tile: &Rectangle, dbounds: &[f64; 4]) -> Option<Vec<Rectangle>> {
        let mut needs_reflection = [false, false, false, false];
        let other_bounds = tile.bounds();
        let self_bounds = self.region.bounds();

        for (i, wd)  in self.region.bounds().windows(2).enumerate() {
                let sqd = (wd[0] - tile.bounds()[i]).powf(2.0) + (wd[1] - tile.bounds()[i+1]).powf(2.0);
                let distance = sqd.sqrt();
                needs_reflection[i] = distance < self.max_border;
            }
        let sqd = (self_bounds[0] - other_bounds[0]).powf(2.0) + (self_bounds[3] - other_bounds[3]).powf(2.0);
        let distance = sqd.sqrt();
        needs_reflection[3] = distance < self.max_border;

        if !needs_reflection.iter().any(|&val| val) {
            return None
        }
        
        let mut new_rectangles: Vec<Rectangle> = Vec::new();

        for (i, &nr)  in needs_reflection.iter().enumerate() {
            
            if nr {
                let mut new_bounds = [0.0, 0.0, 0.0, 0.0];
                let new_ref_x1 = self_bounds[(i+2) % 4];
                let new_ref_x2 = self_bounds[(i+3) % 4];
                let dx1 = dbounds[i];
                let dx2 = dbounds[(i+1)%4];
                new_bounds[i] = new_ref_x1 - dx1;
                new_bounds[(i+1) % 4] = new_ref_x2 - dx2;

                let dx1_rec = other_bounds[(i+2)%4] - other_bounds[i];
                let dx2_rec = other_bounds[(i+3)%4] - other_bounds[(i+1)%4];
                new_bounds[(i+2)%4] = new_bounds[i] + dx1_rec;
                new_bounds[(i+3)%4] = new_bounds[(i+1%4)] + dx2_rec;
                let rectangle = Rectangle::new(new_bounds[0], new_bounds[1], new_bounds[2], new_bounds[3], tile.get_idx());
                new_rectangles.push(rectangle)
            }
        }
        return Option::Some(new_rectangles)

    }
}
