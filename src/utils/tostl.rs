use crate::utils::cncrouter;
use crate::utils::running_gcode;

use std::fs;

const TEMPLATE_FILE_PATH: &str = "./openscad_files/file.scad";

pub fn to_scad<T, W: std::io::Write>(
    fnvalue: u64,
    block_width: f64,
    block_height: f64,
    tools: Vec<cncrouter::Tool>,
    cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    non_cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    safe_point: (f64, f64, f64),
    s: &mut T,
    writer: &mut W,
) -> std::io::Result<(Vec<running_gcode::Warnings>, f64)>
where
    T: Iterator<Item = char>,
{
    let contents =
        fs::read_to_string(TEMPLATE_FILE_PATH).expect("Cant find the template file anywhere.");

    writeln!(
        writer,
        "$fn = {};\n{}\n    cube([{},{},0.5]);",
        fnvalue, &contents, block_width, block_height,
    )?;

    let (warnings, time) = running_gcode::draw_path(
        tools,
        cutting_box,
        non_cutting_box,
        safe_point,
        s,
        |p1, p2, length, radius, _| {
            writeln!(
                writer,
                "    bitpath({}, {}, {}, {}, {}, {}, {}, {});",
                p1.0, p1.1, p1.2, p2.0, p2.1, p2.2, length, radius,
            );
        },
    );

    writeln!(writer, "{}", "}")?;

    Ok((warnings, time))
}
