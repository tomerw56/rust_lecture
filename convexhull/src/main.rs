    use geo::{polygon};
    use geo::geometry::{Polygon,LineString};
    use geo::ConvexHull;
    use rand::distributions::{Distribution, Standard};
    use rand::Rng;
    use std::time::{Duration, Instant};
    use std::vec;

fn generate_polygon(number_of_points:u32)->Polygon{
        let mut exterior: Vec<(f64,f64)>=Vec::new();
        let mut rnd = rand::thread_rng();
        for x in 0..number_of_points {
            let coordinate = rnd.gen::<(f64, f64)>();
            exterior.insert(0, coordinate);
        }
        let line_string: LineString<f64> = exterior.into();
        return  Polygon::new(line_string,vec![]);
}
fn main() {
    
    for x in 0..100000 {
        println!("Iteration {}", x); // x: i32
       
        // an L shape
        let poly = generate_polygon(100);
        let start = Instant::now();
        let res = poly.convex_hull();
        let duration = start.elapsed();
        println!("Time elapsed in convex_Hull() is: {:?}", duration);

        }
    
}
