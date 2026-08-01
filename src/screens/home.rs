use std::time::{Duration, Instant};

use eframe::egui::{Align, Color32, Direction, Layout, RichText, Slider, Ui};
use egui_extras::{Size, StripBuilder};

use crate::array::{BubbleSort, SortableArray};

const DEFAULT_MIN_RANGE: u8 = 1;
const DEFAULT_MAX_RANGE: u8 = 30;

pub struct HomeScreen {
    array: SortableArray<BubbleSort>,
    bar_count: usize,
    last_execution_time: Option<Duration>,
}

impl HomeScreen {
    pub fn new(bar_count: usize) -> Self {
        Self {
            array: SortableArray::generate_random_data(
                bar_count,
                DEFAULT_MAX_RANGE,
                DEFAULT_MIN_RANGE,
            ),
            bar_count,
            last_execution_time: None,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let bg_color = ui.stack().bg_color();

        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    if ui.button("Generate New Set").clicked() {
                        self.array.shuffle_new_set(Some(self.bar_count));
                    }
                    ui.add(Slider::new(&mut self.bar_count, 1..=50).text("Sample count"));
                    ui.with_layout(
                        Layout::from_main_dir_and_cross_align(Direction::BottomUp, Align::Center)
                            .with_cross_justify(true),
                        |ui| {
                            ui.separator();
                            if ui.button(RichText::new("Sort").size(24.0)).clicked()
                                && !self.array.is_sorted()
                            {
                                let start_time = Instant::now();

                                self.array.sort();

                                let end_time = start_time.elapsed();
                                self.last_execution_time = Some(end_time);
                            }
                            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                if let Some(time) = self.last_execution_time {
                                    ui.label(format!("Last execution time: {:.2?}", time));
                                }
                            });
                        },
                    );
                });
                strip.strip(|builder| {
                    builder
                        .sizes(Size::remainder(), self.array.length)
                        .horizontal(|mut strip| {
                            for data in self.array.data.borrow().iter() {
                                strip.strip(|builder| {
                                    let value = data.value as f32;
                                    let fraction = 1.0 / value;

                                    builder
                                        .size(Size::relative(fraction))
                                        .size(Size::remainder())
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.painter().rect_filled(
                                                    ui.available_rect_before_wrap(),
                                                    0.0,
                                                    bg_color,
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.painter().rect_filled(
                                                    ui.available_rect_before_wrap(),
                                                    0.0,
                                                    Color32::YELLOW,
                                                );
                                                ui.with_layout(
                                                    Layout::bottom_up(Align::Center),
                                                    |ui| {
                                                        ui.label(format!("{}", value));
                                                    },
                                                );
                                            });
                                        });
                                });
                            }
                        });
                });
            });
    }
}
