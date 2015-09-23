
// Third party crates
extern crate lodestone_bbox_polygon;
extern crate lodestone_extent;
extern crate lodestone_polygon;

use lodestone_bbox_polygon::bbox_polygon;
use lodestone_extent::Extent;
use lodestone_polygon::FeaturePolygon;

pub trait Envelope {
  fn envelope(&self) -> FeaturePolygon;
}

impl Envelope for FeaturePolygon {
  fn envelope(&self) -> FeaturePolygon {
    let bbox = self.extent();
    bbox_polygon(bbox)
  }
}

#[cfg(test)]
mod tests {
  use lodestone_polygon::FeaturePolygon;
  use super::Envelope;

  #[test]
  fn test_polygon() {
    let original_ring = vec![vec![0.0, 0.0], vec![1.0, 2.0], vec![2.0, 0.0], vec![0.0, 0.0]];
    let original_polygon = FeaturePolygon::new(vec![original_ring]);
    
    let extent_ring = vec![vec![0.0, 0.0], vec![2.0, 0.0], vec![2.0, 2.0], vec![0.0, 2.0], vec![0.0, 0.0]];
    let expected = FeaturePolygon::new(vec![extent_ring]);

    assert_eq!(expected, original_polygon.envelope());
  }
}
