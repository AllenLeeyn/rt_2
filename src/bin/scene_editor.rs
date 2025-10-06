use eframe::{self, egui};
use std::fs;
use serde_json;

// Import the SceneData and related structs from the main project
use rt_2::scene::storage::{SceneData, CameraData, ObjectData, LightData, SphereData, PlaneData, CubeData, CylinderData, TextureData, PointLightData, DirectionalLightData};
use rt_2::core::vec3::{Point3, Vec3};
use rt_2::core::color::Color;

struct SceneEditorApp {
    scene_data: SceneData,
    json_string: String,
    error_message: Option<String>,
}

impl SceneEditorApp {
    fn update_json_string(&mut self) {
        match serde_json::to_string_pretty(&self.scene_data) {
            Ok(json) => {
                self.json_string = json;
                self.error_message = None;
            },
            Err(e) => {
                self.error_message = Some(format!("Failed to serialize scene data: {}", e));
            }
        }
    }

    fn draw_scene_2d(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.rect_filled(rect, 0.0, egui::Color32::DARK_GRAY);

        // Define a coordinate system for the 2D view
        // For a top-down view (X-Z plane), X maps to screen X, Z maps to screen Y
        // We need to scale and translate to fit the scene within the rect
        let _scene_center_x = 0.0; // Assuming scene center is at (0,0,0)
        let _scene_center_z = 0.0;
        let scene_scale = 20.0; // Pixels per unit in scene

        let to_screen_pos = |p: Point3| {
            let screen_x = rect.center().x + p.x * scene_scale;
            let screen_y = rect.center().y + p.z * scene_scale; // Invert Y for top-down view
            egui::pos2(screen_x, screen_y)
        };

        // Draw Camera
        let cam_pos_2d = to_screen_pos(self.scene_data.camera.position);
        painter.circle_filled(cam_pos_2d, 5.0, egui::Color32::WHITE);
        let look_at_pos_2d = to_screen_pos(self.scene_data.camera.look_at);
        painter.line_segment([cam_pos_2d, look_at_pos_2d], egui::Stroke::new(1.0, egui::Color32::WHITE));

        // Draw Lights
        for light in &self.scene_data.lights {
            match light {
                LightData::Point(point_light) => {
                    let light_pos_2d = to_screen_pos(point_light.position);
                    painter.circle_filled(light_pos_2d, 4.0, egui::Color32::YELLOW);
                },
                LightData::Directional(directional_light) => {
                    let light_dir_start = Point3::new(0.0, 0.0, 0.0); // Arbitrary start for arrow
                    let light_dir_end = light_dir_start + directional_light.direction * 2.0; // Arrow length
                    let start_pos_2d = to_screen_pos(light_dir_start);
                    let end_pos_2d = to_screen_pos(light_dir_end);
                    painter.arrow(start_pos_2d, end_pos_2d - start_pos_2d, egui::Stroke::new(2.0, egui::Color32::GOLD));
                },
            }
        }

        // Draw Objects
        for object in &self.scene_data.objects {
            match object {
                ObjectData::Sphere(sphere) => {
                    let center_2d = to_screen_pos(sphere.center);
                    let radius_pixels = sphere.radius * scene_scale;
                    painter.circle_stroke(center_2d, radius_pixels, egui::Stroke::new(1.0, egui::Color32::BLUE));
                },
                ObjectData::Plane(plane) => {
                    let center_2d = to_screen_pos(plane.center);
                    let half_size_x = plane.size.x * scene_scale / 2.0;
                    let half_size_z = plane.size.z * scene_scale / 2.0;
                    let rect_min = egui::pos2(center_2d.x - half_size_x, center_2d.y - half_size_z);
                    let rect_max = egui::pos2(center_2d.x + half_size_x, center_2d.y + half_size_z);
                    painter.rect_stroke(egui::Rect::from_min_max(rect_min, rect_max), 0.0, egui::Stroke::new(1.0, egui::Color32::GREEN));
                },
                ObjectData::Cube(cube) => {
                    let center_2d = to_screen_pos(cube.center);
                    let half_size = cube.size * scene_scale / 2.0;
                    let rect_min = egui::pos2(center_2d.x - half_size, center_2d.y - half_size);
                    let rect_max = egui::pos2(center_2d.x + half_size, center_2d.y + half_size);
                    painter.rect_stroke(egui::Rect::from_min_max(rect_min, rect_max), 0.0, egui::Stroke::new(1.0, egui::Color32::RED));
                },
                ObjectData::Cylinder(cylinder) => {
                    let center_2d = to_screen_pos(cylinder.center);
                    let radius_pixels = cylinder.radius * scene_scale;
                    painter.circle_stroke(center_2d, radius_pixels, egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)));
                },
            }
        }
    }
}

fn texture_editor(ui: &mut egui::Ui, texture: &mut TextureData, scene_changed: &mut bool) {
    let mut current_texture_type = match texture {
        TextureData::SolidColor(_) => "SolidColor",
        TextureData::Gradient(_, _, _) => "Gradient",
        TextureData::Checkerboard(_, _, _) => "Checkerboard",
        TextureData::Image(_) => "Image",
    };

    ui.horizontal(|ui| {
        if ui.radio_value(&mut current_texture_type, "SolidColor", "Solid Color").changed() {
            *texture = TextureData::SolidColor(Color::WHITE);
            *scene_changed = true;
        }
        if ui.radio_value(&mut current_texture_type, "Gradient", "Gradient").changed() {
            *texture = TextureData::Gradient(Color::WHITE, Color::BLACK, 0.0);
            *scene_changed = true;
        }
        if ui.radio_value(&mut current_texture_type, "Checkerboard", "Checkerboard").changed() {
            *texture = TextureData::Checkerboard(Color::WHITE, Color::BLACK, 1.0);
            *scene_changed = true;
        }
        if ui.radio_value(&mut current_texture_type, "Image", "Image").changed() {
            *texture = TextureData::Image(String::new());
            *scene_changed = true;
        }
    });

    ui.indent("texture_indent", |ui| {
        match texture {
            TextureData::SolidColor(color) => {
                ui.label("Color:");
                ui.horizontal(|ui| {
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color.r).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color.g).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color.b).speed(0.01).range(0.0..=1.0)).changed();
                });
            },
            TextureData::Gradient(color1, color2, angle) => {
                ui.label("Color 1:");
                ui.horizontal(|ui| {
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.r).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.g).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.b).speed(0.01).range(0.0..=1.0)).changed();
                });
                ui.label("Color 2:");
                ui.horizontal(|ui| {
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.r).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.g).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.b).speed(0.01).range(0.0..=1.0)).changed();
                });
                ui.label("Angle:");
                *scene_changed |= ui.add(egui::DragValue::new(&mut *angle).speed(0.1)).changed();
            },
            TextureData::Checkerboard(color1, color2, frequency) => {
                ui.label("Color 1:");
                ui.horizontal(|ui| {
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.r).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.g).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color1.b).speed(0.01).range(0.0..=1.0)).changed();
                });
                ui.label("Color 2:");
                ui.horizontal(|ui| {
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.r).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.g).speed(0.01).range(0.0..=1.0)).changed();
                    *scene_changed |= ui.add(egui::DragValue::new(&mut color2.b).speed(0.01).range(0.0..=1.0)).changed();
                });
                ui.label("Frequency:");
                *scene_changed |= ui.add(egui::DragValue::new(&mut *frequency).speed(0.1)).changed();
            },
            TextureData::Image(path) => {
                ui.label("Path:");
                *scene_changed |= ui.text_edit_singleline(path).changed();
            },
        }
    });
}

impl Default for SceneEditorApp {
    fn default() -> Self {
        let default_camera = CameraData {
            position: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 90.0,
            aspect_ratio: 1.777,
            resolution: (400, 300),
        };
        let mut app = Self {
            scene_data: SceneData {
                objects: Vec::new(),
                lights: Vec::new(),
                camera: default_camera,
            },
            json_string: String::new(),
            error_message: None,
        };
        app.update_json_string(); // Initialize json_string with default scene_data
        app
    }
}

impl eframe::App for SceneEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut scene_changed = false;

        egui::SidePanel::left("scene_editor_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Scene Editor");

                if ui.button("Load scene.json").clicked() {
                    match fs::read_to_string("scene.json") {
                        Ok(data) => {
                            self.json_string = data.clone();
                            match serde_json::from_str::<SceneData>(&data) {
                                Ok(scene) => {
                                    self.scene_data = scene;
                                    self.error_message = None;
                                    scene_changed = true; // Indicate change after loading
                                },
                                Err(e) => {
                                    self.error_message = Some(format!("Failed to parse scene.json: {}", e));
                                }
                            }
                        },
                        Err(e) => {
                            self.error_message = Some(format!("Failed to read scene.json: {}", e));
                        }
                    }
                }

                if ui.button("Save scene.json").clicked() {
                    match serde_json::to_string_pretty(&self.scene_data) {
                        Ok(json) => {
                            match fs::write("scene.json", json) {
                                Ok(_) => {
                                    self.error_message = None;
                                },
                                Err(e) => {
                                    self.error_message = Some(format!("Failed to write scene.json: {}", e));
                                }
                            }
                        },
                        Err(e) => {
                            self.error_message = Some(format!("Failed to serialize scene data: {}", e));
                        }
                    }
                }

                if let Some(msg) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, msg);
                }

                ui.separator();

                // Camera Editor
                ui.collapsing("Camera", |ui| {
                    ui.label("Position:");
                    ui.horizontal(|ui| {
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.position.x).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.position.y).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.position.z).speed(0.1)).changed();
                    });

                    ui.label("Look At:");
                    ui.horizontal(|ui| {
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.look_at.x).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.look_at.y).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.look_at.z).speed(0.1)).changed();
                    });

                    ui.label("Up Vector:");
                    ui.horizontal(|ui| {
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.up.x).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.up.y).speed(0.1)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.up.z).speed(0.1)).changed();
                    });

                    ui.label("FOV:");
                    scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.fov).speed(1.0)).changed();

                    ui.label("Aspect Ratio:");
                    scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.aspect_ratio).speed(0.01)).changed();

                    ui.label("Resolution:");
                    ui.horizontal(|ui| {
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.resolution.0).speed(1.0)).changed();
                        scene_changed |= ui.add(egui::DragValue::new(&mut self.scene_data.camera.resolution.1).speed(1.0)).changed();
                    });
                });

                ui.separator();

                // Lights Editor
                ui.collapsing("Lights", |ui| {
                    let mut light_to_remove = None;
                    for (i, light) in self.scene_data.lights.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading(format!("Light {}", i));
                                    if ui.button("Remove").clicked() {
                                        light_to_remove = Some(i);
                                        scene_changed = true;
                                    }
                                });

                                let mut current_light_type = match light {
                                    LightData::Point(_) => "Point",
                                    LightData::Directional(_) => "Directional",
                                };

                                ui.horizontal(|ui| {
                                    if ui.radio_value(&mut current_light_type, "Point", "Point").changed() {
                                        *light = LightData::Point(PointLightData::default());
                                        scene_changed = true;
                                    }
                                    if ui.radio_value(&mut current_light_type, "Directional", "Directional").changed() {
                                        *light = LightData::Directional(DirectionalLightData::default());
                                        scene_changed = true;
                                    }
                                });

                                match light {
                                    LightData::Point(point_light) => {
                                        ui.label("Position:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.position.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.position.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.position.z).speed(0.1)).changed();
                                        });

                                        ui.label("Color:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.color.r).speed(0.01).range(0.0..=1.0)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.color.g).speed(0.01).range(0.0..=1.0)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut point_light.color.b).speed(0.01).range(0.0..=1.0)).changed();
                                        });

                                        ui.label("Intensity:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut point_light.intensity).speed(0.1)).changed();

                                        ui.label("Samples:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut point_light.samples).speed(1.0)).changed();

                                        ui.label("Radius:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut point_light.radius).speed(0.1)).changed();

                                        ui.label("Softness:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut point_light.softness).speed(0.1)).changed();
                                    },
                                    LightData::Directional(directional_light) => {
                                        ui.label("Direction:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.direction.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.direction.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.direction.z).speed(0.1)).changed();
                                        });

                                        ui.label("Color:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.color.r).speed(0.01).range(0.0..=1.0)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.color.g).speed(0.01).range(0.0..=1.0)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.color.b).speed(0.01).range(0.0..=1.0)).changed();
                                        });

                                        ui.label("Intensity:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut directional_light.intensity).speed(0.1)).changed();
                                    },
                                }
                            });
                        });
                    }

                    if let Some(i) = light_to_remove {
                        self.scene_data.lights.remove(i);
                        scene_changed = true;
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Add Point Light").clicked() {
                            self.scene_data.lights.push(LightData::Point(PointLightData::default()));
                            scene_changed = true;
                        }
                        if ui.button("Add Directional Light").clicked() {
                            self.scene_data.lights.push(LightData::Directional(DirectionalLightData::default()));
                            scene_changed = true;
                        }
                    });
                });

                ui.separator();

                // Objects Editor
                ui.collapsing("Objects", |ui| {
                    let mut object_to_remove = None;
                    for (i, object) in self.scene_data.objects.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading(format!("Object {}", i));
                                    if ui.button("Remove").clicked() {
                                        object_to_remove = Some(i);
                                        scene_changed = true;
                                    }
                                });

                                let mut current_object_type = match object {
                                    ObjectData::Sphere(_) => "Sphere",
                                    ObjectData::Plane(_) => "Plane",
                                    ObjectData::Cube(_) => "Cube",
                                    ObjectData::Cylinder(_) => "Cylinder",
                                };

                                ui.horizontal(|ui| {
                                    if ui.radio_value(&mut current_object_type, "Sphere", "Sphere").changed() {
                                        *object = ObjectData::Sphere(SphereData::default());
                                        scene_changed = true;
                                    }
                                    if ui.radio_value(&mut current_object_type, "Plane", "Plane").changed() {
                                        *object = ObjectData::Plane(PlaneData::default());
                                        scene_changed = true;
                                    }
                                    if ui.radio_value(&mut current_object_type, "Cube", "Cube").changed() {
                                        *object = ObjectData::Cube(CubeData::default());
                                        scene_changed = true;
                                    }
                                    if ui.radio_value(&mut current_object_type, "Cylinder", "Cylinder").changed() {
                                        *object = ObjectData::Cylinder(CylinderData::default());
                                        scene_changed = true;
                                    }
                                });

                                match object {
                                    ObjectData::Sphere(sphere) => {
                                        ui.label("Center:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut sphere.center.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut sphere.center.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut sphere.center.z).speed(0.1)).changed();
                                        });
                                        ui.label("Radius:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut sphere.radius).speed(0.1)).changed();
                                        ui.group(|ui| {
                                            ui.label("Texture:");
                                            texture_editor(ui, &mut sphere.texture, &mut scene_changed);
                                        });
                                    },
                                    ObjectData::Plane(plane) => {
                                        ui.label("Center:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.center.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.center.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.center.z).speed(0.1)).changed();
                                        });
                                        ui.label("Size:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.size.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.size.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut plane.size.z).speed(0.1)).changed();
                                        });
                                        ui.group(|ui| {
                                            ui.label("Texture:");
                                            texture_editor(ui, &mut plane.texture, &mut scene_changed);
                                        });
                                    },
                                    ObjectData::Cube(cube) => {
                                        ui.label("Center:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cube.center.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cube.center.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cube.center.z).speed(0.1)).changed();
                                        });
                                        ui.label("Size:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut cube.size).speed(0.1)).changed();
                                        ui.group(|ui| {
                                            ui.label("Texture:");
                                            texture_editor(ui, &mut cube.texture, &mut scene_changed);
                                        });
                                    },
                                    ObjectData::Cylinder(cylinder) => {
                                        ui.label("Center:");
                                        ui.horizontal(|ui| {
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cylinder.center.x).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cylinder.center.y).speed(0.1)).changed();
                                            scene_changed |= ui.add(egui::DragValue::new(&mut cylinder.center.z).speed(0.1)).changed();
                                        });
                                        ui.label("Radius:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut cylinder.radius).speed(0.1)).changed();
                                        ui.label("Height:");
                                        scene_changed |= ui.add(egui::DragValue::new(&mut cylinder.height).speed(0.1)).changed();
                                        ui.group(|ui| {
                                            ui.label("Texture:");
                                            texture_editor(ui, &mut cylinder.texture, &mut scene_changed);
                                        });
                                    },
                                }
                            });
                        });
                    }

                    if let Some(i) = object_to_remove {
                        self.scene_data.objects.remove(i);
                        scene_changed = true;
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Add Sphere").clicked() {
                            self.scene_data.objects.push(ObjectData::Sphere(SphereData::default()));
                            scene_changed = true;
                        }
                        if ui.button("Add Plane").clicked() {
                            self.scene_data.objects.push(ObjectData::Plane(PlaneData::default()));
                            scene_changed = true;
                        }
                        if ui.button("Add Cube").clicked() {
                            self.scene_data.objects.push(ObjectData::Cube(CubeData::default()));
                            scene_changed = true;
                        }
                        if ui.button("Add Cylinder").clicked() {
                            self.scene_data.objects.push(ObjectData::Cylinder(CylinderData::default()));
                            scene_changed = true;
                        }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_scene_2d(ui);
        });

        if scene_changed {
            self.update_json_string();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Scene Editor",
        options,
        Box::new(|_cc| Ok(Box::new(SceneEditorApp::default()))),
    )
}