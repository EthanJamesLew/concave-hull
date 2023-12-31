/// Geometric Intersection Methods
use crate::point::Point;

/// Determines if two line segments intersect.
///
/// This function checks whether the line segment formed by points `a.0` and `a.1`
/// intersects with the line segment formed by points `b.0` and `b.1`. It uses a
/// geometric approach to calculate the intersection point and checks if this point
/// lies within the bounds of both line segments.
///
/// # Parameters
///
/// * `a`: (&Point, &Point) - A tuple containing two references to `Point` objects,
///         representing the first line segment.
/// * `b`: (&Point, &Point) - A tuple containing two references to `Point` objects,
///         representing the second line segment.
///
/// # Returns
///
/// * `bool` - `true` if the line segments intersect, `false` otherwise.
///
/// # Note
///
/// The function uses a threshold (1E-10) to handle floating-point arithmetic precision issues.
/// This means very close lines that don't technically intersect might be considered as intersecting.
pub fn intersects(a: (&Point, &Point), b: (&Point, &Point)) -> bool {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn test_intersects() {
        let mut values = HashMap::new();
        values.insert(
            'A',
            Point {
                x: 0.0,
                y: 0.0,
                id: 0,
            },
        );
        values.insert(
            'B',
            Point {
                x: -1.5,
                y: 3.0,
                id: 0,
            },
        );
        values.insert(
            'C',
            Point {
                x: 2.0,
                y: 2.0,
                id: 0,
            },
        );
        values.insert(
            'D',
            Point {
                x: -2.0,
                y: 1.0,
                id: 0,
            },
        );
        values.insert(
            'E',
            Point {
                x: -2.5,
                y: 5.0,
                id: 0,
            },
        );
        values.insert(
            'F',
            Point {
                x: -1.5,
                y: 7.0,
                id: 0,
            },
        );
        values.insert(
            'G',
            Point {
                x: 1.0,
                y: 9.0,
                id: 0,
            },
        );
        values.insert(
            'H',
            Point {
                x: -4.0,
                y: 7.0,
                id: 0,
            },
        );
        values.insert(
            'I',
            Point {
                x: 3.0,
                y: 10.0,
                id: 0,
            },
        );
        values.insert(
            'J',
            Point {
                x: 2.0,
                y: 11.0,
                id: 0,
            },
        );
        values.insert(
            'K',
            Point {
                x: -1.0,
                y: 11.0,
                id: 0,
            },
        );
        values.insert(
            'L',
            Point {
                x: -3.0,
                y: 11.0,
                id: 0,
            },
        );
        values.insert(
            'M',
            Point {
                x: -5.0,
                y: 9.5,
                id: 0,
            },
        );
        values.insert(
            'N',
            Point {
                x: -6.0,
                y: 7.5,
                id: 0,
            },
        );
        values.insert(
            'O',
            Point {
                x: -6.0,
                y: 4.0,
                id: 0,
            },
        );
        values.insert(
            'P',
            Point {
                x: -5.0,
                y: 2.0,
                id: 0,
            },
        );

        let test = |a1: char, a2: char, b1: char, b2: char, expected: bool| {
            let line1 = (&values[&a1], &values[&a2]);
            let line2 = (&values[&b1], &values[&b2]);
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
}
