pub type Position = (i64, i64);
const PI: f64 = 3.14159265358979323;

#[derive(Debug, Clone)]
pub struct Component {
    pub m00: f64,
    pub m01: f64,
    pub m10: f64,
    pub m11: f64,
    pub m02: f64,
    pub m20: f64,
    pub rad: f64,
    pub theta: f64,
    pub vec: (f64, f64),
    pub area: Vec<Position>,
    pub contour: Vec<Position>,
    pub center: Position,

    pub x_max: Position,
    pub x_min: Position,
    pub y_max: Position,
    pub y_min: Position,
    pub ratio: f64,

    pub max_value: i64,
    pub max_position: Position,
    pub min_value: i64,
    pub min_position: Position,
}

impl Component {
    pub fn new() -> Component {
        Component {
            m00: 0.0,
            m01: 0.0,
            m10: 0.0,
            m11: 0.0,
            m02: 0.0,
            m20: 0.0,
            rad: 0.0,
            theta: 0.0,
            x_max: (0, 0),
            x_min: (10000, 10000),
            y_max: (0, 0),
            y_min: (10000, 10000),
            ratio: 0.0,
            max_value: 0,
            max_position: (0, 0),
            min_value: 10000,
            min_position: (10000, 10000),
            area: Vec::new(),
            contour: Vec::new(),
            vec: (0.0, 0.0),
            center: (0, 0),
        }
    }

    pub fn get_vec_x(&self) -> f64 {
        self.vec.0
    }

    pub fn get_vec_y(&self) -> f64 {
        self.vec.1
    }

    pub fn update(&mut self, position: Position) {
        self.m00 += 1.0;
        self.m10 += position.0 as f64;
        self.m01 += position.1 as f64;
        self.m11 += (position.0 + position.1) as f64;
        self.m20 += (position.0.pow(2)) as f64;
        self.m02 += (position.1.pow(2)) as f64;

        if position.0 >= self.x_max.0 {
            self.x_max = position;
        }
        if position.0 <= self.x_min.0 {
            self.x_min = position;
        }
        if position.1 >= self.y_max.1 {
            self.y_max = position;
        }
        if position.1 <= self.y_min.1 {
            self.y_min = position;
        }
        self.area.push(position);
    }

    pub fn update_by_contour(&mut self, position: Position) {
        self.update(position);
        self.contour.push(position);

        if self.max_value < position.0 + position.1 {
            self.max_value = position.0 + position.1;
            self.max_position = position;
        }
        if self.min_value > position.0 + position.1 {
            self.min_value = position.0 + position.1;
            self.min_position = position;
        }
    }

    pub fn moment(&mut self) {
        if self.m00 == 0.0 {
            return;
        }

        // 重点
        let cx = self.m10 / self.m00;
        let cy = self.m01 / self.m00;
        self.center = (cx as i64, cy as i64);

        let mu20 = self.m20 / self.m00 - cx.powi(2);
        let mu02 = self.m02 / self.m00 - cy.powi(2);
        let mu11 = self.m11 / self.m00 - cx * cy;

        // theta
        self.rad = 0.5 * ((2.0 * mu11) / (mu20 - mu02)).atan();
        if self.rad < 0.0 {
            self.rad = PI + self.rad;
        }
        self.theta = self.rad * 180.0 / PI;
        self.vec = (self.rad.cos(), self.rad.sin());

        // hypotenuse
        let bottom =
            ((self.x_max.0 - self.y_min.0).pow(2) + (self.x_max.1 - self.y_min.1).pow(2)) as f64;
        let bottom = bottom.sqrt();
        let height =
            ((self.x_max.0 - self.y_max.0).pow(2) + (self.y_max.1 - self.x_max.1).pow(2)) as f64;
        let height = height.sqrt();

        let ratio = if bottom > height {
            bottom / height
        } else {
            height / bottom
        };

        self.ratio = ratio;
    }
}
