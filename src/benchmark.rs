use std::collections::HashMap;
use std::io::{stdout, Write};
use std::ops::Div;
use std::time::Duration;
use itertools::Itertools;
use svg::Document;
use svg::node::element::{Group, Rectangle, Text, LinearGradient, Stop, Line, Script};
use crate::{solve_day, Year};


/// The result of performance benchmarking, indexed on the day numbers.
pub type BenchmarkResult = HashMap<u32, anyhow::Result<Duration>>;

/// Raw result of performance benchmarking, as execution times indexed on the day numbers.
type BenchmarkRawResult = HashMap<u32, anyhow::Result<Vec<Duration>>>;

/// Do a benchmark of the provided `year`. The function execute each daily puzzle in turn,
/// and repeat the operation multiple times according to the parameter `num_repetitions`.
pub fn benchmark_year<Y> (year: &Y, num_repetitions: usize) -> BenchmarkResult
where Y : Year {

    println!("Benchmark year {:?}: ", year.get_year());

    let mut raw_durations = BenchmarkRawResult::new();

    for idx in 0..num_repetitions {

        if idx% 10 == 0 { print!("#"); }
        else { print!("."); }
        stdout().flush().expect("TODO: panic message");

        for day in 1..=25 {

            // Get the function related to the current day, or skip the test
            let Some(fn_solve) = year.get_day_fn(day) else { continue };

            // Also skip if failed in previous iteration
            let day_entry = raw_durations.entry(day).or_insert_with(|| Ok(vec![]));
            let Ok(day_duration) = day_entry else { continue };

            // Solve and collect the solving time, or the error
            match solve_day(year.get_year(), day, fn_solve) {
                Ok((_a, _b, duration)) => { day_duration.push(duration); }
                Err(err) => { *day_entry = Err(err) }
            };
        }
    }

    // Take each vector of measurement and compute a trimmed mean
    raw_durations.into_iter().map(
        |(day, duration_or_err)| (
            day,
            duration_or_err.map(|d| trimmed_mean (&d))
        )
    ).collect()
}

/// Compute a mean of the execution time vector `data`, excluding the 10% topmost and 10%
/// bottommost outliers.
fn trimmed_mean (data: &[Duration]) -> Duration {

    let trim_size = data.len() / 10;
    let trimmed_data_len = data.len()-trim_size*2;

    let sorted: Duration = data.iter()
        .sorted()
        .skip(trim_size)
        .take(trimmed_data_len)
        .sum();

    sorted.div(trimmed_data_len as u32)
}

pub fn make_svg (benchmark_result: &BenchmarkResult) {

    let svg_width = 1024;
    let svg_height = 512;

    // Création du document SVG
    let mut document = Document::new()
        .set("viewBox", (0, 0, svg_width, svg_height))
        .set("width", svg_width)
        .set("height", svg_height)
        .set("xmlns", "http://www.w3.org/2000/svg");

    let margin_left = svg_width / 15;
    let margin_top = svg_height / 10;
    let margin_bottom = svg_height / 20;
    let histo_width = svg_width-margin_left-4;
    let histo_height = svg_height-margin_top-margin_bottom;

    let bar_width = histo_width as f32 / benchmark_result.len() as f32;
    let space = bar_width / 2.0;
    let new_width = histo_width as f32 + space * benchmark_result.len() as f32;
    let bar_width = bar_width * histo_width as f32 / new_width;
    let space = space * histo_width as f32 / new_width;

    let script = Script::new(
        r#"
        function highlight(rect) {
          rect.setAttribute("stroke", "white");
          rect.setAttribute("stroke-width", "2");
        }

        // Function to remove highlight on mouseout
        function unhighlight(rect) {
          rect.setAttribute("stroke", "none");
        }
        "#,
    );
    document = document.add(script);

    let mut group = Group::new();

    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", svg_width)
        .set("height", svg_height)
        .set("fill", "rgba(255, 255, 255, 0.6)");
    group = group.add(background);

    let graph_background = Rectangle::new()
        .set("x", margin_left)
        .set("y", margin_top)
        .set("width", histo_width)
        .set("height", svg_height-margin_top-margin_bottom)
        .set("fill", "rgb(200, 200, 200)")
        .set("stroke-width", "2")
        .set("stroke", "black");
    group = group.add(graph_background);

    let labels = ["100 µs", "1 ms", "10 ms", "100 ms", "1s"];
    for y in 1..=5 {

        let y_pos = svg_height - margin_bottom - y * histo_height / 5;

        let text = Text::new(labels [y as usize -1])
            .set("x", margin_left-4)
            .set("y", y_pos)
            .set("text-anchor", "end")
            .set("dominant-baseline", "middle")
            .set("font-size", margin_bottom * 6 / 10)
            .set("font-weight", "bold")
            .set("fill", "black");
        group = group.add(text);

        if y < 5 {
            let line = Line::new()
                .set("x1", margin_left)
                .set("y1", y_pos)
                .set("x2", svg_width - 4)
                .set("y2", y_pos)
                .set("stroke-width", "1")
                .set("stroke", "rgb(150,150,150)");
            group = group.add(line);
        }
    }

    let gradient = LinearGradient::new()
        .set("id", "gradient")
        .set("gradientUnits", "userSpaceOnUse")
        .set("x1", "0%")
        .set("y1", format!("{}", svg_height-margin_bottom))
        .set("x2", "0%")
        .set("y2", format!("{}", margin_top))
        .add(Stop::new ()
            .set("offset", "0%")
            .set("stop-color", "rgb(30,30,100)")
        )
        .add(Stop::new ()
            .set("offset", "20%")
            .set("stop-color", "rgb(25,140,140)")
        )
        .add(Stop::new ()
            .set("offset", "40%")
            .set("stop-color", "rgb(50,180,80)")
        )
        .add(Stop::new ()
            .set("offset", "60%")
            .set("stop-color", "rgb(255,240,100)")
        )
        .add(Stop::new ()
            .set("offset", "80%")
            .set("stop-color", "rgb(255,70,10)")
        )
        .add(Stop::new ()
            .set("offset", "100%")
            .set("stop-color", "black")
        );
    group = group.add(gradient);

    for (&day, duration) in benchmark_result.iter() {
        let Ok(duration) = duration else { continue };

        let y = (duration.as_micros() as f32 / 10.0).log10().max(0.0);
        let bar_height = (y * histo_height as f32 / 5.0) as i32;
        let x_position = margin_left + (space / 2.0 + (day-1) as f32 * (space + bar_width)) as i32;
        let y_position = margin_top + histo_height - bar_height;

        let bar = Rectangle::new()
            .set("x", x_position)
            .set("y", y_position)
            .set("width", bar_width)
            .set("height", bar_height)
            .set("style", "cursor: pointer;")
            .set("onmouseover", "highlight(this)")
            .set("onmouseout", "unhighlight(this)")
            .set("fill", "url(#gradient)");

        let text = Text::new(day.to_string())
            .set("x", x_position + bar_width as i32 / 2)
            .set("y", svg_height - margin_bottom/2)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("font-size", margin_bottom * 6 / 10)
            .set("fill", "black");

        group = group.add(bar);
        group = group.add(text);
    }

    document = document.add(group);

    svg::save("./out/perfo-2024.svg", &document).expect("Cannot save SVG file");
    println!("Fichier SVG généré : histogram.svg");
}