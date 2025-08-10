use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenesisTriad {
    pub vertices: [Point; 3],
    pub area: f64,
    pub depth: u32,
    pub id: String,
}

impl GenesisTriad {
    pub fn new(center: Point, side_length: f64) -> Self {
        let height = (3.0_f64).sqrt() * side_length / 2.0;
        
        let vertices = [
            Point::new(center.x, center.y + (2.0 * height / 3.0)),
            Point::new(center.x - side_length / 2.0, center.y - height / 3.0),
            Point::new(center.x + side_length / 2.0, center.y - height / 3.0),
        ];
        
        let area = (3.0_f64).sqrt() * side_length * side_length / 4.0;
        
        GenesisTriad {
            vertices,
            area,
            depth: 0,
            id: Self::generate_id(&vertices),
        }
    }

    fn generate_id(vertices: &[Point; 3]) -> String {
        let coords: Vec<String> = vertices.iter()
            .map(|p| format!("{:.6},{:.6}", p.x, p.y))
            .collect();
        coords.join("|")
    }

    pub fn is_equilateral(&self) -> bool {
        let d1 = self.vertices[0].distance_to(&self.vertices[1]);
        let d2 = self.vertices[1].distance_to(&self.vertices[2]);
        let d3 = self.vertices[2].distance_to(&self.vertices[0]);
        
        let epsilon = 1e-10;
        (d1 - d2).abs() < epsilon && (d2 - d3).abs() < epsilon
    }

    pub fn centroid(&self) -> Point {
        let x = (self.vertices[0].x + self.vertices[1].x + self.vertices[2].x) / 3.0;
        let y = (self.vertices[0].y + self.vertices[1].y + self.vertices[2].y) / 3.0;
        Point::new(x, y)
    }
}

impl fmt::Display for GenesisTriad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GenesisTriad[id={}, depth={}, area={:.6}]", self.id, self.depth, self.area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_triad_creation() {
        let center = Point::new(0.0, 0.0);
        let triad = GenesisTriad::new(center, 2.0);
        
        assert!(triad.is_equilateral());
        assert_eq!(triad.depth, 0);
        assert!(triad.area > 0.0);
    }

    #[test]
    fn test_centroid_calculation() {
        let center = Point::new(0.0, 0.0);
        let triad = GenesisTriad::new(center, 2.0);
        let centroid = triad.centroid();
        
        assert!((centroid.x - 0.0).abs() < 1e-10);
        assert!((centroid.y - 0.0).abs() < 1e-10);
    }
}
