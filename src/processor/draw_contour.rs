use crate::image::Image;
use crate::processor::Processor;
use crate::reader::{Color, Label};

pub type Position = (i64, i64);

impl Processor {
    pub fn draw_contour(image: &mut Image) -> Vec<Label> {
        Processor::threshold(image);
        Processor::draw_frame(image);

        let length: usize = image.length.into();
        let mut label_image = vec![Label::Unmarked; length];

        let height = image.height.into();
        let width = image.width.into();

        let mut last_label_group: u32 = 0;
        for y in 0..height {
            let mut last_label = Label::Unmarked;
            let mut last_color = Color::White;

            for x in 0..width {
                let current_position = ((y * width) + x) as usize;
                let current_color = Color::new(image.pixel(current_position));

                match label_image[current_position] {
                    Label::Unmarked => {
                        // 色の変更がない場合は、ラベルのみを変更する
                        if last_color == current_color {
                            // Do not Anything
                            match last_label {
                                // TODO 後で考える
                                Label::Marked(v) => {
                                    label_image[current_position] = Label::InnerMarked(v)
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // 色の変更がある場合
                        let mut tracer = Tracer::new((x as i64, y as i64));
                        match last_label {
                            // 色の変更があった最後のラベルがUNMarkedな場合、CurrentPositionは外部輪郭線上にいることになる
                            Label::Unmarked => {
                                // last_labelがUnmarkedなとき、外部輪郭上にいる
                                last_label_group += 1;

                                last_label = Label::Marked(last_label_group);
                                let argument = TraceArgument {
                                    trace_color: current_color,
                                    trace_label: last_label,
                                    footprint_label: Label::OutsideEdge,
                                };
                                label_image[current_position] = last_label;
                                tracer.trace(image, &mut label_image, argument);
                            }
                            _ => {
                                // last_labelがUnmarkedででないとき、内部輪郭線上にいる
                                let argument = TraceArgument {
                                    trace_color: current_color,
                                    trace_label: Label::InsideEdge,
                                    footprint_label: last_label,
                                };
                                tracer.trace(image, &mut label_image, argument);
                            }
                        };
                        last_color = current_color;
                    }
                    Label::OutsideEdge | Label::InsideEdge => {
                        // Magic) Unmarkedにする
                        last_label = Label::Unmarked;
                        last_color = current_color;
                    }
                    Label::Marked(v) => {
                        last_label = Label::Marked(v);
                        last_color = current_color;
                    }
                    Label::InnerMarked(v) => {
                        last_label = Label::InnerMarked(v);
                        last_color = current_color;
                    }
                }
            }
        }

        for (i, l) in label_image.iter().enumerate() {
            match l {
                Label::Marked(_v) | Label::InnerMarked(_v) => {
                    image.paint(i * 4, 0);
                }
                _ => {
                    image.paint(i * 4, 255);
                }
            }
        }

        label_image
    }
}

const SEARCH_DIRECTIONS: [[i8; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

#[derive(Debug)]
pub struct Tracer {
    position: Position,
    // next initial direction to search contour2 point
    direction: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TraceArgument {
    pub trace_color: Color,
    pub trace_label: Label,
    pub footprint_label: Label,
}

impl Tracer {
    pub fn new(start_position: Position) -> Tracer {
        Tracer {
            position: start_position,
            direction: 0,
        }
    }

    pub fn trace(
        &mut self,
        image: &mut Image,
        label_image: &mut Vec<Label>,
        argument: TraceArgument,
    ) {
        let start_position = self.position;

        self.trace_by_step(image, label_image, argument);

        loop {
            self.direction = (self.direction + 6) % 8;
            self.trace_by_step(image, label_image, argument);

            if start_position == self.position {
                break;
            }
        }
    }

    fn trace_by_step(
        &mut self,
        image: &mut Image,
        label_image: &mut Vec<Label>,
        argument: TraceArgument,
    ) {
        let width: u32 = image.width.into();
        for _ in 0..7 {
            let target_x = self.position.0 + SEARCH_DIRECTIONS[self.direction][0] as i64;
            let target_y = self.position.1 + SEARCH_DIRECTIONS[self.direction][1] as i64;
            let target_position = (target_y * (width as i64) + target_x) as usize;
            let target_color = Color::new(image.pixel(target_position));

            if target_color == argument.trace_color
                && (label_image[target_position] == Label::Unmarked
                    || label_image[target_position] == argument.trace_label)
            {
                label_image[target_position] = argument.trace_label;
                self.position = (target_x, target_y);

                return;
            }

            if label_image[target_position] == Label::Unmarked {
                label_image[target_position] = argument.footprint_label;
            }

            self.direction = (self.direction + 1) % 8;
        }
    }
}
