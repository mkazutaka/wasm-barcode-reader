use super::component::Component;
use crate::detector::cluster::Cluster;
use crate::image::Image;

use crate::processor::Processor;
use crate::reader::Label;

pub type Position = (i64, i64);

pub fn detect(image: &mut Image) -> Cluster {
    let label_image = Processor::draw_contour(image);

    let width = image.width();

    let mut components: Vec<Component> = Vec::with_capacity(400);

    for y in 0..image.height() {
        for x in 0..width {
            let position = ((y * width) + x) as usize;
            let position_label = label_image[position];

            match position_label {
                Label::Marked(v) => {
                    let position: Position = (x as i64, y as i64);

                    let marked_index = (v - 1) as usize;
                    if components.len() == marked_index {
                        components.push(Component::new())
                    }
                    &mut components[marked_index].update_by_contour(position);
                }
                Label::InnerMarked(v) => {
                    let position: Position = (x as i64, y as i64);
                    let index = (v - 1) as usize;
                    components[index].update(position);
                }
                _ => {}
            }
        }
    }

    // Cluster
    let threshold = 0.90;
    let mut clusters: Vec<Cluster> = Vec::new();
    for (_i, component) in components.iter_mut().enumerate() {
        if component.area.len() < 5 {
            continue;
        }
        component.moment();
        if component.ratio < 8.0 {
            continue;
        }

        let mut found = false;
        for cluster in clusters.iter_mut() {
            if cluster.check_fit(&component) {
                found = true;
                cluster.update(component.clone());
                break;
            }
        }

        if !found {
            let mut cluster = Cluster::new(threshold);
            cluster.update(component.clone());
            clusters.push(cluster);
        }
    }

    // Top Cluster
    let top_index = {
        let mut count = 0;
        let mut top_index = 0;
        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.components.len() > count {
                top_index = i;
                count = cluster.components.len();
            }
        }
        top_index
    };
    let top_cluster = clusters[top_index].clone();

    top_cluster
}
