#![feature(int_roundings)]
/// Fast Concave Hull Implementation
///
/// This module provides an efficient implementation of the concave hull algorithm,
/// using a k-nearest neighbour approach.
use kiddo::{KdTree, SquaredEuclidean};

use std::collections::HashMap;

pub mod binding;
mod intersect;
pub mod point;
use intersect::intersects;
use point::{normalise_angle, Point, PointValue};

/// Computes the concave hull of a given set of points.
///
/// This function iteratively calls k-nearest neighbors based concave hull algorithm.
///
/// # Arguments
///
/// * `dataset` -  2D point cloud.
/// * `k` - number of nearest neighbors.
/// * `iterate` - A boolean flag that, when set to `false`, stops the iteration
///               as soon as the algorithm succeeds.
///
/// # Returns
///
/// Returns a vector of `Point` structures representing the concave hull of the provided dataset.
/// If the hull cannot be computed, it returns an empty vector.
///
/// # Examples
///
/// ```
/// let mut dataset = vec![Point { x: 1.0, y: 1.0, id: 1 }, Point { x: 2.0, y: 2.0, id: 2 }];
/// let k = 3;
/// let hull = concave_hull(&mut dataset, k, true);
/// ```
pub fn concave_hull(dataset: &mut Vec<Point>, mut k: usize, iterate: bool) -> Vec<Point> {
    while k < dataset.len() {
        let mut hull = Vec::<Point>::new();
        if concave_hull_inner(dataset, k, &mut hull) || !iterate {
            return hull;
        }
        k += 1;
    }

    Vec::<Point>::new()
}

fn concave_hull_inner(point_list: &mut Vec<Point>, k: usize, hull: &mut Vec<Point>) -> bool {
    hull.clear();

    if let 0..=3 = point_list.len() {
        hull.extend(point_list.iter().cloned());
        return true;
    }

    // build a kd tree so we can do the spatial queries
    let mut tree: KdTree<_, 2> = KdTree::new(); //(&entries).into();
    for point in point_list.iter() {
        tree.add(&[point.x, point.y], point.id)
    }

    // map id to points for lookup
    let mut point_map: HashMap<u64, Point> = point_list
        .iter()
        .map(|point| (point.id, point.clone()))
        .collect();

    // Initialize hull with the min-y point
    let mut first_point = find_min_y_point(point_list);
    hull.push(first_point.clone());

    // Until the hull is of size > 3 we want to ignore the first point from nearest neighbour searches
    let mut current_point = first_point.clone();
    let cp = current_point.clone();
    tree.remove(&[cp.x, cp.y], cp.id);

    let mut prev_angle = 0.0f64;
    let mut step = 1usize;

    // Iterate until we reach the start, or until there's no points left to process
    while (!(current_point == first_point) || step == 1) && hull.len() != point_list.len() {
        if step == 4 {
            first_point.id = point_list.len() as u64;
            let p = first_point.clone();
            tree.add(&[p.x, p.y], p.id);
            point_map.insert(first_point.id, first_point.clone());
        }

        let cp = current_point.clone();
        let knn = tree.nearest_n::<SquaredEuclidean>(&[cp.x, cp.y], k);
        let mut nearest: Vec<PointValue> = knn
            .iter()
            .map(|p| {
                let point = point_map.get(&p.item).unwrap();
                PointValue {
                    point: point.clone(),
                    distance: p.distance,
                    angle: current_point.angle(point),
                }
            })
            .collect();
        let c_points = sort_by_angle(&mut nearest, &current_point, prev_angle);

        let mut its = true;
        let mut i = 0usize;

        while its && i < c_points.len() {
            let mut last_point = 0;
            if *c_points.get(i).unwrap() == first_point {
                last_point = 1;
            }

            let mut j = 2;
            its = false;

            while !its && j < hull.len() - last_point {
                let line1 = (hull.get(step - 1).unwrap(), c_points.get(i).unwrap());
                let line2 = (hull.get(step - j - 1).unwrap(), hull.get(step - j).unwrap());
                its = intersects(line1, line2);
                j += 1;
            }

            if its {
                i += 1;
            }
        }

        if its {
            return false;
        }

        current_point = c_points[i].clone();

        hull.push(current_point.clone());

        prev_angle = hull[step].angle(&hull[step - 1]);

        let cp = current_point.clone();
        tree.remove(&[cp.x, cp.y], cp.id);

        step += 1;
    }

    let new_end = remove_hull(point_list, hull);

    multiple_point_in_polygon(&new_end, hull)
}

fn find_min_y_point(points: &[Point]) -> Point {
    assert!(!points.is_empty());

    points
        .iter()
        .min_by(|a, b| {
            if a.y == b.y {
                greater_than(a.x, b.x)
            } else {
                less_than(a.y, b.y)
            }
        })
        .expect("No minimum element found")
        .clone()
}

fn greater_than(a: f64, b: f64) -> std::cmp::Ordering {
    if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

fn less_than(a: f64, b: f64) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn sort_by_angle(values: &mut [PointValue], from: &Point, prev_angle: f64) -> Vec<Point> {
    // Calculate angles
    for to in values.iter_mut() {
        to.angle = normalise_angle(from.angle(&to.point) - prev_angle);
    }

    // Sort in descending order of angle
    values.sort_by(|a, b| greater_than(a.angle, b.angle));

    // Extract points from PointValue and collect into a vector
    values.iter().map(|pv| pv.point.clone()).collect()
}

fn remove_hull(points: &mut Vec<Point>, hull: &[Point]) -> Vec<Point> {
    let ids: Vec<u64> = hull.iter().map(|p| p.id).collect();

    points.retain(|p| ids.binary_search(&p.id).is_err());

    points.to_vec()
}

fn multiple_point_in_polygon(points: &[Point], hull: &[Point]) -> bool {
    points.iter().all(|p| point_in_polygon(p, hull))
}

fn point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    if polygon.len() <= 2 {
        return false;
    }

    let x = point.x;
    let y = point.y;

    let mut inout = 0;
    let mut v0 = &polygon[0];

    for v1 in polygon.iter() {
        if (((v0.y <= y) && (y < v1.y)) || ((v1.y <= y) && (y < v0.y)))
            && ((v1.y - v0.y).abs() >= 1E-10)
        {
            let tdbl1 = (y - v0.y) / (v1.y - v0.y);
            let tdbl2 = v1.x - v0.x;

            if x < v0.x + (tdbl2 * tdbl1) {
                inout += 1;
            }
        }

        v0 = v1;
    }

    inout % 2 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concave_hull() {
        let mut point_list = vec![
            Point {
                x: 0.0,
                y: 1.0,
                id: 0,
            },
            Point {
                x: -1.0,
                y: 0.0,
                id: 1,
            },
            Point {
                x: 1.0,
                y: 0.0,
                id: 2,
            },
        ];
        let hull = concave_hull(&mut point_list, 1, true);
        assert!(hull.len() == 3);

        let mut point_list = vec![
            Point {
                x: 1.0 / 2.0,
                y: 1.0 / 2.0,
                id: 0,
            },
            Point {
                x: -1.0 / 2.0,
                y: 1.0 / 2.0,
                id: 1,
            },
            Point {
                x: -1.0 / 2.0,
                y: -1.0 / 2.0,
                id: 2,
            },
            Point {
                x: 1.0 / 2.0,
                y: -1.0 / 2.0,
                id: 3,
            },
            Point {
                x: 0.0,
                y: 0.0,
                id: 4,
            },
        ];
        let _hull = concave_hull(&mut point_list, 1, true);
    }
}
