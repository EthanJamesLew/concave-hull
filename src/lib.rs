#![feature(int_roundings)]

use kiddo::{KdTree, SquaredEuclidean};

use std::f64::consts::PI;
use std::collections::HashMap;

pub mod point;
pub mod binding;
use point::{Point, PointValue};

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
    let mut point_map: HashMap<u64, Point> = point_list.iter().map(|point| (point.id, point.clone())).collect();
    
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
                    PointValue{
                        point: point.clone(),
                        distance: p.distance,
                        angle: angle(&current_point, point)

                    }
                })
                .collect();
        let c_points = sort_by_angle(
                &mut nearest,
                &current_point, 
                prev_angle
        );

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
                let line1 = (hull.get(step-1).unwrap(), c_points.get(i).unwrap());
                let line2 = (hull.get(step-j-1).unwrap(), hull.get(step-j).unwrap());
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

        prev_angle = angle(&hull[step], &hull[step - 1]);

        let cp = current_point.clone();
        tree.remove(&[cp.x, cp.y], cp.id);

        step += 1;

    }

    let new_end = remove_hull(point_list, hull);
    

    multiple_point_in_polygon(&new_end, hull)
}

fn find_min_y_point(points: &[Point]) -> Point {
    assert!(!points.is_empty());

    points.iter()
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
    if a > b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
}

fn less_than(a: f64, b: f64) -> std::cmp::Ordering {
    if a < b { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
}

fn sort_by_angle(values: &mut [PointValue], from: &Point, prev_angle: f64) -> Vec<Point> {
    // Calculate angles
    for to in values.iter_mut() {
        to.angle = normalise_angle(angle(from, &to.point) - prev_angle);
    }

    // Sort in descending order of angle
    values.sort_by(|a, b| greater_than(a.angle, b.angle));

    // Extract points from PointValue and collect into a vector
    values.iter().map(|pv| pv.point.clone()).collect()
}

fn angle(a: &Point, b: &Point) -> f64 {
    let angle = -((b.y - a.y).atan2(b.x - a.x));
    normalise_angle(angle)
}

fn normalise_angle(radians: f64) -> f64 {
    if radians < 0.0 {
        radians + PI + PI
    } else {
        radians
    }
}

fn intersects(a: (&Point, &Point), b: (&Point, &Point)) -> bool {
    let ax1 = a.0.x;
    let ay1 = a.0.y;
    let ax2 = a.1.x;
    let ay2 = a.1.y;
    let bx1 = b.0.x;
    let by1 = b.0.y;
    let bx2 = b.1.x;
    let by2 = b.1.y;

    let a1 = ay2 - ay1;
    let b1 = ax1 - ax2;
    let c1 = a1 * ax1 + b1 * ay1;
    let a2 = by2 - by1;
    let b2 = bx1 - bx2;
    let c2 = a2 * bx1 + b2 * by1;
    let det = a1 * b2 - a2 * b1;

    if det.abs() < 1E-10 {
        false
    } else {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;

        
        ax1.min(ax2) <= x
            && (x <= ax1.max(ax2))
            && (ay1.min(ay2) <= y)
            && (y <= ay1.max(ay2))
            && (bx1.min(bx2) <= x)
            && (x <= bx1.max(bx2))
            && (by1.min(by2) <= y)
            && (y <= by1.max(by2))
    }
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
        if (((v0.y <= y) && (y < v1.y)) || ((v1.y <= y) && (y < v0.y))) && ((v1.y - v0.y).abs() >= 1E-10) {
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
    use std::collections::HashMap;

    #[test]
    fn test_concave_hull() {
        let mut point_list = vec![
            Point {x: 0.0, y: 1.0, id: 0},
            Point {x: -1.0, y: 0.0, id: 1},
            Point {x: 1.0, y: 0.0, id: 2},
        ];
        let hull = concave_hull(&mut point_list, 1, true);
        assert!(hull.len()  == 3);
        
        let mut point_list = vec![
            Point {x: 1.0 / 2.0, y: 1.0 / 2.0, id: 0},
            Point {x: -1.0 / 2.0, y: 1.0 / 2.0, id: 1},
            Point {x: -1.0 / 2.0, y: -1.0 / 2.0, id: 2},
            Point {x: 1.0 / 2.0, y: -1.0 / 2.0, id: 3},
            Point {x: 0.0, y: 0.0, id: 4}
        ];
        let _hull = concave_hull(&mut point_list, 1, true);
    }

    fn test_intersects() {
        let mut values = HashMap::new();
        values.insert('A', Point { x:  0.0, y:  0.0, id: 0 });
        values.insert('B', Point { x: -1.5, y:  3.0, id: 0 });
        values.insert('C', Point { x:  2.0, y:  2.0, id: 0 });
        values.insert('D', Point { x: -2.0, y:  1.0, id: 0 });
        values.insert('E', Point { x: -2.5, y:  5.0, id: 0 });
        values.insert('F', Point { x: -1.5, y:  7.0, id: 0 });
        values.insert('G', Point { x:  1.0, y:  9.0, id: 0 });
        values.insert('H', Point { x: -4.0, y:  7.0, id: 0 });
        values.insert('I', Point { x:  3.0, y: 10.0, id: 0 });
        values.insert('J', Point { x:  2.0, y: 11.0, id: 0 });
        values.insert('K', Point { x: -1.0, y: 11.0, id: 0 });
        values.insert('L', Point { x: -3.0, y: 11.0, id: 0 });
        values.insert('M', Point { x: -5.0, y:  9.5, id: 0 });
        values.insert('N', Point { x: -6.0, y:  7.5, id: 0 });
        values.insert('O', Point { x: -6.0, y:  4.0, id: 0 });
        values.insert('P', Point { x: -5.0, y:  2.0, id: 0 });

        let test = |a1: char, a2: char, b1: char, b2: char, expected: bool| {
            let line1 = (&values[&a1], &values[&a2] );
            let line2 = (&values[&b1], &values[&b2] );
            assert!(intersects(line1, line2) == expected);
        };

        test('B', 'D', 'A', 'C', false);
        test('A', 'B', 'C', 'D', true);
        test('L', 'K', 'H', 'F', false);
        test('E', 'C', 'F', 'B', true);
        test('P', 'C', 'E', 'B', false);
        test('P', 'C', 'A', 'B', true);
        test('O', 'E', 'C', 'F', false);
        test('L', 'C', 'M', 'N', false);
        test('L', 'C', 'N', 'B', false);
        test('L', 'C', 'M', 'K', true);
        test('L', 'C', 'G', 'I', false);
        test('L', 'C', 'I', 'E', true);
        test('M', 'O', 'N', 'F', true);
    }

    #[test]
    fn test_intersects_function() {
        test_intersects();
    }

    fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    fn test_angle() {
        let test = |p: Point, expected: f64| {
            let actual = to_degrees(angle(&Point { x: 0.0, y: 0.0, id: 0 }, &p));
            assert!((actual == expected), "Test failed for point: ({}, {})", p.x, p.y);
        };

        let value = to_degrees((3.0f64 / 4.0).atan());

        test(Point { x:  5.0, y:  0.0, id: 0 }, 0.0);
        test(Point { x:  4.0, y:  3.0, id: 0 }, 360.0 - value);
        test(Point { x:  3.0, y:  4.0, id: 0 }, 270.0 + value);
        test(Point { x:  0.0, y:  5.0, id: 0 }, 270.0);
        test(Point { x: -3.0, y:  4.0, id: 0 }, 270.0 - value);
        test(Point { x: -4.0, y:  3.0, id: 0 }, 180.0 + value);
        test(Point { x: -5.0, y:  0.0, id: 0 }, 180.0);
        test(Point { x: -4.0, y: -3.0, id: 0 }, 180.0 - value);
        test(Point { x: -3.0, y: -4.0, id: 0 }, 90.0 + value);
        test(Point { x:  0.0, y: -5.0, id: 0 }, 90.0);
        test(Point { x:  3.0, y: -4.0, id: 0 }, 90.0 - value);
        test(Point { x:  4.0, y: -3.0, id: 0 }, value);
    }

    #[test]
    fn test_angle_function() {
        test_angle();
    }
}
