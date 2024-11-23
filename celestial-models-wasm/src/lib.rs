use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CANVAS_WIDTH: f64 = 1200.0;
const CANVAS_HEIGHT: f64 = 600.0;
const CENTER_X: f64 = CANVAS_WIDTH / 4.0;
const CENTER_X2: f64 = 3.0 * CANVAS_WIDTH / 4.0;
const CENTER_Y: f64 = CANVAS_HEIGHT / 2.0;

// 基準となるスケール係数
const SUN_DISPLAY_RADIUS: f64 = 30.0; // 表示上の太陽の半径

#[derive(Clone, Copy)]
struct CelestialBody {
    display_radius: f64,
    orbit_radius: f64,
    color: &'static str,
    period: f64,
    name: &'static str,
}

#[derive(Clone, Copy)]
struct TrailPoint {
    x: f64,
    y: f64,
    alpha: f64,
}

#[wasm_bindgen]
pub struct CelestialModel {
    context: CanvasRenderingContext2d,
    angle: f64,
    trails: Vec<Vec<TrailPoint>>,
    show_labels: bool,
    show_orbits: bool,
}

#[wasm_bindgen]
impl CelestialModel {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<CelestialModel, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(CelestialModel {
            context,
            angle: 0.0,
            trails: vec![Vec::new(); 5],
            show_labels: true,
            show_orbits: true,
        })
    }

    fn get_bodies() -> [CelestialBody; 4] {
        [
            CelestialBody {
                display_radius: 4.0,
                orbit_radius: 50.0,
                color: "#A0522D",
                period: 0.24,
                name: "水星",
            },
            CelestialBody {
                display_radius: 7.0,
                orbit_radius: 80.0,
                color: "#DEB887",
                period: 0.615,
                name: "金星",
            },
            CelestialBody {
                display_radius: 8.0,
                orbit_radius: 120.0,
                color: "#4169E1",
                period: 1.0,
                name: "地球",
            },
            CelestialBody {
                display_radius: 6.0,
                orbit_radius: 160.0,
                color: "#CD5C5C",
                period: 1.88,
                name: "火星",
            },
        ]
    }

    fn draw_circle(&self, x: f64, y: f64, radius: f64, color: &str) {
        self.context.begin_path();
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.arc(x, y, radius, 0.0, 2.0 * PI).unwrap();
        self.context.fill();

        // 発光効果（太陽のみ）
        if color == "#FFD700" {
            self.context.begin_path();
            self.context
                .set_fill_style(&JsValue::from_str(&format!("{}40", color)));
            self.context.arc(x, y, radius * 1.5, 0.0, 2.0 * PI).unwrap();
            self.context.fill();
        }
    }

    fn draw_orbit(&self, center_x: f64, radius: f64) {
        if !self.show_orbits {
            return;
        }
        self.context.begin_path();
        self.context
            .set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.15)"));
        self.context.set_line_width(0.5);
        self.context
            .arc(center_x, CENTER_Y, radius, 0.0, 2.0 * PI)
            .unwrap();
        self.context.stroke();
    }

    fn draw_label(&self, text: &str, x: f64, y: f64) {
        if !self.show_labels {
            return;
        }
        self.context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        self.context.set_font("14px Arial");
        self.context.set_text_align("center");
        self.context.fill_text(text, x, y).unwrap();
    }

    fn update_trails(&mut self, body_index: usize, x: f64, y: f64) {
        let trail = &mut self.trails[body_index];
        trail.push(TrailPoint { x, y, alpha: 1.0 });

        for point in trail.iter_mut() {
            point.alpha *= 0.98;
        }
        trail.retain(|point| point.alpha > 0.05);

        if trail.len() > 50 {
            trail.remove(0);
        }
    }

    fn draw_trails(&self, body_index: usize, color: &str) {
        for point in self.trails[body_index].iter() {
            self.context.begin_path();
            self.context.set_fill_style(&JsValue::from_str(&format!(
                "{}{}",
                color,
                ((point.alpha * 255.0) as u8).to_string()
            )));
            self.context
                .arc(point.x, point.y, 0.5, 0.0, 2.0 * PI)
                .unwrap();
            self.context.fill();
        }
    }

    pub fn draw_frame(&mut self) {
        // 背景をクリア
        self.context
            .set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.15)"));
        self.context
            .fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

        let bodies = Self::get_bodies();

        // システムタイトル
        self.draw_label("プトレマイオス体系", CENTER_X, 30.0);
        self.draw_label("コペルニクス体系", CENTER_X2, 30.0);

        // 軌道を描画
        for body in &bodies {
            self.draw_orbit(CENTER_X, body.orbit_radius);
            self.draw_orbit(CENTER_X2, body.orbit_radius);
        }

        // プトレマイオス体系（地球中心）
        self.draw_circle(CENTER_X, CENTER_Y, 8.0, "#4169E1");
        self.draw_label("地球", CENTER_X, CENTER_Y - 15.0);

        // コペルニクス体系（太陽中心）
        self.draw_circle(CENTER_X2, CENTER_Y, SUN_DISPLAY_RADIUS, "#FFD700");
        self.draw_label("太陽", CENTER_X2, CENTER_Y - 35.0);

        // 惑星の描画
        for (i, body) in bodies.iter().enumerate() {
            // 天動説側の描画
            if body.name != "地球" {
                let angle = self.angle * body.period;
                let x = CENTER_X + body.orbit_radius * angle.cos();
                let y = CENTER_Y + body.orbit_radius * angle.sin();

                self.update_trails(i, x, y);
                self.draw_trails(i, body.color);
                self.draw_circle(x, y, body.display_radius, body.color);
                self.draw_label(body.name, x, y - body.display_radius - 5.0);
            }

            // 地動説側の描画
            let angle = self.angle * body.period;
            let x = CENTER_X2 + body.orbit_radius * angle.cos();
            let y = CENTER_Y + body.orbit_radius * angle.sin();

            self.update_trails(i, x, y);
            self.draw_trails(i, body.color);
            self.draw_circle(x, y, body.display_radius, body.color);
            self.draw_label(body.name, x, y - body.display_radius - 5.0);
        }

        // 天動説側の太陽
        let sun_x = CENTER_X + 100.0 * self.angle.cos();
        let sun_y = CENTER_Y + 100.0 * self.angle.sin();
        self.draw_circle(sun_x, sun_y, SUN_DISPLAY_RADIUS, "#FFD700");
        self.draw_label("太陽", sun_x, sun_y - SUN_DISPLAY_RADIUS - 5.0);

        // 角度の更新
        self.angle += 0.01;
        if self.angle >= 2.0 * PI {
            self.angle = 0.0;
        }
    }

    #[wasm_bindgen]
    pub fn toggle_labels(&mut self) {
        self.show_labels = !self.show_labels;
    }

    #[wasm_bindgen]
    pub fn toggle_orbits(&mut self) {
        self.show_orbits = !self.show_orbits;
    }
}
