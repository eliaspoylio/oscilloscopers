//#[path = "../vector.rs"] mod vector;
use crate::vector::{create_line_float, Point};

use std::collections::HashMap;

fn make_letter(
    font: HashMap<char, Vec<(Point, Point)>>,
    c: char,
    coord: (f32, f32),
    size: f32,
    spread: f32,
) -> Vec<Point> {
    let mut points = Vec::new();
    let lines = match font.get(&c) {
        Some(character) => character,
        None => panic!(),
    };
    for line in lines {
        let cline = create_line_float(
            Point {
                x: (coord.0 + line.0.x) * size,
                y: (coord.1 + line.0.y) * (size * spread),
            },
            Point {
                x: (coord.0 + line.1.x) * size,
                y: (coord.1 + line.1.y) * (size * spread),
            },
            1.,
        );
        for c in cline {
            points.push(c)
        }
    }
    points
}

pub fn letter(char: char, coord: (f32, f32), size: f32, spread: f32) -> Vec<Point> {
    let font = [
        (
            'c',
            vec![
                (Point::new(2., 0.), Point::new(7., 0.)),
                (Point::new(1., -1.), Point::new(3., -1.)),
                (Point::new(6., -1.), Point::new(8., -1.)),
                (Point::new(0., -2.), Point::new(2., -2.)),
                (Point::new(7., -2.), Point::new(9., -2.)),
                (Point::new(0., -3.), Point::new(2., -3.)),
                (Point::new(7., -3.), Point::new(9., -3.)),
                (Point::new(0., -4.), Point::new(2., -4.)),
                (Point::new(0., -5.), Point::new(2., -5.)),
                (Point::new(0., -6.), Point::new(2., -6.)),
                (Point::new(7., -6.), Point::new(9., -6.)),
                (Point::new(0., -7.), Point::new(2., -7.)),
                (Point::new(7., -7.), Point::new(9., -7.)),
                (Point::new(1., -8.), Point::new(3., -8.)),
                (Point::new(6., -8.), Point::new(8., -8.)),
                (Point::new(2., -9.), Point::new(7., -9.)),
            ],
        ),
        (
            'o',
            vec![
                (Point::new(2., 0.), Point::new(7., 0.)),
                (Point::new(1., -1.), Point::new(3., -1.)),
                (Point::new(6., -1.), Point::new(8., -1.)),
                (Point::new(0., -2.), Point::new(2., -2.)),
                (Point::new(7., -2.), Point::new(9., -2.)),
                (Point::new(0., -3.), Point::new(2., -3.)),
                (Point::new(7., -3.), Point::new(9., -3.)),
                (Point::new(0., -4.), Point::new(2., -4.)),
                (Point::new(7., -4.), Point::new(9., -4.)),
                (Point::new(0., -5.), Point::new(2., -5.)),
                (Point::new(7., -5.), Point::new(9., -5.)),
                (Point::new(0., -6.), Point::new(2., -6.)),
                (Point::new(7., -6.), Point::new(9., -6.)),
                (Point::new(0., -7.), Point::new(2., -7.)),
                (Point::new(7., -7.), Point::new(9., -7.)),
                (Point::new(1., -8.), Point::new(3., -8.)),
                (Point::new(6., -8.), Point::new(8., -8.)),
                (Point::new(2., -9.), Point::new(7., -9.)),
            ],
        ),
        (
            'd',
            vec![
                (Point::new(0., 0.), Point::new(7., 0.)),
                (Point::new(0., -1.), Point::new(3., -1.)),
                (Point::new(6., -1.), Point::new(8., -1.)),
                (Point::new(0., -2.), Point::new(2., -2.)),
                (Point::new(7., -2.), Point::new(9., -2.)),
                (Point::new(0., -3.), Point::new(2., -3.)),
                (Point::new(7., -3.), Point::new(9., -3.)),
                (Point::new(0., -4.), Point::new(2., -4.)),
                (Point::new(7., -4.), Point::new(9., -4.)),
                (Point::new(0., -5.), Point::new(2., -5.)),
                (Point::new(7., -5.), Point::new(9., -5.)),
                (Point::new(0., -6.), Point::new(2., -6.)),
                (Point::new(7., -6.), Point::new(9., -6.)),
                (Point::new(0., -7.), Point::new(2., -7.)),
                (Point::new(7., -7.), Point::new(9., -7.)),
                (Point::new(6., -8.), Point::new(8., -8.)),
                (Point::new(0., -8.), Point::new(3., -8.)),
                (Point::new(0., -9.), Point::new(7., -9.)),
            ],
        ),
        (
            'e',
            vec![
                (Point::new(0., 0.), Point::new(9., 0.)),
                (Point::new(0., -1.), Point::new(9., -1.)),
                (Point::new(0., -2.), Point::new(2., -2.)),
                (Point::new(0., -3.), Point::new(2., -3.)),
                (Point::new(0., -4.), Point::new(6., -4.)),
                (Point::new(0., -5.), Point::new(6., -5.)),
                (Point::new(0., -6.), Point::new(2., -6.)),
                (Point::new(0., -7.), Point::new(2., -7.)),
                (Point::new(0., -8.), Point::new(9., -8.)),
                (Point::new(0., -9.), Point::new(9., -9.)),
            ],
        ),
        (
            'm',
            vec![
                (Point::new(0., 0.), Point::new(1., 0.)),
                (Point::new(8., 0.), Point::new(9., 0.)),
                (Point::new(0., -1.), Point::new(2., -1.)),
                (Point::new(7., -1.), Point::new(9., -1.)),
                (Point::new(0., -1.), Point::new(3., -2.)),
                (Point::new(6., -1.), Point::new(9., -2.)),
                (Point::new(0., -3.), Point::new(9., -3.)),
                (Point::new(0., -4.), Point::new(9., -4.)),
                (Point::new(0., -5.), Point::new(2., -5.)),
                (Point::new(7., -5.), Point::new(9., -5.)),
                (Point::new(0., -6.), Point::new(2., -6.)),
                (Point::new(7., -6.), Point::new(9., -6.)),
                (Point::new(0., -7.), Point::new(2., -7.)),
                (Point::new(7., -7.), Point::new(9., -7.)),
                (Point::new(0., -8.), Point::new(2., -8.)),
                (Point::new(7., -8.), Point::new(9., -8.)),
                (Point::new(0., -9.), Point::new(2., -9.)),
                (Point::new(7., -9.), Point::new(9., -9.)),
            ],
        ),
    ];

    let font = HashMap::from(font);
    let letter: Vec<Point> = make_letter(font, char, coord, size, spread);

    letter
}
