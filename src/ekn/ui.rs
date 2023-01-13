use egui::FontFamily;

use super::{ Ekn, EknType };

pub struct EknUi {
    zoom: f32,
    duration: u32,

    max_step: u32,
    pub ekn: Ekn,
    edit: bool,

    events: u32, // Pulsation
    steps: u32,  // Durée
    rotation: u32,
    step: u32,
    pub debug: bool,
}

impl Default for EknUi {
    fn default() -> Self {
        let mut ekn_ui = Self {
            zoom: 8.0,
            duration: 8,

            max_step: 40,
            ekn: Ekn::default(),
            edit: false,

            events: 0,
            steps: 0,
            rotation: 0,
            step: 0,
            debug: false,
        };

        ekn_ui.events = ekn_ui.ekn.get_events();
        ekn_ui.steps = ekn_ui.ekn.get_steps();
        ekn_ui.rotation = ekn_ui.ekn.get_rotation();
        ekn_ui.step = ekn_ui.ekn.get_current_step();

        return ekn_ui;
    }
}

impl EknUi {
    pub fn new(k: u32, n: u32, maxn: u32, debug: bool) -> Self {
        let mut ekn_ui = Self::default();
        ekn_ui.max_step = if maxn<n { maxn } else { n };
        ekn_ui.debug = debug;
        ekn_ui.ekn.set_steps(n);
        ekn_ui.ekn.set_events(k);
        ekn_ui
    }

    pub fn ekn_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let inner_resp = ui.horizontal(|ui| {
            self.ekn_circle_ui(ui);
            ui.vertical(|ui| {
                ui.heading("Euclidian Rythm");
                if ui
                    .add(egui::Slider::new(&mut self.steps, 2..=self.max_step).text("Durée"))
                    .changed()
                {
                    (self.events, self.steps) = self.ekn.set_steps(self.steps);
                }
                if ui
                    .add(
                        egui::Slider::new(&mut self.events, 1..=self.ekn.get_steps())
                            .text("Pulsation"),
                    )
                    .changed()
                {
                    self.events = self.ekn.set_events(self.events);
                }
                if ui
                    .add(
                        egui::Slider::new(&mut self.rotation, 0..=self.ekn.get_steps())
                            .text("Rotation"),
                    )
                    .changed()
                {
                    self.rotation = self.ekn.set_rotation(self.rotation);
                }
                ui.horizontal(|ui| {
                    ui.label("Ekn Type:");
                    if ui.add(egui::Button::new("All")).clicked() {
                        self.events = self.ekn.set_euclidian(EknType::All);
                        // println!("pattern: {:?}", self.ekn.get_pattern());
                        // println!("events: {:?}", self.ekn.get_events_list());
                    }
                    if ui.add(egui::Button::new("Euclidian")).clicked() {
                        self.events = self.ekn.set_euclidian(EknType::Euclidian);
                        // println!("pattern: {:?}", self.ekn.get_pattern());
                        // println!("events: {:?}", self.ekn.get_events_list());
                    }
                    if ui.add(egui::Button::new("Non Trivial")).clicked() {
                        self.events = self.ekn.set_euclidian(EknType::EuclidianNoTrivial);
                    }
                });
                // DEBUG MODE START
                if self.debug {
                    ui.heading("Debug activated:");
                    if ui.add(egui::Button::new("Pattern to CLI")).clicked() {
                        println!("pattern: {:?}", self.ekn.get_pattern());
                        println!("events: {:?}", self.ekn.get_events_list());
                    }
                    ui.label(format!(
                        "E({},{},{}) pattern = {:?}",
                        self.events,
                        self.steps,
                        self.rotation,
                        self.ekn.get_pattern()
                    ));
                    ui.label(format!(
                        "E(*,{},*) euclidian possibility = {:?}",
                        self.steps,
                        self.ekn.get_events_list()
                    ));
                }
                // DEBUG MODE END
            });
        });
        inner_resp.response
    }

    pub fn ekn_circle_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(self.zoom, self.zoom);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            self.edit = true;
        }
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().noninteractive();
            let rect = rect.expand(visuals.expansion);
            let center = egui::pos2(rect.center().x, rect.center().y);
            let radius = 0.8 * rect.height() / 2.0;
            ui.painter()
                .circle(center, radius, visuals.bg_fill, visuals.fg_stroke);

            // calculate steps positions on circle (is_event, x, y)
            let mut steps_coord: Vec<(bool, f32, f32)> = vec![];
            let rad_spacing: f32 = 2.0 * std::f32::consts::PI / self.ekn.steps as f32;
            for i in 0..self.ekn.steps {
                let where_on_circle = rad_spacing * i as f32;
                let is_event = self.ekn.pattern_cache[i as usize] == 1;
                steps_coord.push((
                    is_event,
                    f32::cos(where_on_circle),
                    f32::sin(where_on_circle),
                ));
            }

            // specific step color
            let mut step_color = visuals.fg_stroke.clone();
            step_color.width *= 4.0;
            step_color.color = egui::Color32::LIGHT_RED;

            // specific events color
            let mut event_color = visuals.fg_stroke.clone();
            event_color.width *= 2.0;
            event_color.color = egui::Color32::DARK_RED;

            // lines
            let mut events_steps: Vec<&(bool, f32, f32)> =
                steps_coord.iter().filter(|x| x.0).collect();
            let mut first = events_steps[0];
            events_steps.rotate_left(1);
            for step in events_steps {
                ui.painter().line_segment(
                    [
                        egui::pos2(center.x + radius * first.1, center.y + radius * first.2),
                        egui::pos2(center.x + radius * step.1, center.y + radius * step.2),
                    ],
                    event_color,
                );
                first = step;
            }

            // small circle on big circle
            for (i, step) in steps_coord.iter().enumerate() {
                let center2 = egui::pos2(center.x + radius * step.1, center.y + radius * step.2);
                let radius2 = 0.05 * rect.height() / 2.0;
                if step.0 {
                    let c = if self.ekn.get_current_step() as usize == i {
                        step_color
                    } else {
                        event_color
                    };
                    ui.painter().circle(center2, radius2, visuals.bg_fill, c);
                } else {
                    let c = if self.ekn.get_current_step() as usize == i {
                        step_color
                    } else {
                        visuals.fg_stroke
                    };
                    ui.painter()
                        .circle(center2, 0.5 * radius2, visuals.bg_fill, c);
                }
            }

            let t = self.duration.to_string();
            ui.painter().text(center, egui::Align2::CENTER_CENTER, &t, egui::FontId::new(64.0, FontFamily::Monospace), egui::Color32::DARK_GREEN);

        }
        response
    }
}
