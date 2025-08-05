use crate::state::TrafficLightColor;
use eframe::egui::{Ui, Color32, Vec2, Stroke};

pub fn draw_traffic_light(ui: &mut Ui, active_color: TrafficLightColor) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(80.0, 240.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 5.0, Color32::DARK_GRAY);

    let draw_light = |center: egui::Pos2, color: Color32| {
        ui.painter().circle_filled(center, 30.0, color);
        ui.painter().circle_stroke(center, 30.0, Stroke::new(1.0, Color32::BLACK));
    };
    
    let red_center = rect.center_top() + Vec2::new(0.0, 50.0);
    let yellow_center = rect.center();
    let green_center = rect.center_bottom() - Vec2::new(0.0, 50.0);

    let red_light_color = if active_color == TrafficLightColor::Red { Color32::RED } else { Color32::from_rgb(50, 0, 0) };
    let yellow_light_color = if active_color == TrafficLightColor::Yellow { Color32::YELLOW } else { Color32::from_rgb(50, 50, 0) };
    let green_light_color = if active_color == TrafficLightColor::Green { Color32::GREEN } else { Color32::from_rgb(0, 50, 0) };

    draw_light(red_center, red_light_color);
    draw_light(yellow_center, yellow_light_color);
    draw_light(green_center, green_light_color);
}