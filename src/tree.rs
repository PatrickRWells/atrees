use crate::primitives::Rectangle;
use crate::primitives::Geometric;
use std::rc::Rc;

struct ATree {
    geos: Vec<Rc<dyn Geometric>>,
    nodes: Vec<Node>,
    region: Rectangle,
    nper_leaf: usize
}

struct Node {
    leafs: Vec<Leaf>,
    pub region: Rectangle
}

struct Leaf {
    geos: Vec<Rectangle>,
    pub region: Rectangle
}

impl ATree {

    pub fn new_from_geometries(geos: Vec<Rc<dyn Geometric>>, region: Rectangle, nper_leaf: Option<usize>) -> ATree {
        let mut nper_leaf_ = 1000;
        match nper_leaf {
            Option::Some(val) => nper_leaf_ = val,
            _ => (),

        }
        let mut tree = ATree {geos: geos, region: region, nper_leaf: nper_leaf_, nodes: Vec::new()};
        tree.pack();
        tree
    }

    pub fn search(&self, region: impl Geometric) -> Option<Vec<Rc<dyn Geometric>>> {
        let bbox = region.bounding_box();
        let mut search_bboxes: Vec<&Rectangle> = Vec::new();
        let overlaps = self.nodes
                            .iter()
                            .filter_map(|node| node.search(&bbox));
        for mut o in overlaps {
            search_bboxes.append(&mut o)
        }
        None
                            
    }


    fn pack(&mut self) -> () {

        let mut bboxes: Vec<Rectangle> = self.geos
                            .iter()
                            .map(|geo| geo.bounding_box())
                            .collect();
        bboxes.sort_by(|b1, b2| b1.center().x().partial_cmp(&b2.center().x()).unwrap());
        let slices = (bboxes.len() as f64 / self.nper_leaf as f64).sqrt().floor() as usize + 1;
        let nper_slice = bboxes.len() / slices + 1;
        for i in 0..slices {
            let start =  i*nper_slice;
            let mut end = (i+1) * nper_slice;
            if i == bboxes.len() {
                end = bboxes.len() - 1
            }
            let slice_node = self.pack_slice(&mut bboxes[start..end]);
            self.nodes.push(slice_node)
        }
        ()
    }

    fn pack_slice(&mut self, slice: &mut [Rectangle]) -> Node {
        let x_min = slice[0].center().x();
        let x_max = slice[slice.len() - 1].center().x();
        slice.sort_by(|b1, b2| b1.center().y().partial_cmp(&b2.center().y()).unwrap());
        let node_ymin = slice[0].center().y();
        let node_ymax = slice[slice.len()].center().y();
        let slice_geometry = Rectangle::new(x_min, node_ymin, x_max, node_ymax, None);
        let nnodes = (slice.len() as f64 / self.nper_leaf as f64).floor() as usize + 1;
        let nper_node = slice.len() / nnodes + 1;
        let mut leaves: Vec<Leaf> = Vec::new();
        for i in 0..nnodes {
            let start =  i*nper_node;
            let mut end = (i+1) * nper_node;
            if i == slice.len() {
                end = slice.len() - 1
            }
            let geos = &slice[start..end].to_vec();
            let leaf_geometry = Rectangle::new(x_min, geos[0].center().y() , x_max, geos[geos.len() - 1].center().y(),  None);
            let leaf = Leaf {geos: geos.to_vec(), region: leaf_geometry};
            leaves.push(leaf)
        }
        Node {leafs: leaves, region: Rectangle::new(x_min, node_ymin, x_max, node_ymax, None)}
    }
}

impl Node {
    fn search(&self, other: &Rectangle) -> Option<Vec<&Rectangle>> {
        if !self.region.overlaps(other) {
            return None
        }
        None

    }
}

impl Leaf {
    fn search(&self, other: &Rectangle) -> Option<Vec<&Rectangle>> {
        None
    }
}