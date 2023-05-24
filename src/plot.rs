use plotly::common::Mode;
use plotly::{common::Title, layout::Axis, Layout, Plot, Scatter};
use serde::Serialize;
use std::io::Write;
use uuid::Uuid;

/// Generic plotter that contructs multiple traces into a single plot
pub fn plot<X, Y>(title: &str, lines: Vec<Box<Scatter<X, Y>>>) -> Plot
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(Title::new(title))
        .x_axis(Axis::new().title(Title::new("Time")))
        .y_axis(Axis::new().title(Title::new("")));
    plot.set_layout(layout);

    lines.into_iter().for_each(|l| plot.add_trace(l));

    plot
}

/// Generic trace generator give a data set and a list of tuples that provide a trace name and map
/// the x and y values for a plot
///
/// # Args
/// * data: Some array of data
/// * lines: A slice of tuples that can map a data item to a x,y value (trace_name, fn_map_x_axis, fn_map_y_axis)
///
pub fn lines<'a, T, X, Y>(
    data: &'a Vec<T>,
    lines: &[(&'a str, fn(&'a T) -> X, fn(&'a T) -> Y)],
) -> Vec<Box<Scatter<X, Y>>>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    lines
        .iter()
        .map(|(label, x, y)| {
            Scatter::new(data.iter().map(x).collect(), data.iter().map(y).collect())
                .name(label)
                .mode(Mode::LinesMarkers)
        })
        .collect()
}

/// Render the plots out to HTML and bundle them with a handy script to synchronize X axis zoom
/// levels, along with opening it in your default web browswer for fast viewing
pub fn show(plots: &Vec<Plot>) {
    use std::env;

    let divs: Vec<String> = plots
        .iter()
        .enumerate()
        .map(|(i, _)| format!("plot{}", i))
        .collect();
    let divs = divs.join("\",\"");

    let rendered = plots
        .iter()
        .enumerate()
        .map(|(i, p)| p.to_inline_html(Some(format!("plot{}", i).as_str())));

    let mut html = format!(
        r#"
    <!doctype html>
    <html lang=\"en\">
    <head>
        <meta charset=\"utf-8\" />
        <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
        <script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
        <script types="text/javascript">

        function relayout(ed, divs) {{
            if (Object.entries(ed).length == 0) {{return;}}
            divs.forEach((div, i) => {{
                let x = div.layout.xaxis;
                let y = div.layout.yaxis;
                var update = {{}};
                if ("xaxis.autorange" in ed && ed["xaxis.autorange"] != x.autorange) {{
                    update['xaxis.autorange']= ed["xaxis.autorange"];
                }}
                if ("yaxis.autorange" in ed && ed["yaxis.autorange"] != y.autorange) {{
                    update['yaxis.autorange'] = ed["yaxis.autorange"];
                }}
                if ("xaxis.range[0]" in ed && ed["xaxis.range[0]"] != x.range[0]) {{
                    update['xaxis.range[0]'] = ed["xaxis.range[0]"];
                }}
                if ("xaxis.range[1]" in ed && ed["xaxis.range[1]"] != x.range[1]) {{
                    update['xaxis.range[1]'] = ed["xaxis.range[1]"];
                }}
                // if ("yaxis.range[0]" in ed && ed["yaxis.range[0]"] != y.range[0]) {{
                //     update['yaxis.range[0]'] = ed["yaxis.range[0]"];
                // }}
                // if ("yaxis.range[1]" in ed && ed["yaxis.range[1]"] != y.range[1]) {{
                //     update['yaxis.range[1]'] = ed["yaxis.range[1]"];
                // }}
                Plotly.relayout(div, update);
            }});
        }}

        document.addEventListener("DOMContentLoaded", function() {{
            var divIds = ["{divs}"];
            document.divs = divIds.map(x => document.getElementById(x));

            document.divs.forEach(div => {{
                div.on("plotly_relayout", function(ed) {{relayout(ed, document.divs);}});
            }});
        }});

        </script>
    </head>
    <body>"#
    );
    rendered.for_each(|x| html.push_str(&x));
    html.push_str("</body></html>");

    // Set up the temp file with a unique filename.
    let mut temp = env::temp_dir();
    let plot_name = format!("plot_{}.html", Uuid::new_v4());
    temp.push(plot_name);

    // Save the rendered plot to the temp file.
    let temp_path = temp.to_str().unwrap();
    println!("Rendering plot to: {}", temp_path);
    let mut file = std::fs::File::create(temp_path).unwrap();
    file.write_all(html.as_bytes())
        .expect("failed to write html output");
    file.flush().unwrap();

    // Hand off the job of opening the browser to an OS-specific implementation.
    show_with_default_app(temp_path);
}

const DEFAULT_HTML_APP_NOT_FOUND: &str = r#"Could not find default application for HTML files.
Consider using the `to_html` method obtain a string representation instead. If using the `kaleido` feature the
`write_image` method can be used to produce a static image in one of the following formats:
- ImageFormat::PNG
- ImageFormat::JPEG
- ImageFormat::WEBP
- ImageFormat::SVG
- ImageFormat::PDF
- ImageFormat::EPS"#;

#[cfg(target_os = "linux")]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("xdg-open")
        .args([temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "macos")]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("open")
        .args(&[temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "windows")]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("cmd")
        .arg("/C")
        .arg(format!(r#"start {}"#, temp_path))
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}
