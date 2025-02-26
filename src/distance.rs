use crate::types::GeoLocation;

pub fn calculate_distance(point_a: GeoLocation, point_b: GeoLocation) -> f32 {
    let x = (point_a.0 - point_b.0) * 111.32; // lat to km
    let y = ((point_a.1 - point_b.1) * 111.319444444).cos(); // long to km

    ((x * x) + (y * y)).sqrt()
}
