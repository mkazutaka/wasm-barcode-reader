use crate::detector::component::Component;

pub type Position = (i64, i64);

#[derive(Clone)]
pub struct Cluster {
    pub threshold: f64,
    pub contour_size: usize,

    pub rad: f64,
    pub vec: (f64, f64),
    pub components: Vec<Component>,

    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
}

impl Cluster {
    pub fn new(threshold: f64) -> Cluster {
        Cluster {
            threshold,
            contour_size: 0,
            rad: 0.0,
            vec: (0.0, 0.0),
            components: Vec::new(),
            top_left: (1000, 1000),
            top_right: (0, 0),
            bottom_left: (1000, 1000),
            bottom_right: (0, 0),
        }
    }

    #[inline]
    pub fn check_fit(&self, component: &Component) -> bool {
        let dot = component.get_vec_x() * self.vec.0 + component.get_vec_y() * self.vec.1;
        let dot = dot.abs();
        if dot < self.threshold {
            return false;
        }

        let contour_len = component.contour.len();
        let contour_rate = if self.contour_size > contour_len {
            self.contour_size as f64 / contour_len as f64
        } else {
            contour_len as f64 / self.contour_size as f64
        };
        if contour_rate < 0.5 || 1.5 < contour_rate {
            return false;
        }

        for c in self.components.iter() {
            if ((component.center.0 - c.center.0).abs() < (c.contour.len() / 2) as i64)
                && ((component.center.1 - c.center.1).abs() < (c.contour.len() / 2) as i64)
            {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn update(&mut self, component: Component) {
        self.components.push(component);

        self.contour_size = 0;
        let mut sum = 0.0;
        let mut center_sum = (0, 0);
        for component in self.components.iter() {
            sum += component.rad;
            self.contour_size += component.contour.len();
            center_sum = (
                center_sum.0 + component.center.0,
                center_sum.1 + component.center.1,
            );

            // min_x_min_position
            if component.min_position.0 < self.top_left.0 {
                self.top_left = component.min_position
            }
            if component.min_position.0 > self.top_right.0 {
                self.top_right = component.min_position
            }
            if component.max_position.0 < self.bottom_left.0 {
                self.bottom_left = component.max_position
            }
            if component.max_position.0 > self.bottom_right.0 {
                self.bottom_right = component.max_position
            }
        }
        self.contour_size = self.contour_size / self.components.len();
        self.rad = sum / self.components.len() as f64;
        self.vec = (self.rad.cos(), self.rad.sin());
    }
}

impl Cluster {
    #[inline]
    pub fn center(&self) -> (Position, Position) {
        let a = (self.top_right.1 - self.top_left.1) as f64
            / (self.top_right.0 - self.top_left.0) as f64;

        // x = 10
        let x = 20;
        let y = (x as f64 * a) as i64;

        let left_x = (self.top_left.0 + self.bottom_left.0) / 2 - x;
        let left_y = (self.top_left.1 + self.bottom_left.1) / 2 - y;
        let right_x = (self.top_right.0 + self.bottom_right.0) / 2 + x;
        let right_y = (self.top_right.1 + self.bottom_right.1) / 2 + y;

        ((left_x, left_y), (right_x, right_y))
    }
}
