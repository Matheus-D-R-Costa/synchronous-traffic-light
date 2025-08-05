use std::time::Instant;
use eframe::{egui, App, Frame};
use egui::{Color32, Context, RichText, Vec2};

use crate::constants::*;
use crate::state::TrafficLightColor;
use crate::ui::draw_traffic_light;

pub struct TrafficLightApp {
    color_a: TrafficLightColor,
    color_b: TrafficLightColor,
    simulated_time_seconds: f64,
    state_timer_seconds: f64,
    total_latency_seconds: f64,
    simulation_complete: bool,
    last_update: Instant
}

impl Default for TrafficLightApp {
    fn default() -> Self {
        Self {
            color_a: TrafficLightColor::Green,
            color_b: TrafficLightColor::Red,
            simulated_time_seconds: 0.0,
            state_timer_seconds: 0.0,
            total_latency_seconds: 0.0,
            simulation_complete: false,
            last_update: Instant::now(),
        }
    }
}

impl App for TrafficLightApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.run_simulation_step();
        self.draw_ui(ctx);
        ctx.request_repaint();
    }
}

impl TrafficLightApp {
    fn run_simulation_step(&mut self) {
        if self.simulation_complete {
            return;
        }

        let delta_time = self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();

        let simulated_delta = delta_time * SIMULATION_SPEEDUP_FACTOR;
        self.simulated_time_seconds += simulated_delta;
        self.state_timer_seconds += simulated_delta;

        let target_duration = if self.simulated_time_seconds < SECONDS_IN_12_HOURS {
            SECONDS_IN_A_MINUTE
        } else {
            SECONDS_IN_3_MINUTES
        };

        if self.state_timer_seconds >= target_duration {
            let latency_this_step = target_duration * (LATENCY_FACTOR - 1.0);
            self.total_latency_seconds += latency_this_step;
            self.state_timer_seconds -= target_duration;

            let (next_a, next_b) = match (self.color_a, self.color_b) {
                (TrafficLightColor::Green, TrafficLightColor::Red) => (TrafficLightColor::Yellow, TrafficLightColor::Red),
                (TrafficLightColor::Yellow, TrafficLightColor::Red) => (TrafficLightColor::Red, TrafficLightColor::Green),
                (TrafficLightColor::Red, TrafficLightColor::Green) => (TrafficLightColor::Red, TrafficLightColor::Yellow),
                (TrafficLightColor::Red, TrafficLightColor::Yellow) => (TrafficLightColor::Green, TrafficLightColor::Red),
                _ => (self.color_a, self.color_b),
            };
            self.color_a = next_a;
            self.color_b = next_b;
        }

        if self.simulated_time_seconds >= SECONDS_IN_24_HOURS {
            self.simulation_complete = true;
            self.simulated_time_seconds = SECONDS_IN_24_HOURS;
        }
    }

    fn draw_ui(&self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synchronous Traffic Light Simulator");
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(RichText::new("Simulation Status").size(FONT_SIZE_LARGE));
                ui.add_space(5.0);
                
                let hours = (self.simulated_time_seconds / 3600.0).floor();
                let minutes = ((self.simulated_time_seconds % 3600.0) / 60.0).floor();
                let seconds = self.simulated_time_seconds % 60.0;
                ui.label(format!("Simulated Time: {hours:02.0}:{minutes:02.0}:{seconds:04.1}"));

                if self.simulation_complete {
                    ui.label(RichText::new("SIMULATION COMPLETED (24h)").color(Color32::GREEN).size(FONT_SIZE_MEDIUM));
                }
            });

            ui.add_space(20.0);
            
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(30.0, 0.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Traffic light A").size(FONT_SIZE_LARGE));
                    draw_traffic_light(ui, self.color_a);
                });
                ui.group(|ui| {
                    ui.label(RichText::new("Traffic light B").size(FONT_SIZE_LARGE));
                    draw_traffic_light(ui, self.color_b);
                });
            });

            ui.add_space(20.0);
            
            ui.group(|ui| {
                ui.label(RichText::new("Latency Calculation").size(FONT_SIZE_LARGE));
                ui.add_space(5.0);
                ui.label(format!("Accumulated Latency: {:.8} seconds", self.total_latency_seconds));
                
                if self.simulation_complete {
                    let total_latency_minutes = self.total_latency_seconds / 60.0;
                    ui.separator();
                    ui.label(RichText::new(format!("Final Total Latency: {total_latency_minutes:.5} minutes")).size(FONT_SIZE_MEDIUM).color(Color32::YELLOW).strong());
                }
            });
        });
    }
}