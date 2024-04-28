use charming::{
    component::{Axis, Feature, Grid, Legend, SaveAsImage, Title, Toolbox, ToolboxDataZoom},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart(legend: &[&str; 2], x: Vec<String>, y: &[Vec<f32>; 2]) -> Chart {
    Chart::new()
        .title(Title::new().text("Loss Comparison"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().data(legend.into()))
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .save_as_image(SaveAsImage::new())
                    .data_zoom(ToolboxDataZoom::new()),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(x),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().name(legend[0]).data(y[0].clone()))
        .series(Line::new().name(legend[1]).data(y[1].clone()))
}
