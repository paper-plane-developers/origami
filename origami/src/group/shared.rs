use super::*;

pub(super) fn layout(widget: &gtk::Widget, width: i32, spacing: f32) -> Vec<ChildWrapper> {
    let children: Vec<_> = widget.iter_children().map(ChildWrapper::new).collect();

    let aspect_ratios = children.iter().map(|child| child.aspect_ratio);

    let proportions: String = aspect_ratios
        .clone()
        .map(|ar| {
            if ar > 1.2 {
                "w"
            } else if ar < 0.8 {
                "n"
            } else {
                "q"
            }
        })
        .collect();

    let average_aspect_ratio = aspect_ratios.clone().sum::<f32>() / children.len() as f32;

    let force_calc = aspect_ratios.clone().any(|ar| ar > 2.0);

    let layout_function = layout_function(children.len(), force_calc);

    let width = width as f32;

    layout_function(
        &children,
        &proportions,
        average_aspect_ratio,
        width,
        spacing,
    );

    children
}
