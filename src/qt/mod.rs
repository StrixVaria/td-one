use std::ops::{Deref, DerefMut};

const SPLIT_SIZE: usize = 10;

#[derive(Debug)]
pub struct QuadTree<T: HasRegion + Copy> {
    contents: Vec<T>,
    split: bool,
    children: Option<Box<[QuadTree<T>; 4]>>,
    bounds: RectangleData,
}

impl<T: HasRegion + Copy> QuadTree<T> {
    pub fn new(bounds: RectangleData) -> Self {
        QuadTree {
            contents: vec![],
            split: false,
            children: None,
            bounds,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.split {
            self.insert_children(value);
        } else if self.contents.len() >= SPLIT_SIZE {
            self.split();
            self.insert_children(value);
        } else {
            if self.bounds.intersects(&value) {
                self.contents.push(value);
            }
        }
    }

    pub fn query(&self, region: &Region) -> Vec<T> {
        let mut found: Vec<T> = vec![];
        if self.split {
            let children = self.children.as_ref().unwrap().deref();
            for i in 0..4 {
                for child_item in children[i].query(region) {
                    found.push(child_item);
                }
            }
        } else if self.bounds.intersects(region) {
            for item in self.contents.iter() {
                if region.intersects(item) {
                    found.push(*item);
                }
            }
        }
        found
    }

    fn split(&mut self) {
        let (q1, q2, q3, q4) = self.child_coords();
        self.children = Some(Box::new([
            QuadTree::new(q1),
            QuadTree::new(q2),
            QuadTree::new(q3),
            QuadTree::new(q4),
        ]));
        self.split = true;
        let contents_copy = self.contents.clone();
        for item in contents_copy {
            self.insert_children(item);
        }
        self.contents.clear();
    }

    fn insert_children(&mut self, value: T) {
        if self.children.is_none() {
            return;
        }
        let children = self.children.as_mut().unwrap().deref_mut();
        for i in 0..4 {
            children[i].insert(value);
        }
    }

    // Order: Top-right, top-left, bottom-left, bottom-right
    fn child_coords(&self) -> (RectangleData, RectangleData, RectangleData, RectangleData) {
        let x = self.bounds.x;
        let y = self.bounds.y;
        let half_width = self.bounds.w / 2.0;
        let half_height = self.bounds.h / 2.0;
        (
            RectangleData::new(x + half_width, y, half_width, half_height),
            RectangleData::new(x, y, half_width, half_height),
            RectangleData::new(x, y + half_height, half_width, half_height),
            RectangleData::new(x + half_width, y + half_height, half_width, half_height),
        )
    }
}

pub trait HasRegion {
    fn get_region(&self) -> Region;

    fn intersects<H: HasRegion>(&self, h: &H) -> bool
    where
        Self: std::marker::Sized,
    {
        h.get_region().intersects(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointData {
    x: f64,
    y: f64,
}

impl HasRegion for PointData {
    fn get_region(&self) -> Region {
        Region::Point(self.clone())
    }
}

impl PointData {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectangleData {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl HasRegion for RectangleData {
    fn get_region(&self) -> Region {
        Region::Rectangle(self.clone())
    }
}

impl RectangleData {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircleData {
    x: f64,
    y: f64,
    r: f64,
}

impl HasRegion for CircleData {
    fn get_region(&self) -> Region {
        Region::Circle(self.clone())
    }
}

impl CircleData {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self { x, y, r }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Region {
    Point(PointData),
    Rectangle(RectangleData),
    Circle(CircleData),
}

impl HasRegion for Region {
    fn get_region(&self) -> Region {
        self.clone()
    }
}

impl Region {
    pub fn intersects<H: HasRegion>(&self, h: &H) -> bool {
        let other = h.get_region();
        match self {
            Region::Point(self_data) => match other {
                Region::Point(other_data) => math::point_intersects_point(self_data, &other_data),
                Region::Rectangle(other_data) => {
                    math::point_intersects_rectangle(self_data, &other_data)
                }
                Region::Circle(other_data) => math::point_intersects_circle(self_data, &other_data),
            },
            Region::Rectangle(self_data) => match other {
                Region::Point(other_data) => {
                    math::rectangle_intersects_point(self_data, &other_data)
                }
                Region::Rectangle(other_data) => {
                    math::rectangle_intersects_rectangle(self_data, &other_data)
                }
                Region::Circle(other_data) => {
                    math::rectangle_intersects_circle(self_data, &other_data)
                }
            },
            Region::Circle(self_data) => match other {
                Region::Point(other_data) => math::circle_intersects_point(self_data, &other_data),
                Region::Rectangle(other_data) => {
                    math::circle_intersects_rectangle(self_data, &other_data)
                }
                Region::Circle(other_data) => {
                    math::circle_intersects_circle(self_data, &other_data)
                }
            },
        }
    }

    pub fn new_rect(x: f64, y: f64, w: f64, h: f64) -> Self {
        Region::Rectangle(RectangleData::new(x, y, w, h))
    }

    pub fn new_point(x: f64, y: f64) -> Self {
        Region::Point(PointData::new(x, y))
    }

    pub fn new_circle(x: f64, y: f64, r: f64) -> Self {
        Region::Circle(CircleData::new(x, y, r))
    }
}

mod math {
    use super::{CircleData, PointData, RectangleData};

    pub fn approx_eq(v1: f64, v2: f64) -> bool {
        const POINT_DELTA: f64 = 0.001;
        v1 - POINT_DELTA < v2 && v1 + POINT_DELTA > v2
    }

    pub fn point_intersects_point(p1: &PointData, p2: &PointData) -> bool {
        approx_eq(p1.x, p2.x) && approx_eq(p1.y, p2.y)
    }

    pub fn circle_intersects_circle(c1: &CircleData, c2: &CircleData) -> bool {
        // sqrt is slower than pow so go that way instead of the other way to compare
        (c2.x - c1.x).powi(2) + (c2.y - c1.y).powi(2) <= (c1.r + c2.r).powi(2)
    }

    pub fn rectangle_intersects_rectangle(r1: &RectangleData, r2: &RectangleData) -> bool {
        !(r1.x + r1.w < r2.x || r2.x + r2.w < r1.x || r1.y + r1.h < r2.y || r2.y + r2.h < r1.y)
    }

    pub fn point_intersects_circle(p: &PointData, c: &CircleData) -> bool {
        // sqrt is slower than pow so go that way instead of the other way to compare
        (p.x - c.x).powi(2) + (p.y - c.y).powi(2) <= c.r.powi(2)
    }

    pub fn circle_intersects_point(c: &CircleData, p: &PointData) -> bool {
        point_intersects_circle(p, c)
    }

    pub fn point_intersects_rectangle(p: &PointData, r: &RectangleData) -> bool {
        p.x >= r.x && p.y >= r.y && p.x <= r.x + r.w && p.y <= r.y + r.h
    }

    pub fn rectangle_intersects_point(r: &RectangleData, p: &PointData) -> bool {
        point_intersects_rectangle(p, r)
    }

    pub fn rectangle_intersects_circle(r: &RectangleData, c: &CircleData) -> bool {
        // Centerpoint of the rectangle
        let (r_mid_x, r_mid_y) = ((r.x + r.x + r.w) / 2.0, (r.y + r.y + r.h) / 2.0);
        // Distance between center of the rectangle and center of the circle
        let (dx, dy) = ((c.x - r_mid_x).abs(), (c.y - r_mid_y).abs());

        let (half_width, half_height) = (r.w / 2.0, r.h / 2.0);

        // If the center of the circle is further away from the center of the
        // rectangle than the rectangle's dimension in that axis plus the
        // circle's radius, they definitely don't intersect.
        if dx > half_width + c.r || dy > half_height + c.r {
            return false;
        }

        // Given the check above, if the distance away from the center of the
        // rectangle is less than the dimension in that axis, it's now
        // guaranteed to be intersecting.
        if dx <= half_width || dy <= half_height {
            return true;
        }

        // If we haven't returned yet, it's the hard case where the circle may
        // just intersect the rectangle's corner, so do that check.
        (dx - half_width).powi(2) + (dy - half_height).powi(2) <= c.r.powi(2)
    }

    pub fn circle_intersects_rectangle(c: &CircleData, r: &RectangleData) -> bool {
        rectangle_intersects_circle(r, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Place {
        index: usize,
        region: Region,
    }

    impl Place {
        pub fn point(i: usize, x: f64, y: f64) -> Self {
            Place {
                index: i,
                region: Region::Point(PointData::new(x, y)),
            }
        }

        pub fn rect(i: usize, x: f64, y: f64, w: f64, h: f64) -> Self {
            Place {
                index: i,
                region: Region::Rectangle(RectangleData::new(x, y, w, h)),
            }
        }

        pub fn circle(i: usize, x: f64, y: f64, r: f64) -> Self {
            Place {
                index: i,
                region: Region::Circle(CircleData::new(x, y, r)),
            }
        }
    }

    impl HasRegion for Place {
        fn get_region(&self) -> Region {
            self.region.clone()
        }
    }

    impl HasRegion for &Place {
        fn get_region(&self) -> Region {
            self.region.clone()
        }
    }

    #[test]
    fn test_quadtree_insert() {
        let mut qt = QuadTree::new(RectangleData::new(0.0, 0.0, 10.0, 10.0));
        let (elem1, elem2) = (Place::point(0, 2.0, 4.0), Place::point(1, 3.0, 6.0));
        qt.insert(&elem1);
        qt.insert(&elem2);
        assert_eq!(qt.contents.len(), 2);
    }

    #[test]
    fn test_basic_query() {
        let mut qt = QuadTree::new(RectangleData::new(0.0, 0.0, 10.0, 10.0));
        let elem = Place::point(0, 5.0, 5.0);
        qt.insert(&elem);
        let results = qt.query(&Region::new_circle(4.0, 4.0, 2.0));
        println!("Basic results: {:?}", results);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_intermediate_query() {
        let mut qt = QuadTree::new(RectangleData::new(0.0, 0.0, 10.0, 10.0));
        let p = Place::point(0, 4.0, 5.0);
        let c = Place::circle(1, 6.0, 4.0, 2.0);
        let r = Place::rect(2, 4.5, 4.5, 1.0, 1.0);
        qt.insert(&p);
        qt.insert(&c);
        qt.insert(&r);
        let results = qt.query(&Region::Rectangle(RectangleData::new(4.5, 0.0, 5.0, 10.0)));
        println!("Intermediate results: {:?}", results);
        assert_eq!(results.len(), 2);
        for result in results.iter() {
            assert!(result.index == 1 || result.index == 2);
            assert_ne!(result.index, 0);
        }
    }

    #[test]
    fn test_insert_to_split() {
        let mut qt = QuadTree::new(RectangleData::new(0.0, 0.0, 100.0, 100.0));
        let mut values = vec![];
        for i in 0..15 {
            let val = i as f64;
            values.push(Place::point(i, 3.0 * val, 6.0 * val));
        }
        for i in 0..15 {
            qt.insert(&values[i]);
        }
        println!("Split Quadtree: {:?}", qt);
        // If we split then we've distributed all the elements to the children.
        assert_eq!(qt.contents.len(), 0);
    }

    #[test]
    fn test_advanced_query() {
        let mut qt = QuadTree::new(RectangleData::new(0.0, 0.0, 100.0, 100.0));
        let mut values: Vec<Place> = vec![];
        let p1 = Place::point(100, 75.0, 25.0);
        let p2 = Place::point(101, 40.0, 60.0);
        values.push(p1);
        values.push(p2);
        for i in 0..15 {
            let val = i as f64;
            let index = i * 3;
            values.push(Place::point(index, val * 3.0, val * 4.0));
            values.push(Place::circle(index + 1, 100.0 - val * 3.0, 17.0, val));
            values.push(Place::rect(
                index + 2,
                val * 2.0,
                val * 2.0,
                val * 3.0,
                val * 0.5,
            ));
        }
        for val in values.iter() {
            qt.insert(val);
        }
        println!("Massive Quadtree: {:?}", qt);
        let results = qt.query(&Region::new_circle(50.0, 50.0, 25.0));
        println!("Massive Results: {:?}", results);
        let mut found = false;
        for result in results {
            if result.index == 101 {
                found = true;
            }
            assert_ne!(result.index, 100);
        }
        assert!(found);
    }

    #[test]
    fn test_approx_eq() {
        let (f1, f2) = (1.5, 1.5);
        assert!(math::approx_eq(f1, f2));
        let f3 = 1.51;
        assert!(!math::approx_eq(f1, f3));
        let f4 = 1.50001;
        assert!(math::approx_eq(f1, f4));
    }

    #[test]
    fn test_point_intersects_point() {
        let p1 = PointData::new(3.0, 5.0);
        let p2 = PointData::new(3.000001, 4.99999);
        assert!(math::point_intersects_point(&p1, &p2));
        let p3 = PointData::new(4.0, 5.0);
        assert!(!math::point_intersects_point(&p1, &p3));
    }

    #[test]
    fn test_point_intersects_circle() {
        let p1 = PointData::new(3.0, 3.0);
        let c = CircleData::new(0.0, 0.0, 5.0);
        assert!(math::point_intersects_circle(&p1, &c));
        assert!(math::circle_intersects_point(&c, &p1));
        let p2 = PointData::new(4.0, 4.0);
        assert!(!math::point_intersects_circle(&p2, &c));
        assert!(!math::circle_intersects_point(&c, &p2));
    }

    #[test]
    fn test_point_intersects_rectangle() {
        let p1 = PointData::new(3.0, 4.0);
        let r = RectangleData::new(2.0, 2.0, 4.0, 4.0);
        assert!(math::point_intersects_rectangle(&p1, &r));
        assert!(math::rectangle_intersects_point(&r, &p1));
        let p2 = PointData::new(1.0, 0.0);
        assert!(!math::point_intersects_rectangle(&p2, &r));
        assert!(!math::rectangle_intersects_point(&r, &p2));
    }

    #[test]
    fn test_rectangle_intersects_rectangle() {
        let control = RectangleData::new(0.0, 0.0, 5.0, 4.0);
        let overlap = RectangleData::new(3.0, 3.0, 5.0, 4.0);
        assert!(math::rectangle_intersects_rectangle(&control, &overlap));
        assert!(math::rectangle_intersects_rectangle(&overlap, &control));
        let inside = RectangleData::new(1.0, 1.0, 3.0, 2.0);
        assert!(math::rectangle_intersects_rectangle(&control, &inside));
        assert!(math::rectangle_intersects_rectangle(&inside, &control));
        let outside = RectangleData::new(-1.0, -2.0, 15.0, 18.0);
        assert!(math::rectangle_intersects_rectangle(&outside, &control));
        assert!(math::rectangle_intersects_rectangle(&control, &outside));
        let left = RectangleData::new(-5.0, 0.0, 4.0, 3.0);
        assert!(!math::rectangle_intersects_rectangle(&control, &left));
        assert!(!math::rectangle_intersects_rectangle(&left, &control));
        let right = RectangleData::new(6.0, 0.0, 4.0, 3.0);
        assert!(!math::rectangle_intersects_rectangle(&control, &right));
        assert!(!math::rectangle_intersects_rectangle(&right, &control));
        let above = RectangleData::new(0.0, -5.0, 4.0, 4.0);
        assert!(!math::rectangle_intersects_rectangle(&control, &above));
        assert!(!math::rectangle_intersects_rectangle(&above, &control));
        let below = RectangleData::new(0.0, 5.0, 100.0, 81.38374);
        assert!(!math::rectangle_intersects_rectangle(&control, &below));
        assert!(!math::rectangle_intersects_rectangle(&below, &control));
    }

    #[test]
    fn test_circle_intersects_circle() {
        let c1 = CircleData::new(3.0, 4.0, 5.0);
        let c2 = CircleData::new(-3.0, -4.0, 6.0);
        assert!(math::circle_intersects_circle(&c1, &c2));
        let c3 = CircleData::new(15.0, 15.0, 1.0);
        assert!(!math::circle_intersects_circle(&c1, &c3));
    }

    #[test]
    fn test_rectangle_intersects_circle() {
        let control = RectangleData::new(0.0, 0.0, 10.0, 5.0);
        let overlap_corner = CircleData::new(-1.0, -1.0, 2f64.sqrt() + 0.01);
        assert!(math::rectangle_intersects_circle(&control, &overlap_corner));
        assert!(math::circle_intersects_rectangle(&overlap_corner, &control));
        let overlap_side = CircleData::new(-1.0, 2.5, 1.01);
        assert!(math::rectangle_intersects_circle(&control, &overlap_side));
        assert!(math::circle_intersects_rectangle(&overlap_side, &control));
        let inside = CircleData::new(5.0, 2.5, 1.0);
        assert!(math::rectangle_intersects_circle(&control, &inside));
        assert!(math::circle_intersects_rectangle(&inside, &control));
        let outside = CircleData::new(5.0, 2.0, 100.0);
        assert!(math::rectangle_intersects_circle(&control, &outside));
        assert!(math::circle_intersects_rectangle(&outside, &control));
    }
}
