// Button matrix dimensions.
const ROWS: usize = 8;
const COLS: usize = 8;

// Draw the Ui.
pub fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids) {
    use conrod_core::{color, widget, Colorable, Positionable, Sizeable, Widget};

    // Construct our main `Canvas` tree.
    widget::Canvas::new()
        .flow_down(&[(
            ids.footer,
            widget::Canvas::new()
                .color(color::BLUE)
                .scroll_kids_vertically(),
        )])
        .set(ids.master, ui);

    // Here we make some canvas `Tabs` in the middle column.
    widget::Text::new("Echiquier")
        .color(color::LIGHT_ORANGE)
        .font_size(48)
        .middle_of(ids.header)
        .set(ids.title, ui);
    widget::Text::new("A chessboard written in RUST")
        .color(color::BLUE.complement())
        .mid_bottom_of(ids.header)
        .set(ids.subtitle, ui);

    let footer_wh = ui.wh_of(ids.footer).unwrap();
    let mut elements = widget::Matrix::new(COLS, ROWS)
        .w_h(footer_wh[0], footer_wh[1])
        .mid_top_of(ids.footer)
        .set(ids.button_matrix, ui);
    while let Some(elem) = elements.next(ui) {
        let (r, c) = (elem.row, elem.col);
        let n = c + r * c;
        let luminance = n as f32 / (COLS * ROWS) as f32;
        let button = widget::Button::new().color(color::BLUE.with_luminance(luminance));
        for _click in elem.set(button, ui) {
            println!("Hey! {:?}", (r, c));
        }
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
pub struct Ids {
    master,
    header,
    footer,
    footer_scrollbar,
    floating_a,

    title,
    subtitle,
    button_matrix,
    bing,
}}
