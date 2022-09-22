use crate::graphics::{
    components::{draw_button, draw_horizontal_slider},
    PowderState,
};
use femtovg::{renderer::OpenGl, Align, Baseline, Canvas, Color, LineCap, LineJoin, Paint};
use powder::Meta;

pub fn render_market_listings(
    canvas: &mut Canvas<OpenGl>,
    meta: &mut Meta,
    state: &mut PowderState,
) {
    // Render tab bar
    let x = 200.0;
    let y = 15.0;
    let tab_width = 120.0;
    let tab_height = 32.0;
    let text_padding = 15.0;
    render_tabs(canvas, meta, state, x, y, tab_width, tab_height);

    // Render selected tab
    let tab = state.dynamic_state.get("market_info_tab");
    let y = y + tab_height + 25.0;
    match tab {
        Some(tab) if tab == "Actors" => {
            render_actors(canvas, meta, state, x, y, text_padding);
        }
        _ => {
            let width = 500.0;
            render_listings(canvas, meta, state, x, y, width, text_padding);
        }
    }
}

fn render_tabs(
    canvas: &mut Canvas<OpenGl>,
    meta: &mut Meta,
    state: &mut PowderState,
    x: f32,
    y: f32,
    tab_width: f32,
    tab_height: f32,
) {
    let tabs = vec!["Listings", "Actors"];
    for (i, tab) in tabs.iter().enumerate() {
        if draw_button(
            canvas,
            meta,
            state,
            x + tab_width * i as f32,
            y,
            tab_width,
            tab_height,
            tab,
        ) {
            state
                .dynamic_state
                .insert("market_info_tab".to_string(), tab.to_string());
        }
    }
}

fn render_listings(
    canvas: &mut Canvas<OpenGl>,
    _meta: &mut Meta,
    state: &mut PowderState,
    x: f32,
    y: f32,
    width: f32,
    text_padding: f32,
) {
    let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
    paint.set_line_cap(LineCap::Butt);
    paint.set_line_join(LineJoin::Bevel);
    paint.set_line_width(4.0);
    paint.set_font_size(14.0);
    paint.set_font(&[state.font.unwrap()]);
    paint.set_text_baseline(Baseline::Middle);

    let maximum_count = state
        .renderable_state
        .listed_item_kinds
        .iter()
        .reduce(|acc, cur| if cur.1 > acc.1 { cur } else { acc })
        .map(|res| res.1);
    if maximum_count.is_none() {
        return;
    }
    let maximum_count = maximum_count.unwrap();

    for (i, (item_kind, count)) in state.renderable_state.listed_item_kinds.iter().enumerate() {
        let x = x + 64.0;
        let vertical_offset = y + i as f32 * 30.0;
        paint.set_text_align(Align::Right);
        let _ = canvas.fill_text(x, vertical_offset, format!("{}", item_kind), paint);
        draw_horizontal_slider(
            canvas,
            x + text_padding,
            vertical_offset,
            width,
            0.0,
            maximum_count as f32,
            *count as f32,
        );
        paint.set_text_align(Align::Left);
        let _ = canvas.fill_text(
            x + text_padding * 2.0 + width,
            vertical_offset,
            format!("{}", count),
            paint,
        );
    }
}

fn render_actors(
    canvas: &mut Canvas<OpenGl>,
    _meta: &mut Meta,
    state: &mut PowderState,
    x: f32,
    y: f32,
    text_padding: f32,
) {
    let indentation = 15.0;

    let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
    paint.set_font_size(14.0);
    paint.set_font(&[state.font.unwrap()]);
    paint.set_text_baseline(Baseline::Middle);
    paint.set_text_align(Align::Left);

    // Render chosen actor log
    for (actor_name, actor_log) in state.renderable_state.actor_logs.iter().last() {
        let _ = canvas.fill_text(x + text_padding, y, format!("{}", actor_name), paint);
        for (i, entry) in actor_log.iter().enumerate() {
            let vertical_offset = y + (i + 1) as f32 * 30.0;
            let _ = canvas.fill_text(
                x + text_padding + indentation,
                vertical_offset,
                format!("{}", entry),
                paint,
            );
        }
    }
}
