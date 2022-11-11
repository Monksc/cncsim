use crate::utils::cncrouter;

use std::collections::{ HashMap };
use std::iter::{ Peekable };
use std::fs;

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

const TEMPLATE_FILE_PATH: &str = "./openscad_files/file.scad";

pub fn to_scad<T, W : std::io::Write>(
    fnvalue: u64,
    block_width: f64,
    block_height: f64,
    tools: Vec<cncrouter::Tool>,
    s: &mut T,
    writer: &mut W
) -> std::io::Result<()>
where T : Iterator<Item=char>
{

    let mut cnc : cncrouter::CNCRouter = tools.into();
    let contents = fs::read_to_string(TEMPLATE_FILE_PATH)
        .expect("Cant find the template file anywhere.");
    writeln!(
        writer,
        "$fn = {};\n{}\n    cube([{},{},0.5]);",
        fnvalue, &contents,
        block_width, block_height,
    )?;
    

    let mut s = s.peekable();
    let mut variables_updates = Vec::new();
    let mut variables = HashMap::<char, f64>::new();
    variables.insert('X', 0.0);
    variables.insert('Y', 0.0);
    variables.insert('Z', 0.0);
    variables.insert('T', 1.0);
    while let Some(c) = s.next() {
        if c == '\n' {
            let mut changed_pos = false;
            let mut changed_m = false;
            let mut changed_g = false;

            for v in &variables_updates {
                // changed_pos |= *v == 'X' || *v == 'Y' || *v == 'Z';
                changed_pos |= *v == 'X' || *v == 'Y';
                changed_m |= *v == 'M';
                changed_g |= *v == 'G';
            }

            if changed_m {
                if variables[&'M'] == 6.0 {
                    cnc.set_tool((&variables[&'T']).round() as usize - 1);
                }
            }
            else if changed_pos && cnc.get_pos().z <= 0.0 && cnc.get_pos().z == variables[&'Z'] {
                writeln!(
                    writer, "    bitpath({}, {}, {}, {}, {}, {}, {}, {});",
                    cnc.get_pos().x, cnc.get_pos().y, cnc.get_pos().z,
                    variables[&'X'], variables[&'Y'], variables[&'Z'],
                    cnc.get_tool().length,
                    cnc.get_tool().radius,
                )?;
            }

            cnc.set_pos(
                &cncrouter::OptionCoordinate {
                    x: Some(variables[&'X']),
                    y: Some(variables[&'Y']),
                    z: Some(variables[&'Z']),
                }
            );

            variables_updates.clear();
        } else if c.is_ascii_uppercase() {
            let value = to_f64(&mut s);
            variables.insert(c, value);
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
    writeln!(writer, "{}", "}")?;

    Ok(())
}
