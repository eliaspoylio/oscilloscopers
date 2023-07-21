//#[path = "../vector.rs"] mod vector;
use crate::vector::{create_line_float, Point};

use std::collections::HashMap;

fn make_letter(
    font: HashMap<char, Vec<(Point, Point)>>,
    c: char,
    coord: (f32, f32),
    size: f32
) -> Vec<Point> {
    let mut points = Vec::new();
    let lines = match font.get(&c) {
        Some(review) => review,
        None => panic!(),
    };
    for line in lines {
        let x = line.0.x + coord.0;
        let cline = create_line_float(
            Point {
                x: coord.0 + line.0.x * size,
                y: coord.1 + line.0.y * (size/5.),
            },
            Point {
                x: coord.0 + line.1.x * size,
                y: coord.1 + line.1.y * (size/5.),
            },
            0.1,
        );
        for c in cline {
            points.push(c)
        }
    }
    points
}

pub fn letter(coord: (f32, f32), size: f32) -> Vec<Point> {
    let font: [(char, Vec<(Point, Point)>); 2] = [
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
        ('b', vec![(Point::new(1., 1.), Point::new(1., 1.))]),
    ];

    let font = HashMap::from(font);
    let letter: Vec<Point> = make_letter(font, 'c', coord, size);

    letter
}
