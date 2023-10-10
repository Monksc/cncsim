use crate::utils::cncrouter;
use crate::utils::running_gcode;

use algorithms;
use std::fs;


pub fn to_png<T, W : std::io::Write>(
    image_size: (u32, u32),
    frame: (f64, f64, f64, f64),
    z_axis_of_cut: f64,
    tools: Vec<cncrouter::Tool>,
    cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    non_cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    safe_point: (f64, f64, f64),
    s: &mut T,
    writer: &mut W
) -> std::io::Result<(Vec<running_gcode::Warnings>, f64)>
where T : Iterator<Item=char>
{
    let mut data: Vec<u8> = Vec::new();
    for _ in 0..(image_size.0 * image_size.1) {
        data.push(0);
        data.push(0);
        data.push(255);
    }

    let (warnings, time) = running_gcode::draw_path(
        tools,
        cutting_box,
        non_cutting_box,
        safe_point,
        s,
        |p1, p2, length, radius| {
            if p1.2 > 0.0 && p2.0 > 0.0 { return }

            let convert = |min: f64, max: f64, value: f64, new_size: u32| {
                (new_size as f64 * (((value - min) / (max - min)))) as u32
            };
            let reverse_convert = |min: f64, max: f64, value: u32, new_size: u32| {
                min + (value as f64 / new_size as f64 ) * (max - min)
            };
            let x = convert(frame.0, frame.2, p1.0, image_size.0);
            let y = convert(frame.1, frame.3, p1.1, image_size.1);

            let line = running_gcode::Line(p1, p2);

            algorithms::bfs(
                (x, y),
                |(x, y)| {
                    let mut r : Vec<(u32, u32)> = Vec::new();

                    for (dx, dy) in vec![(1,0), (0,1), (-1,0), (0,-1)] {
                        let x = x as i64 + dx;
                        let y = y as i64 + dy;

                        if x <= 0 || y <= 0 {
                            continue;
                        }

                        let xf = reverse_convert(frame.0, frame.2, x as u32, image_size.0);
                        let yf = reverse_convert(frame.1, frame.3, y as u32, image_size.1);

                        let distance = line.distance_to(running_gcode::Point(xf, yf, 0.0));

                        if xf < frame.2 && xf > frame.0 &&
                            yf < frame.3 && xf > frame.1 &&
                                 distance <= radius {
                            r.push((x as u32, y as u32));
                            let position = (image_size.1 - y as u32) * image_size.0 + x as u32;
                            let new_value = (255.0 * (1.0-(distance / radius).powf(2.0))) as u8;
                            if new_value > data[position as usize * 3] {
                                data[position as usize * 3+0] = new_value;
                                data[position as usize * 3+1] = new_value;
                                data[position as usize * 3+2] = new_value;
                            }
                        }
                    }

                    return r;
                },
                |(x, y)| false,
            );
        }
    );

    let mut img = image::codecs::png::PngEncoder::new(writer);

    let r = img.encode(
        &data,
        image_size.0,
        image_size.1,
        image::ColorType::Rgb8,
    );

    Ok((warnings, time))
}


fn float_loop(start: f64, threshold: f64, step_size: f64) -> impl Iterator<Item = f64> {
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        if prev >= threshold {
            None
        } else if next < threshold {
            Some(next)
        }
        else {
            Some(threshold)
        }
    })
}

