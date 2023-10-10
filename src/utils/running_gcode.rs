#[derive(Debug, Clone, PartialEq)]
pub struct Point(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Line(pub Point, pub Point);

/**
 *
 * Minimize distance from point to line
 * Minimize x: y=mx+b : ((p.x - x)^2 + (p.y - y)^2)^0.5
 * Minimize x: ((p.x - x)^2 + (p.y - (mx+b))^2)^0.5
 * Minimize x: ((p.x - x)^2 + (p.y - mx-b)^2)^0.5
 * Minimize x: ((p.x - x)^2 + (p.y - mx-b)^2)^0.5
 *
 */

fn multiply_matrix(l11: f64, l12: f64, l21: f64, l22: f64, a: f64, b:f64) -> (f64, f64) {
    (
        l11 * a + l12 * b,
        l21 * a + l22 * b,
    )
}
fn multiply_matrix_m(m: (f64, f64, f64, f64), x: f64, y: f64) -> (f64, f64) {
    multiply_matrix(m.0, m.1, m.2, m.3, x, y)
}

impl Point {
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn from(x: f64, y: f64) -> Self {
        Self(x, y, 0.0)
    }

    pub fn distance_to(&self, point: &Point) -> f64 {
        let difx = self.0 - point.0;
        let dify = self.1 - point.1;
        return (difx * difx + dify * dify).sqrt();
    }
}

impl Line {
    pub fn distance_to(
        &self, point: Point
    ) -> f64 {
        return self.distance_to_point(&point);
        // ((self.1.0 - self.0.0) * (self.0.1 - point.1) -
        //     (self.0.0 - point.0) * (self.1.1 - self.0.1)).abs() /
        //     ((self.1.0 - self.0.0).powf(2.0) + (self.1.1 - self.0.1).powf(2.0)).sqrt()
    }
    pub fn distance_to_point(&self, point: &Point) -> f64 {
        // TODO: This function really needs to be speed up a bit.
        // Maybe look up the best way how instead of trying to figure
        //      out on your own.

        // theta = atan(
        //      [cos a2 (d(p2) / d(p1)) - cos a1 ] /
        //      [sin a2 (d(p2) / d(p1)) - sin a1 ]
        // )

        let degrees = if self.0 == Point::zero() && self.1 == Point::zero() {
            0.0
        } else if self.0 == Point::zero() {
            let a = (self.1.0 / self.1.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else if self.1 == Point::zero() {
            let a = (self.0.0 / self.0.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else {
            let a1 = (self.0.0 / self.0.distance_to(&Point::zero())).acos();
            let a2 = (self.1.0 / self.1.distance_to(&Point::zero())).acos();

            let p1d = self.0.distance_to(&Point::zero());
            let p2d = self.1.distance_to(&Point::zero());

            (
                (a2.cos() * (p2d / p1d) - a1.cos()) /
                (a2.sin() * (p2d / p1d) - a1.sin())
            ).atan()
        };


        let l11 = degrees.cos();
        let l12 = -degrees.sin();
        let l21 = degrees.sin();
        let l22 = degrees.cos();

        let m = (l11, l12, l21, l22);
        let p1 = multiply_matrix_m(m, self.0.0, self.0.1);
        let p2 = multiply_matrix_m(m, self.1.0, self.1.1);
        let c  = multiply_matrix_m(m, point.0, point.1);

        // println!("P1: ({}, {})", p1.0, p1.1);
        // println!("P2: ({}, {})", p2.0, p2.1);
        // println!("C : ({}, {})", c.0, c.1);
        if p1.1 > c.1 && p2.1 > c.1 {
            let c = Point::from(c.0, c.1);
            return if p1.1 > p2.1 {
                Point::from(p2.0, p2.1).distance_to(&c)
            } else {
                Point::from(p1.0, p1.1).distance_to(&c)
            };
        }

        if p1.1 < c.1 && p2.1 < c.1 {
            let c = Point::from(c.0, c.1);
            return if p1.1 < p2.1 {
                Point::from(p2.0, p2.1).distance_to(&c)
            } else {
                Point::from(p1.0, p1.1).distance_to(&c)
            };
        }

        (p1.0 - c.0).abs()
    }
}

use crate::utils::cncrouter;
use std::collections::{ HashMap };
use std::iter::{ Peekable };

pub fn to_f64<T>(s: &mut Peekable<T>) -> f64
where T : Iterator<Item=char>
{
    let mut r = 0.0;
    let mut seen_dot = false;
    let mut multiplier = 1.0;
    let mut is_negative = 1.0;

    loop {
        let Some(c) = s.peek() else {
            return r * is_negative;
        };

        if seen_dot {
            multiplier /= 10.0;
        }

        if *c == '.' {
            s.next();
            if seen_dot {
                return r * is_negative;
            }
            seen_dot = true;
        }
        else if *c == '-' {
            is_negative = -1.0;
            s.next();
        }
        else if let Some(d) = c.to_digit(10) {
            s.next();
            if seen_dot {
                r += multiplier * d as f64;
            } else {
                r = r * 10.0 + d as f64;
            }
        } else {
            return r * is_negative;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Warnings {
    OutOfBoundsLowXAxis,
    OutOfBoundsLowYAxis,
    OutOfBoundsLowZAxis,
    OutOfBoundsHighXAxis,
    OutOfBoundsHighYAxis,
    OutOfBoundsHighZAxis,
    CuttingNotInBounds,
}
impl std::fmt::Display for Warnings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Warnings::OutOfBoundsLowXAxis => write!(f, "X axis too low."),
            Warnings::OutOfBoundsLowYAxis => write!(f, "Y axis too low."),
            Warnings::OutOfBoundsLowZAxis => write!(f, "Z axis too low."),
            Warnings::OutOfBoundsHighXAxis => write!(f, "X axis too high."),
            Warnings::OutOfBoundsHighYAxis => write!(f, "Y axis too high."),
            Warnings::OutOfBoundsHighZAxis => write!(f, "Z axis too high."),
            Warnings::CuttingNotInBounds => write!(f, "You are cutting but not in bounds."),
        }
    }
}
impl Warnings {
    fn raw_value(&self) -> usize {
        match self {
            Warnings::OutOfBoundsLowXAxis => 0,
            Warnings::OutOfBoundsLowYAxis => 1,
            Warnings::OutOfBoundsLowZAxis => 2,
            Warnings::OutOfBoundsHighXAxis => 3,
            Warnings::OutOfBoundsHighYAxis => 4,
            Warnings::OutOfBoundsHighZAxis => 5,
            Warnings::CuttingNotInBounds => 6,
        }
    }
}

pub fn draw_path<T, F: FnMut(Point, Point, f64, f64)>(
    tools: Vec<cncrouter::Tool>,
    cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    non_cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    safe_point: (f64, f64, f64),
    s: &mut T,
    mut draw_line: F,
) -> (Vec<Warnings>, f64)
where T : Iterator<Item=char>
{
    let mut warnings = std::collections::HashSet::new();
    let mut time = 0.0;

    let mut cnc : cncrouter::CNCRouter = tools.into();
    let give_warnings = move |bounds: ((f64, f64, f64), (f64, f64, f64)), cnc : &cncrouter::CNCRouter| -> Vec<Warnings> {
        let mut warnings = Vec::new();

        if cnc.get_pos().x < bounds.0.0 {
            warnings.push(Warnings::OutOfBoundsLowXAxis);
        }
        if cnc.get_pos().y < bounds.0.1 {
            warnings.push(Warnings::OutOfBoundsLowYAxis);
        }
        if cnc.get_pos().z < bounds.0.2 {
            warnings.push(Warnings::OutOfBoundsLowZAxis);
        }

        if cnc.get_pos().x > bounds.1.0 {
            warnings.push(Warnings::OutOfBoundsHighXAxis);
        }
        if cnc.get_pos().y > bounds.1.1 {
            warnings.push(Warnings::OutOfBoundsHighYAxis);
        }
        if cnc.get_pos().z > bounds.1.2 {
            warnings.push(Warnings::OutOfBoundsHighZAxis);
        }

        return warnings;
    };

    let mut s = s.peekable();
    let mut variables_updates = Vec::new();
    let mut variables = HashMap::<char, f64>::new();
    variables.insert('X', cutting_box.0.0);
    variables.insert('Y', cutting_box.0.1);
    variables.insert('Z', cutting_box.1.2);
    variables.insert('T', 1.0);
    variables.insert('F', 120.0);
    variables.insert('G', 0.0);
    variables.insert('M', 0.0);

    let mut spindle_on = false;
    let mut is_fast_route = true;

    while let Some(c) = s.next() {
        if c == '\n' {
            let mut changed_pos = false;
            let mut changed_m = false;
            // let mut changed_g = false;

            for v in &variables_updates {
                // changed_pos |= *v == 'X' || *v == 'Y' || *v == 'Z';
                changed_pos |= *v == 'X' || *v == 'Y';
                changed_m |= *v == 'M';
                // changed_g |= *v == 'G';
            }

            if changed_m {
                if variables[&'M'] == 6.0 {
                    cnc.set_tool((&variables[&'T']).round() as usize - 1);
                    spindle_on = false;
                    time += 0.1;
                }
            }
            else if changed_pos && cnc.get_pos().z <= 0.0 &&
                cnc.get_pos().z == variables[&'Z'] {
                draw_line(
                    Point(cnc.get_pos().x, cnc.get_pos().y, cnc.get_pos().z),
                    Point(variables[&'X'], variables[&'Y'], variables[&'Z']),
                    cnc.get_tool().length,
                    cnc.get_tool().radius,
                );
            }

            let distance = cnc.get_pos().distance_to(&cncrouter::Coordinate {
                x: variables[&'X'],
                y: variables[&'Y'],
                z: variables[&'Z'],
            });

            time += distance * if is_fast_route {
                1.0 / 2_000.0
            } else {
                1.0 / variables[&'F']
            };

            if distance > 0.0001 {
                time += 1. / 9_000.;
            }

            cnc.set_pos(
                &cncrouter::OptionCoordinate {
                    x: Some(variables[&'X']),
                    y: Some(variables[&'Y']),
                    z: Some(variables[&'Z']),
                }
            );

            if !(
                cnc.get_pos().x == safe_point.0 &&
                cnc.get_pos().y == safe_point.1 &&
                cnc.get_pos().z == safe_point.2
            ) {
                for warning in give_warnings(
                    if spindle_on {
                        cutting_box
                    } else {
                        non_cutting_box
                    },
                    &cnc,
                )
                {
                    warnings.insert(warning);
                }
            }

            variables_updates.clear();
        } else if c.is_ascii_uppercase() {
            let value = to_f64(&mut s);
            variables.insert(c, value);
            if c == 'M' && value == 5. {
                spindle_on = false;
            }
            else if c == 'M' && value == 3. {
                spindle_on = true;
            }
            else if c == 'M' && value == 4. {
                spindle_on = true;
            }
            else if c == 'G' && value == 0. {
                is_fast_route = true;
            }
            else if c == 'G' && value == 1. {
                is_fast_route = false;
            }
            variables_updates.push(c);
        } else if c == '(' {
            while let Some(n) = s.next() {
                if n == ')' {
                    break;
                }
            }
        } else if c == ' ' {

        } else {
            // eprintln!("CANT RECOGNIZE: {}", c);
        }
    }

    return (warnings.into_iter().collect(), time);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_distance_to_line() {
        assert!((
            Line(
                Point(0.0, 0.0, 0.0),
                Point(1.0, 1.0, 0.0)
            ).distance_to(
                Point(0.5, 0.5, 0.0)
            ) - 0.0).abs() < 0.1
        );

        let d = Line(
            Point(0.0, 0.0, 0.0),
            Point(1.0, 1.0, 0.0)
        ).distance_to(
            Point(100.0, 100.0, 0.0)
        );
        println!("{} != 141.4213562373095", d);
        assert!((d - 141.4213562373095).abs() < 3.0);
    }
}
