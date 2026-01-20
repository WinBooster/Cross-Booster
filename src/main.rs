mod power;
mod registry;
mod utils;

rust_i18n::i18n!("locales");

use std::sync::Arc;
use std::thread;

use eframe::egui;
use eframe::egui::IconData;
use image::{ImageError, ImageReader};
use rust_i18n::t;

use crate::utils::{get_icon, run_cmd_hidden};

struct MyApp {
    #[cfg(target_os = "windows")]
    high_performance_enabled: bool,
    #[cfg(target_os = "windows")]
    performance_optimized: bool,
    #[cfg(target_os = "windows")]
    initialized: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            high_performance_enabled: false,
            #[cfg(target_os = "windows")]
            performance_optimized: false,
            #[cfg(target_os = "windows")]
            initialized: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "windows")] {
            egui::CentralPanel::default().show(ctx, |ui| {
                if !self.initialized {
                    match crate::power::get_current_power_scheme_guid() {
                        Ok(guid) => {
                            self.high_performance_enabled =
                                guid == "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
                        }
                        Err(_) => {}
                    }
                    match crate::registry::RegistryUtils::get_performance_visual_fx_setting() {
                        Ok(value) => {
                            self.performance_optimized = value == 2;
                        }
                        Err(_) => {
                            self.performance_optimized = false;
                        }
                    }
                    self.initialized = true;
                }

                let mut enabled = self.high_performance_enabled;
                if ui
                    .checkbox(&mut enabled, t!("high_performance_checkbox"))
                    .changed()
                {
                    self.high_performance_enabled = enabled;
                    if enabled {
                        // Switch to high performance
                        let guid = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
                        match crate::power::create_scheme_if_not_exists(guid) {
                            Ok(_) => {}
                            Err(_) => {
                                return;
                            }
                        }
                        match run_cmd_hidden(&format!("powercfg /setactive {}", guid)) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    } else {
                        // Switch back to balanced
                        let balanced_guid = "381b4222-f694-41f0-9685-ff5bb260df2e";
                        match run_cmd_hidden(&format!("powercfg /setactive {}", balanced_guid)) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    // Clean up duplicates on every checkbox change
                    match crate::power::cleanup_duplicate_schemes() {
                        Ok(_) => {}
                        Err(_) => {} // Ignore errors for cleanup
                    }
                }

                let mut optimized = self.performance_optimized;
                if ui
                    .checkbox(&mut optimized, t!("optimize_performance_checkbox"))
                    .changed()
                {
                    self.performance_optimized = optimized;
                    if optimized {
                        match crate::registry::RegistryUtils::set_performance_visual_fx_setting(2) {
                            Ok(_) => {
                                // Restart explorer to apply changes in background
                                thread::spawn(|| {
                                    let _ = run_cmd_hidden("taskkill /f /im explorer.exe && start explorer.exe");
                                });
                            }
                            Err(_) => {}
                        }
                    } else {
                        match crate::registry::RegistryUtils::set_performance_visual_fx_setting(1) {
                            Ok(_) => {
                                // Restart explorer to apply changes in background
                                thread::spawn(|| {
                                    let _ = run_cmd_hidden("taskkill /f /im explorer.exe && start explorer.exe");
                                });
                            }
                            Err(_) => {}
                        }
                    }
                }
            });
        }
    }
}

fn load_icon_from_bytes(bytes: &[u8]) -> Result<Arc<IconData>, image::ImageError> {
    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    Ok(Arc::new(IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    }))
}

fn main() -> eframe::Result<()> {
    rust_i18n::set_locale(crate::utils::get_system_language());

    let icon_bytes = get_icon();
    let icon = load_icon_from_bytes(icon_bytes).expect("Failed to load icon");

    let size = egui::vec2(450.0, 200.0);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        &format!("Cross Booster v{}", crate::utils::get_version()),
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
