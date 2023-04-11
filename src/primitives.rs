#[derive(Debug, Clone)]
pub struct Rectangle {
    bounds: [f64; 4],
    idx: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Point {
    coords: [f64; 2],
    idx: Option<usize>,
}

pub(crate) trait Geometric {
    fn bounding_box(&self) -> Rectangle;
    fn get_idx(&self) -> Option<usize>;
}

impl Rectangle {
    pub(crate) fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64, idx: Option<usize>) -> Self {
        let mut x_bounds = vec![x_min, x_max];
        let mut y_bounds = vec![y_min, y_max];
        x_bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y_bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Rectangle { bounds: [x_bounds[0], y_bounds[0], x_bounds[1], y_bounds[1]], idx: idx }
    }

    pub(crate) fn bounds(&self) -> &[f64; 4] {
        return &self.bounds;
    }

    pub(crate) fn contains(&self, other: &impl Geometric) -> bool {
        let other_bbox = other.bounding_box();
        if self.bounds[0] > other_bbox.bounds[0] {
            return false;
        }
        if self.bounds[1] > other_bbox.bounds[1] {
            return false;
        }
        if self.bounds[2] < other_bbox.bounds[2] {
            return false;
        }
        if self.bounds[3] < other_bbox.bounds[3] {
            return false;
        }
        true
    }

    pub(crate) fn dbounds(&self, other: &impl Geometric) -> [f64; 4] {
        let other_bbox = other.bounding_box();
        let mut dbounds = [0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            dbounds[i] = self.bounds[i] - other_bbox.bounds[i];
        }
        dbounds
    }
}

impl Geometric for Rectangle {
    fn bounding_box(&self) -> Self {
        return self.clone();
    }
    fn get_idx(&self) -> Option<usize> {
        return self.idx.clone();
    }
}

impl Geometric for Point {
    fn bounding_box(&self) -> Rectangle {
        let box_size = 1.0 / 3600.0;
        Rectangle::new(
            self.coords[0] - box_size,
            self.coords[1] - box_size,
            self.coords[2] + box_size,
            self.coords[3] + box_size,
            self.get_idx()
        )
    }
    fn get_idx(&self) -> Option<usize> {
        return self.idx.clone();
    }
}