use eframe::egui::{self, Color32, CornerRadius, FontId, RichText, Sense, Stroke, StrokeKind, Vec2};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Interaktiver 8-Bit-Binärrechner (1 Byte)")
            .with_inner_size([1020.0, 720.0])
            .with_min_inner_size([1020.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "8-Bit-Binärrechner",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(BinaryCalculatorApp::default()))
        }),
    )
}

struct BinaryCalculatorApp {
    value: u8,
    is_simple_mode: bool,
}

impl Default for BinaryCalculatorApp {
    fn default() -> Self {
        Self {
            value: 65,
            is_simple_mode: true,
        }
    }
}

impl BinaryCalculatorApp {
    fn is_bit_set(&self, bit_index: u8) -> bool {
        (self.value & (1 << bit_index)) != 0
    }

    fn toggle_bit(&mut self, bit_index: u8) {
        self.value ^= 1 << bit_index;
    }

    fn build_addition_equation(&self) -> String {
        let mut active_values = Vec::new();
        for bit_index in (0..8).rev() {
            if self.is_bit_set(bit_index) {
                active_values.push((1u16 << bit_index).to_string());
            }
        }

        if active_values.is_empty() {
            "0 = 0 (Alle Karten sind AUS)".to_string()
        } else {
            format!("{} = {}", active_values.join(" + "), self.value)
        }
    }

    fn ascii_description(&self) -> String {
        match self.value {
            65..=90 => format!(
                "'{}' (Großbuchstabe, {}. Buchstabe im Alphabet)",
                self.value as char,
                self.value - 64
            ),
            97..=122 => format!(
                "'{}' (Kleinbuchstabe, {}. Buchstabe im Alphabet)",
                self.value as char,
                self.value - 96
            ),
            48..=57 => format!("'{}' (Ziffer)", self.value as char),
            32 => "' ' (Leerzeichen / Space)".to_string(),
            33..=126 => format!("'{}' (Sonderzeichen)", self.value as char),
            _ => "Kein druckbares Standard-ASCII-Zeichen (Steuerzeichen / erweitert)".to_string(),
        }
    }
}

impl eframe::App for BinaryCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Num1) { self.toggle_bit(0); }
            if i.key_pressed(egui::Key::Num2) { self.toggle_bit(1); }
            if i.key_pressed(egui::Key::Num3) { self.toggle_bit(2); }
            if i.key_pressed(egui::Key::Num4) { self.toggle_bit(3); }
            if i.key_pressed(egui::Key::Num5) { self.toggle_bit(4); }
            if i.key_pressed(egui::Key::Num6) { self.toggle_bit(5); }
            if i.key_pressed(egui::Key::Num7) { self.toggle_bit(6); }
            if i.key_pressed(egui::Key::Num8) { self.toggle_bit(7); }
            if i.key_pressed(egui::Key::ArrowUp) { self.value = self.value.saturating_add(1); }
            if i.key_pressed(egui::Key::ArrowDown) { self.value = self.value.saturating_sub(1); }
            if i.key_pressed(egui::Key::ArrowLeft) { self.value <<= 1; }
            if i.key_pressed(egui::Key::ArrowRight) { self.value >>= 1; }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);

            // --- SIMPEL FULL MODE TOGGLE (Toggle Button - fixe Größe) ---
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(15, 18, 27))
                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 44, 55)))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let is_active = self.is_simple_mode;

                        // Toggle-Button: grau = Full Mode, grün = Simple Mode
                        if ui.add(egui::Button::new(
                            RichText::new(if is_active { "◉ Einfacher Modus" } else { "○ Vollständiger Modus" }).size(12.0))
                                .fill(if is_active { Color32::from_rgb(28, 96, 75) } else { Color32::from_rgb(40, 44, 55) })
                                .stroke(Stroke::new(1.5, if is_active { Color32::WHITE } else { Color32::from_rgb(71, 85, 105) }))
                        ).clicked() {
                            self.is_simple_mode = !self.is_simple_mode;
                        }

                        // Kleiner Tooltip/Label daneben (optional, für Erklärung) – fixe Größe
                        ui.label(RichText::new("↔").font(FontId::monospace(14.0)).color(Color32::from_rgb(148, 163, 184)));
                    });
                });

            ui.separator();

            // --- TITEL (fixe Fontgröße) ---
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Die Sprache der Computer: 8-Bit-Binärrechner")
                        .font(FontId::proportional(24.0))
                        .strong()
                        .color(Color32::from_rgb(147, 197, 253)),
                );
                ui.label(
                    RichText::new("Klicke auf die 8 Karten (oder drücke die Tasten 1–8), um Bits zwischen AN (1) und AUS (0) umzuschalten.")
                        .font(FontId::proportional(14.0))
                        .color(Color32::LIGHT_GRAY),
                );
            });

            ui.separator();

            // --- DIE 8 KARTEN (VON LINKS 128 BIS RECHTS 1) ---
            ui.horizontal(|ui| {
                let available_width = ui.available_width();
                let spacing = 8.0;
                let card_width = (available_width - spacing * 7.0) / 8.0;
                ui.spacing_mut().item_spacing.x = spacing;

                for bit_index in (0..8).rev() {
                    let is_on = self.is_bit_set(bit_index);
                    let place_value = 1u16 << bit_index;
                    let card_number = bit_index + 1;

                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(card_width.max(80.0), 215.0),
                        Sense::click(),
                    );

                    if response.clicked() {
                        self.toggle_bit(bit_index);
                    }

                    if response.hovered() {
                        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    let bg_color = if is_on {
                        if response.hovered() {
                            Color32::from_rgb(30, 74, 150)
                        } else {
                            Color32::from_rgb(23, 58, 117)
                        }
                    } else if response.hovered() {
                        Color32::from_rgb(45, 50, 62)
                    } else {
                        Color32::from_rgb(30, 34, 42)
                    };

                    let border_color = if is_on {
                        Color32::from_rgb(96, 165, 250)
                    } else {
                        Color32::from_rgb(71, 85, 105)
                    };

                    ui.painter().rect(
                        rect,
                        CornerRadius::same(10),
                        bg_color,
                        Stroke::new(if is_on { 2.5 } else { 1.5 }, border_color),
                        StrokeKind::Inside,
                    );

                    let mut child_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(rect.shrink(6.0))
                            .layout(egui::Layout::top_down(egui::Align::Center)),
                    );

                    // Kartennummer – fix Größe in beiden Modi
                    child_ui.label(
                        RichText::new(format!("Karte {}", card_number))
                            .font(FontId::proportional(12.0))
                            .color(Color32::from_rgb(148, 163, 184)),
                    );

                    // Potenzzahl (2^7) – fixe Größe, Platzhalter im Simple Mode
                    let pot_text = if self.is_simple_mode { " ".to_string() } else { format!("2^{}", bit_index).to_string() };
                    child_ui.label(
                        RichText::new(pot_text)
                            .font(FontId::monospace(14.0))
                            .strong()
                            .color(if is_on {
                                Color32::from_rgb(191, 219, 254)
                            } else {
                                Color32::GRAY
                            }),
                    );

                    child_ui.add_space(6.0);

                    let (led_rect, _) = child_ui.allocate_exact_size(Vec2::new(26.0, 26.0), Sense::hover());
                    let led_center = led_rect.center();
                    if is_on {
                        child_ui.painter().circle_filled(
                            led_center,
                            13.0,
                            Color32::from_rgba_unmultiplied(250, 21, 21, 70),
                        );
                        child_ui.painter().circle_filled(
                            led_center,
                            9.0,
                            Color32::from_rgb(250, 10, 11),
                        );
                    } else {
                        child_ui.painter().circle_filled(
                            led_center,
                            9.0,
                            Color32::from_rgb(55, 65, 81),
                        );
                    }

                    child_ui.add_space(6.0);

                    // Dezimalwert (immer sichtbar) – fixe Größe in beiden Modi
                    child_ui.label(
                        RichText::new(format!("{}", place_value))
                            .font(FontId::proportional(18.0))
                            .strong()
                            .color(if is_on {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(100, 116, 139)
                            }),
                    );

                    child_ui.add_space(4.0);

                    // "AN"/"AUS" – immer anzeigen (fixe Größe)
                    if is_on {
                        child_ui.label(
                            RichText::new("1")
                                .font(FontId::monospace(36.0))
                                .strong()
                                .color(Color32::from_rgb(74, 222, 128)),
                        );

                        child_ui.add_space(5.0);
                        child_ui.label(
                            RichText::new("AN")
                                .font(FontId::proportional(11.5))
                                .color(Color32::from_rgb(134, 239, 172)),
                        );
                    } else {
                        child_ui.label(
                            RichText::new("0")
                                .font(FontId::monospace(36.0))
                                .strong()
                                .color(Color32::from_rgb(248, 113, 113)),
                        );

                        child_ui.add_space(5.0);
                        child_ui.label(
                            RichText::new("AUS")
                                .font(FontId::proportional(11.5))
                                .color(Color32::DARK_GRAY),
                        );
                    }
                }
            });

            ui.add_space(12.0);

            // --- RECHENWEG & ERGEBNIS-ANZEIGE (fixe Fontgrößen) ---
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(20, 24, 33))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.columns(2, |cols| {
                        cols[0].vertical(|ui| {
                            ui.label(
                                RichText::new("Rechnung (Summe aller eingeschalteten Karten):")
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::LIGHT_GRAY),
                            );
                            ui.label(
                                RichText::new(self.build_addition_equation())
                                    .font(FontId::monospace(20.0))
                                    .strong()
                                    .color(Color32::from_rgb(250, 204, 21)),
                            );
                            ui.add_space(6.0);

                            let parity_text = if self.value % 2 == 0 {
                                "GERADE Zahl (1. Bit rechts steht auf 0)"
                            } else {
                                "UNGERADE Zahl (1. Bit rechts steht auf 1)"
                            };
                            ui.label(
                                if self.is_simple_mode {
                                    RichText::new(format!(" "))
                                        .font(FontId::proportional(13.5))
                                } else {
                                    RichText::new(format!("• Eigenschaft: {}", parity_text))
                                        .font(FontId::proportional(13.5))
                                        .color(Color32::from_rgb(148, 163, 184))
                                }
                            );

                            ui.label(
                                RichText::new(format!("• ASCII-Code:  {}", self.ascii_description()))
                                    .font(FontId::proportional(13.5))
                                    .color(Color32::from_rgb(148, 163, 184)),
                            );
                        });

                        cols[1].vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("Dezimal: {}₁₀", self.value))
                                    .font(FontId::proportional(30.0))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "Binär: {:04b} {:04b}₂   |   Hex: 0x{:02X}",
                                    self.value >> 4,
                                    self.value & 0x0F,
                                    self.value
                                ))
                                .font(FontId::monospace(16.0))
                                .color(Color32::from_rgb(96, 165, 250)),
                            );
                        });
                    });
                });

            // --- STEUERUNG: SCHIEBEREGLER & BIT-OPERATIONEN (fixe Fontgröße) ---
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Dezimalwert wählen (0–255):")
                        .font(FontId::proportional(15.0))
                        .strong(),
                );
                ui.spacing_mut().slider_width = 320.0;
                ui.add(egui::Slider::new(&mut self.value, 0..=255).suffix("₁₀"));

                ui.separator();

                if ui
                    .button("<< Links-Shift (×2)")
                    .on_hover_text("Hängt rechts eine 0 an (verdoppelt den Wert, Aufgabe 4c)")
                    .clicked()
                {
                    self.value <<= 1;
                }
                if ui
                    .button(">> Rechts-Shift (÷2)")
                    .on_hover_text("Schiebt alle Bits um 1 Stelle nach rechts (halbiert den Wert)")
                    .clicked()
                {
                    self.value >>= 1;
                }
                if ui
                    .button("Bits invertieren (~)")
                    .on_hover_text("Dreht alle Karten um (aus 0 wird 1 und aus 1 wird 0)")
                    .clicked()
                {
                    self.value = !self.value;
                }
            });

            // --- SCHNELLWAHL-BUTTONS AUS DEM ARBEITSBLATT (fixe Fontgröße) ---
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    if self.is_simple_mode {
                        RichText::new(" ")
                            .font(FontId::proportional(13.5))
                            .color(Color32::LIGHT_GRAY)
                    } else {
                        RichText::new("Beispiele aus dem Lehrbogen & Arbeitsblatt:")
                            .font(FontId::proportional(13.5))
                            .color(Color32::LIGHT_GRAY)
                    }
                );

                if !self.is_simple_mode {
                    let presets: &[(&str, u8)] = &[
                        ("Reset (0)", 0),
                        ("Lehrbogen Bsp. A (22)", 22),
                        ("Lehrbogen Bsp. B (13)", 13),
                        ("Aufgabe 1b (57)", 57),
                        ("Aufgabe 2a (42)", 42),
                        ("Aufgabe 2c (173)", 173),
                        ("ASCII 'C' (67)", 67),
                        ("ASCII 'O' (79)", 79),
                        ("ASCII 'D' (68)", 68),
                        ("ASCII 'E' (69)", 69),
                        ("Max Byte (255)", 255),
                    ];

                    for &(label, preset_val) in presets {
                        let is_active = self.value == preset_val;
                        let btn = egui::Button::new(
                            RichText::new(label).color(if is_active {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            }),
                        )
                        .fill(if is_active {
                            Color32::from_rgb(96, 165, 250)
                        } else {
                            Color32::from_rgb(51, 65, 85)
                        });

                        if ui.add(btn).clicked() {
                            self.value = preset_val;
                        }
                    }
                }
            });
        });
    }
}