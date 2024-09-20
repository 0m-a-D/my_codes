use plotters::prelude::*;

pub fn plot(
    weight: f32,
    bias: f32,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let label = format!("y = {}x + {}", weight as i32, bias as i32);

    let root = BitMapBackend::new(path, (960, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&label, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, 0f32..30f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..=10)
                .map(|x| x as f32 / 1.0)
                .map(|x| (x, x * weight + bias)),
            &RED,
        ))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
