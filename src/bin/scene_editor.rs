use eframe::{self, egui};
use rfd::FileDialog;
use serde_json;
use std::fs;
use std::path::PathBuf;

// Import the SceneData and related structs from the main project
use rt_2::core::color::Color;
use rt_2::core::vec3::{Point3, Vec3};
use rt_2::scene::storage::{
    CameraData, CubeData, CylinderData, ObjectData, PlaneData, SceneData, SphereData, TextureData,
};

fn point3_editor(ui: &mut egui::Ui, label: &str, point: &mut Point3, scene_changed: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label("X:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut point.x).speed(0.1))
            .changed();
        ui.label("Y:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut point.y).speed(0.1))
            .changed();
        ui.label("Z:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut point.z).speed(0.1))
            .changed();
    });
}

fn vec3_editor(ui: &mut egui::Ui, label: &str, vec: &mut Vec3, scene_changed: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label("X:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut vec.x).speed(0.1))
            .changed();
        ui.label("Y:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut vec.y).speed(0.1))
            .changed();
        ui.label("Z:");
        *scene_changed |= ui
            .add(egui::DragValue::new(&mut vec.z).speed(0.1))
            .changed();
    });
}

fn color_editor(ui: &mut egui::Ui, label: &str, color: &mut Color, scene_changed: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label("R:");
        *scene_changed |= ui
            .add(
                egui::DragValue::new(&mut color.r)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
        ui.label("G:");
        *scene_changed |= ui
            .add(
                egui::DragValue::new(&mut color.g)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
        ui.label("B:");
        *scene_changed |= ui
            .add(
                egui::DragValue::new(&mut color.b)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
    });
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ViewType {
    TopDown,
    Front,
    Side,
    ThreeD,
}

struct SceneEditorApp {
    scene_data: SceneData,
    json_string: String,
    error_message: Option<String>,
    current_view: ViewType,
    current_file_path: Option<PathBuf>,
    image_previews: std::collections::HashMap<String, egui::TextureHandle>,
}

impl SceneEditorApp {
    fn update_json_string(&mut self) {
        match serde_json::to_string_pretty(&self.scene_data) {
            Ok(json) => {
                self.json_string = json;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to serialize scene data: {}", e));
            }
        }
    }

    fn draw_scene_2d(&self, ui: &mut egui::Ui, view_type: ViewType) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.rect_filled(rect, 0.0, egui::Color32::DARK_GRAY);

        let scene_scale = 20.0; // Pixels per unit in scene

        let to_screen_pos = |p: Point3| {
            let (screen_x, screen_y) = match view_type {
                ViewType::TopDown => (
                    rect.center().x + p.x * scene_scale,
                    rect.center().y + p.z * scene_scale, // Z maps to screen Y
                ),
                ViewType::Front => (
                    rect.center().x + p.x * scene_scale,
                    rect.center().y - p.y * scene_scale, // Y maps to screen Y (inverted for typical screen coords)
                ),
                ViewType::Side => (
                    rect.center().x + p.z * scene_scale, // Z maps to screen X
                    rect.center().y - p.y * scene_scale, // Y maps to screen Y (inverted)
                ),
                _ => (0.0, 0.0), // Should not happen
            };
            egui::pos2(screen_x, screen_y)
        };

        let axis_color = egui::Color32::from_gray(128);
        let text_color = egui::Color32::from_gray(160);
        let tick_length = 5.0;

        let (x_axis_label, y_axis_label) = match view_type {
            ViewType::TopDown => ("X", "Z"),
            ViewType::Front => ("X", "Y"),
            ViewType::Side => ("Z", "Y"),
            _ => ("", ""),
        };

        // Draw X axis
        painter.line_segment(
            [
                egui::pos2(rect.left(), rect.center().y),
                egui::pos2(rect.right(), rect.center().y),
            ],
            egui::Stroke::new(1.0, axis_color),
        );

        // Draw Y axis
        painter.line_segment(
            [
                egui::pos2(rect.center().x, rect.top()),
                egui::pos2(rect.center().x, rect.bottom()),
            ],
            egui::Stroke::new(1.0, axis_color),
        );

        let x_min = (-(rect.width() / 2.0) / scene_scale).floor() as i32;
        let x_max = ((rect.width() / 2.0) / scene_scale).ceil() as i32;
        let y_min = (-(rect.height() / 2.0) / scene_scale).floor() as i32;
        let y_max = ((rect.height() / 2.0) / scene_scale).ceil() as i32;

        for i in x_min..=x_max {
            let x = i as f32 * scene_scale;
            painter.line_segment(
                [
                    egui::pos2(rect.center().x + x, rect.center().y - tick_length),
                    egui::pos2(rect.center().x + x, rect.center().y + tick_length),
                ],
                egui::Stroke::new(1.0, axis_color),
            );

            if i % 5 == 0 {
                painter.text(
                    egui::pos2(rect.center().x + x, rect.center().y + tick_length * 2.0),
                    egui::Align2::CENTER_TOP,
                    i.to_string(),
                    egui::FontId::default(),
                    text_color,
                );
            }
        }

        for i in y_min..=y_max {
            let y = i as f32 * scene_scale;
            painter.line_segment(
                [
                    egui::pos2(rect.center().x - tick_length, rect.center().y + y),
                    egui::pos2(rect.center().x + tick_length, rect.center().y + y),
                ],
                egui::Stroke::new(1.0, axis_color),
            );

            if i % 5 == 0 && i != 0 {
                let label = if view_type == ViewType::TopDown {
                    i.to_string()
                } else {
                    (-i).to_string()
                };
                painter.text(
                    egui::pos2(rect.center().x - tick_length * 2.0, rect.center().y + y),
                    egui::Align2::RIGHT_CENTER,
                    label,
                    egui::FontId::default(),
                    text_color,
                );
            }
        }

        painter.text(
            egui::pos2(rect.right() - 10.0, rect.center().y - 10.0),
            egui::Align2::RIGHT_BOTTOM,
            x_axis_label,
            egui::FontId::default(),
            text_color,
        );

        painter.text(
            egui::pos2(rect.center().x + 10.0, rect.top() + 10.0),
            egui::Align2::LEFT_TOP,
            y_axis_label,
            egui::FontId::default(),
            text_color,
        );

        // Draw Camera
        let cam_pos_2d = to_screen_pos(self.scene_data.camera.position);
        painter.circle_filled(cam_pos_2d, 5.0, egui::Color32::WHITE);
        let look_at_pos_2d = to_screen_pos(self.scene_data.camera.look_at);
        painter.line_segment(
            [cam_pos_2d, look_at_pos_2d],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );

        // Draw Objects
        for object in &self.scene_data.objects {
            match object {
                ObjectData::Sphere(sphere) => {
                    let center_2d = to_screen_pos(sphere.center);
                    let radius_pixels = sphere.radius * scene_scale;
                    painter.circle_stroke(
                        center_2d,
                        radius_pixels,
                        egui::Stroke::new(1.0, egui::Color32::BLUE),
                    );
                }
                ObjectData::Plane(plane) => {
                    let center_2d = to_screen_pos(plane.center);
                    let (half_width, half_height) = match view_type {
                        ViewType::TopDown => (
                            plane.size.x * scene_scale / 2.0,
                            plane.size.z * scene_scale / 2.0,
                        ),
                        ViewType::Front => (
                            plane.size.x * scene_scale / 2.0,
                            plane.size.y * scene_scale / 2.0,
                        ),
                        ViewType::Side => (
                            plane.size.z * scene_scale / 2.0,
                            plane.size.y * scene_scale / 2.0,
                        ),
                        _ => (0.0, 0.0),
                    };

                    let rect_min = egui::pos2(center_2d.x - half_width, center_2d.y - half_height);
                    let rect_max = egui::pos2(center_2d.x + half_width, center_2d.y + half_height);
                    painter.rect_stroke(
                        egui::Rect::from_min_max(rect_min, rect_max),
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::GREEN),
                    );
                }
                ObjectData::Cube(cube) => {
                    let center_2d = to_screen_pos(cube.center);
                    let half_size = cube.size * scene_scale / 2.0;
                    let rect_min = egui::pos2(center_2d.x - half_size, center_2d.y - half_size);
                    let rect_max = egui::pos2(center_2d.x + half_size, center_2d.y + half_size);
                    painter.rect_stroke(
                        egui::Rect::from_min_max(rect_min, rect_max),
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::RED),
                    );
                }
                ObjectData::Cylinder(cylinder) => match view_type {
                    ViewType::TopDown => {
                        let center_2d = to_screen_pos(cylinder.center);
                        let radius_pixels = cylinder.radius * scene_scale;
                        painter.circle_stroke(
                            center_2d,
                            radius_pixels,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                        );
                    }
                    ViewType::Front => {
                        let half_height = cylinder.height / 2.0;
                        let radius = cylinder.radius;
                        let center = cylinder.center;

                        let top_left = to_screen_pos(Point3::new(
                            center.x - radius,
                            center.y + half_height,
                            center.z,
                        ));
                        let bottom_right = to_screen_pos(Point3::new(
                            center.x + radius,
                            center.y - half_height,
                            center.z,
                        ));

                        let rect = egui::Rect::from_min_max(top_left, bottom_right);
                        painter.rect_stroke(
                            rect,
                            0.0,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                        );
                    }
                    ViewType::Side => {
                        let half_height = cylinder.height / 2.0;
                        let radius = cylinder.radius;
                        let center = cylinder.center;

                        let top_left = to_screen_pos(Point3::new(
                            center.x,
                            center.y + half_height,
                            center.z - radius,
                        ));
                        let bottom_right = to_screen_pos(Point3::new(
                            center.x,
                            center.y - half_height,
                            center.z + radius,
                        ));

                        let rect = egui::Rect::from_min_max(top_left, bottom_right);
                        painter.rect_stroke(
                            rect,
                            0.0,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                        );
                    }
                    _ => {}
                },
            }
        }
    }

    fn get_cube_vertices(center: Point3, size: f32) -> [Point3; 8] {
        let half_size = size / 2.0;
        [
            Point3::new(
                center.x - half_size,
                center.y - half_size,
                center.z - half_size,
            ),
            Point3::new(
                center.x + half_size,
                center.y - half_size,
                center.z - half_size,
            ),
            Point3::new(
                center.x + half_size,
                center.y + half_size,
                center.z - half_size,
            ),
            Point3::new(
                center.x - half_size,
                center.y + half_size,
                center.z - half_size,
            ),
            Point3::new(
                center.x - half_size,
                center.y - half_size,
                center.z + half_size,
            ),
            Point3::new(
                center.x + half_size,
                center.y - half_size,
                center.z + half_size,
            ),
            Point3::new(
                center.x + half_size,
                center.y + half_size,
                center.z + half_size,
            ),
            Point3::new(
                center.x - half_size,
                center.y + half_size,
                center.z + half_size,
            ),
        ]
    }

    fn get_cylinder_vertices(
        center: Point3,
        radius: f32,
        height: f32,
        segments: usize,
    ) -> (Vec<Point3>, Vec<Point3>) {
        let mut top_vertices = Vec::new();
        let mut bottom_vertices = Vec::new();
        let half_height = height / 2.0;

        for i in 0..=segments {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let x = center.x + radius * angle.cos();
            let z = center.z + radius * angle.sin();
            top_vertices.push(Point3::new(x, center.y + half_height, z));
            bottom_vertices.push(Point3::new(x, center.y - half_height, z));
        }

        (top_vertices, bottom_vertices)
    }

    fn get_plane_vertices(center: Point3, size: Vec3) -> [Point3; 4] {
        let half_size_x = size.x / 2.0;
        let half_size_z = size.z / 2.0;
        [
            Point3::new(center.x - half_size_x, center.y, center.z - half_size_z),
            Point3::new(center.x + half_size_x, center.y, center.z - half_size_z),
            Point3::new(center.x + half_size_x, center.y, center.z + half_size_z),
            Point3::new(center.x - half_size_x, center.y, center.z + half_size_z),
        ]
    }

    fn draw_scene_3d(&mut self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.rect_filled(rect, 0.0, egui::Color32::DARK_GRAY);

        let camera = &self.scene_data.camera;
        let fov_rad = camera.fov.to_radians();
        let aspect_ratio = camera.aspect_ratio;

        let view_matrix =
            glam::Mat4::look_at_rh(camera.position.into(), camera.look_at.into(), camera.up.into());
        let projection_matrix = glam::Mat4::perspective_rh(fov_rad, aspect_ratio, 0.1, 100.0);
        let transform = projection_matrix * view_matrix;

        let to_screen_pos = |p: Point3| {
            let p_world = glam::vec4(p.x, p.y, p.z, 1.0);
            let p_clip = transform * p_world;

            if p_clip.w <= 0.0 {
                return egui::pos2(-1000.0, -1000.0); // Off-screen
            }

            let p_ndc = p_clip / p_clip.w;

            let screen_x = rect.center().x + p_ndc.x * rect.width() / 2.0;
            let screen_y = rect.center().y - p_ndc.y * rect.height() / 2.0;

            egui::pos2(screen_x, screen_y)
        };

        // --- Pass 1: Draw Planes ---
        for object in &self.scene_data.objects {
            if let ObjectData::Plane(plane) = object {
                let stroke = egui::Stroke::new(1.0, egui::Color32::GREEN);
                let half_size_x = plane.size.x / 2.0;
                let half_size_z = plane.size.z / 2.0;
                let num_lines = 10;

                // Lines along Z
                for i in 0..=num_lines {
                    let p = i as f32 / num_lines as f32; // 0.0 to 1.0
                    let x = plane.center.x - half_size_x + p * plane.size.x;
                    let start = Point3::new(x, plane.center.y, plane.center.z - half_size_z);
                    let end = Point3::new(x, plane.center.y, plane.center.z + half_size_z);
                    painter.line_segment([to_screen_pos(start), to_screen_pos(end)], stroke);
                }

                // Lines along X
                for i in 0..=num_lines {
                    let p = i as f32 / num_lines as f32; // 0.0 to 1.0
                    let z = plane.center.z - half_size_z + p * plane.size.z;
                    let start = Point3::new(plane.center.x - half_size_x, plane.center.y, z);
                    let end = Point3::new(plane.center.x + half_size_x, plane.center.y, z);
                    painter.line_segment([to_screen_pos(start), to_screen_pos(end)], stroke);
                }
            }
        }

        // --- Pass 2: Draw other objects ---
        for object in &self.scene_data.objects {
            match object {
                ObjectData::Plane(_) => continue, // Skip planes, already drawn
                ObjectData::Sphere(sphere) => {
                    let num_segments = 12;
                    let stroke = egui::Stroke::new(1.0, egui::Color32::BLUE);

                    // Draw meridians (vertical loops)
                    for i in 0..num_segments {
                        let angle = i as f32 * std::f32::consts::PI * 2.0 / num_segments as f32;
                        let mut points = Vec::new();
                        for j in 0..=num_segments {
                            let sub_angle = j as f32 * std::f32::consts::PI / num_segments as f32;
                            let x =
                                sphere.center.x + sphere.radius * sub_angle.sin() * angle.cos();
                            let y = sphere.center.y + sphere.radius * sub_angle.cos();
                            let z =
                                sphere.center.z + sphere.radius * sub_angle.sin() * angle.sin();
                            points.push(to_screen_pos(Point3::new(x, y, z)));
                        }
                        painter.add(egui::Shape::line(points, stroke));
                    }

                    // Draw parallels (horizontal loops)
                    for i in 1..num_segments {
                        let sub_angle = i as f32 * std::f32::consts::PI / num_segments as f32;
                        let mut points = Vec::new();
                        for j in 0..=num_segments {
                            let angle =
                                j as f32 * std::f32::consts::PI * 2.0 / num_segments as f32;
                            let x =
                                sphere.center.x + sphere.radius * sub_angle.sin() * angle.cos();
                            let y = sphere.center.y + sphere.radius * sub_angle.cos();
                            let z =
                                sphere.center.z + sphere.radius * sub_angle.sin() * angle.sin();
                            points.push(to_screen_pos(Point3::new(x, y, z)));
                        }
                        painter.add(egui::Shape::line(points, stroke));
                    }
                }
                ObjectData::Cube(cube) => {
                    let vertices = Self::get_cube_vertices(cube.center, cube.size);
                    let mut projected_vertices = [egui::pos2(0.0, 0.0); 8];
                    for i in 0..8 {
                        projected_vertices[i] = to_screen_pos(vertices[i]);
                    }

                    let edges = [
                        (0, 1), (1, 2), (2, 3), (3, 0), // Back face
                        (4, 5), (5, 6), (6, 7), (7, 4), // Front face
                        (0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
                    ];

                    for (i, j) in &edges {
                        painter.line_segment(
                            [projected_vertices[*i], projected_vertices[*j]],
                            egui::Stroke::new(1.0, egui::Color32::RED),
                        );
                    }
                }
                ObjectData::Cylinder(cylinder) => {
                    let num_segments = 12;
                    let (top_vertices, bottom_vertices) = Self::get_cylinder_vertices(
                        cylinder.center,
                        cylinder.radius,
                        cylinder.height,
                        num_segments,
                    );

                    let mut projected_top = Vec::new();
                    for v in top_vertices {
                        projected_top.push(to_screen_pos(v));
                    }

                    let mut projected_bottom = Vec::new();
                    for v in bottom_vertices {
                        projected_bottom.push(to_screen_pos(v));
                    }

                    painter.add(egui::Shape::line(
                        projected_top.clone(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                    ));
                    painter.add(egui::Shape::line(
                        projected_bottom.clone(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                    ));

                    for i in 0..num_segments {
                        painter.line_segment(
                            [projected_top[i], projected_bottom[i]],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                        );
                    }
                }
            }
        }

        // --- Draw Axis Gizmo ---
        let gizmo_origin = egui::pos2(rect.left() + 60.0, rect.bottom() - 60.0);
        let gizmo_length = 50.0;

        // Transform world axes directions into view space
        let x_axis_view = view_matrix.mul_vec4(glam::vec4(1.0, 0.0, 0.0, 0.0));
        let y_axis_view = view_matrix.mul_vec4(glam::vec4(0.0, 1.0, 0.0, 0.0));
        let z_axis_view = view_matrix.mul_vec4(glam::vec4(0.0, 0.0, 1.0, 0.0));

        // Use the x and y components for screen direction (negate y for egui coords)
        let x_screen_dir = egui::vec2(x_axis_view.x, -x_axis_view.y).normalized() * gizmo_length;
        let y_screen_dir = egui::vec2(y_axis_view.x, -y_axis_view.y).normalized() * gizmo_length;
        let z_screen_dir = egui::vec2(z_axis_view.x, -z_axis_view.y).normalized() * gizmo_length;

        // Draw lines and labels
        painter.line_segment(
            [gizmo_origin, gizmo_origin + x_screen_dir],
            egui::Stroke::new(2.0, egui::Color32::RED),
        );
        painter.text(
            gizmo_origin + x_screen_dir * 1.1,
            egui::Align2::CENTER_CENTER,
            "X",
            egui::FontId::default(),
            egui::Color32::RED,
        );

        painter.line_segment(
            [gizmo_origin, gizmo_origin + y_screen_dir],
            egui::Stroke::new(2.0, egui::Color32::GREEN),
        );
        painter.text(
            gizmo_origin + y_screen_dir * 1.1,
            egui::Align2::CENTER_CENTER,
            "Y",
            egui::FontId::default(),
            egui::Color32::GREEN,
        );

        painter.line_segment(
            [gizmo_origin, gizmo_origin + z_screen_dir],
            egui::Stroke::new(2.0, egui::Color32::BLUE),
        );
        painter.text(
            gizmo_origin + z_screen_dir * 1.1,
            egui::Align2::CENTER_CENTER,
            "Z",
            egui::FontId::default(),
            egui::Color32::BLUE,
        );
    }
}
fn material_editor(
    ui: &mut egui::Ui,
    material: &mut rt_2::scene::storage::MaterialData,
    scene_changed: &mut bool,
) {
    ui.group(|ui| {
        ui.label("Material Properties");
        ui.horizontal(|ui| {
            ui.label("Diffuse:");
            *scene_changed |= ui
                .add(
                    egui::DragValue::new(&mut material.diffuse)
                        .speed(0.01)
                        .range(0.0..=1.0),
                )
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Reflectivity:");
            *scene_changed |= ui
                .add(
                    egui::DragValue::new(&mut material.reflectivity)
                        .speed(0.01)
                        .range(0.0..=1.0),
                )
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Transparency:");
            *scene_changed |= ui
                .add(
                    egui::DragValue::new(&mut material.transparency)
                        .speed(0.01)
                        .range(0.0..=1.0),
                )
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Index of Refraction:");
            *scene_changed |= ui
                .add(
                    egui::DragValue::new(&mut material.index_of_refraction)
                        .speed(0.01)
                        .range(0.0..=3.0),
                )
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Emission:");
            let mut emission_enabled = material.emission.is_some();
            if ui
                .checkbox(&mut emission_enabled, "Enable Emission")
                .changed()
            {
                if emission_enabled {
                    material.emission = Some(Color::WHITE);
                } else {
                    material.emission = None;
                }
                *scene_changed = true;
            }
        });
        if let Some(emission_color) = &mut material.emission {
            let mut intensity = emission_color.r.max(emission_color.g).max(emission_color.b);
            if intensity == 0.0 {
                intensity = 1.0;
            }
            let mut normalized_color = Color::new(
                emission_color.r / intensity,
                emission_color.g / intensity,
                emission_color.b / intensity,
            );

            let initial_intensity = intensity;
            let initial_normalized_color = normalized_color;

            ui.horizontal(|ui| {
                ui.label("Intensity:");
                *scene_changed |= ui
                    .add(egui::DragValue::new(&mut intensity).speed(0.1))
                    .changed();
            });
            color_editor(ui, "Color:", &mut normalized_color, scene_changed);

            if intensity != initial_intensity || normalized_color != initial_normalized_color {
                *emission_color = normalized_color * intensity;
                *scene_changed = true;
            }
        }
    });
}

fn texture_editor(
    ui: &mut egui::Ui,
    texture: &mut TextureData,
    scene_changed: &mut bool,
    ctx: &egui::Context,
    image_previews: &mut std::collections::HashMap<String, egui::TextureHandle>,
) {
    let mut current_texture_type = match texture {
        TextureData::SolidColor(_) => "SolidColor",
        TextureData::Gradient(_, _, _) => "Gradient",
        TextureData::Checkerboard(_, _, _) => "Checkerboard",
        TextureData::Image(_) => "Image",
    };

    ui.horizontal(|ui| {
        if ui
            .radio_value(&mut current_texture_type, "SolidColor", "Solid Color")
            .changed()
        {
            *texture = TextureData::SolidColor(Color::WHITE);
            *scene_changed = true;
        }
        if ui
            .radio_value(&mut current_texture_type, "Gradient", "Gradient")
            .changed()
        {
            *texture = TextureData::Gradient(Color::WHITE, Color::BLACK, 0.0);
            *scene_changed = true;
        }
        if ui
            .radio_value(&mut current_texture_type, "Checkerboard", "Checkerboard")
            .changed()
        {
            *texture = TextureData::Checkerboard(Color::WHITE, Color::BLACK, 1.0);
            *scene_changed = true;
        }
        if ui
            .radio_value(&mut current_texture_type, "Image", "Image")
            .changed()
        {
            *texture = TextureData::Image(String::new());
            *scene_changed = true;
        }
    });

    ui.indent("texture_indent", |ui| match texture {
        TextureData::SolidColor(color) => {
            color_editor(ui, "Color:", color, scene_changed);
        }
        TextureData::Gradient(color1, color2, angle) => {
            color_editor(ui, "Color 1:", color1, scene_changed);
            color_editor(ui, "Color 2:", color2, scene_changed);
            ui.label("Angle:");
            *scene_changed |= ui
                .add(egui::DragValue::new(&mut *angle).speed(0.1))
                .changed();
        }
        TextureData::Checkerboard(color1, color2, frequency) => {
            color_editor(ui, "Color 1:", color1, scene_changed);
            color_editor(ui, "Color 2:", color2, scene_changed);
            ui.label("Frequency:");
            *scene_changed |= ui
                .add(egui::DragValue::new(&mut *frequency).speed(0.1))
                .changed();
        }
        TextureData::Image(path) => {
            ui.label("Path:");
            ui.horizontal(|ui| {
                ui.add_sized([200.0, 20.0], egui::TextEdit::singleline(path)).changed();
                if ui.button("Browse...").clicked() {
                    if let Some(new_path) = FileDialog::new()
                        .add_filter("Image Files", &["png", "jpg", "jpeg", "gif", "bmp"])
                        .pick_file()
                    {
                        *path = new_path.to_string_lossy().to_string();
                        *scene_changed = true;
                    }
                }
            });

            if !path.is_empty() {
                if !image_previews.contains_key(path) {
                    if let Ok(image) = rt_2::pixels::image::Image::load(path) {
                        let color_image = egui::ColorImage::from_rgb(
                            [image.width, image.height],
                            &image
                                .pixels
                                .iter()
                                .flat_map(|c| {
                                    vec![
                                        (c.r * 255.0) as u8,
                                        (c.g * 255.0) as u8,
                                        (c.b * 255.0) as u8,
                                    ]
                                })
                                .collect::<Vec<u8>>(),
                        );
                        let handle =
                            ctx.load_texture(path.clone(), color_image, Default::default());
                        image_previews.insert(path.clone(), handle);
                    }
                }

                if let Some(texture_handle) = image_previews.get(path) {
                    ui.image((texture_handle.id(), texture_handle.size_vec2() / 2.0));
                }
            }
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
                camera: default_camera,
            },
            json_string: String::new(),
            error_message: None,
            current_view: ViewType::TopDown,
            current_file_path: None,
            image_previews: std::collections::HashMap::new(),
        };
        app.update_json_string(); // Initialize json_string with default scene_data
        app
    }
}

impl eframe::App for SceneEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut scene_changed = false;

        egui::SidePanel::left("scene_editor_panel")
            .min_width(250.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Scene Editor");

                    if let Some(path) = &self.current_file_path {
                        ui.label(format!("File: {}", path.display()));
                    } else {
                        ui.label("File: New Scene");
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Load Scene").clicked() {
                            if let Some(path) =
                                FileDialog::new().add_filter("JSON", &["json"]).pick_file()
                            {
                                self.current_file_path = Some(path.clone());
                                match fs::read_to_string(path) {
                                    Ok(data) => {
                                        self.json_string = data.clone();
                                        match serde_json::from_str::<SceneData>(&data) {
                                            Ok(scene) => {
                                                self.scene_data = scene;
                                                self.error_message = None;
                                                scene_changed = true; // Indicate change after loading
                                            }
                                            Err(e) => {
                                                self.error_message = Some(format!(
                                                    "Failed to parse scene file: {}",
                                                    e
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.error_message =
                                            Some(format!("Failed to read scene file: {}", e));
                                    }
                                }
                            }
                        }

                        if ui.button("Save").clicked() {
                            let path_to_save = self.current_file_path.clone();
                            if let Some(path) = path_to_save {
                                match serde_json::to_string_pretty(&self.scene_data) {
                                    Ok(json) => match fs::write(path, json) {
                                        Ok(_) => self.error_message = None,
                                        Err(e) => {
                                            self.error_message =
                                                Some(format!("Failed to write scene file: {}", e))
                                        }
                                    },
                                    Err(e) => {
                                        self.error_message =
                                            Some(format!("Failed to serialize scene data: {}", e))
                                    }
                                }
                            } else {
                                // If no path, fallback to Save As
                                if let Some(path) =
                                    FileDialog::new().add_filter("JSON", &["json"]).save_file()
                                {
                                    self.current_file_path = Some(path.clone());
                                    match serde_json::to_string_pretty(&self.scene_data) {
                                        Ok(json) => match fs::write(path, json) {
                                            Ok(_) => self.error_message = None,
                                            Err(e) => {
                                                self.error_message = Some(format!(
                                                    "Failed to write scene file: {}",
                                                    e
                                                ))
                                            }
                                        },
                                        Err(e) => {
                                            self.error_message = Some(format!(
                                                "Failed to serialize scene data: {}",
                                                e
                                            ))
                                        }
                                    }
                                }
                            }
                        }

                        if self.current_file_path.is_some() {
                            if ui.button("Save As").clicked() {
                                if let Some(path) =
                                    FileDialog::new().add_filter("JSON", &["json"]).save_file()
                                {
                                    self.current_file_path = Some(path.clone());
                                    match serde_json::to_string_pretty(&self.scene_data) {
                                        Ok(json) => match fs::write(path, json) {
                                            Ok(_) => {
                                                self.error_message = None;
                                            }
                                            Err(e) => {
                                                self.error_message = Some(format!(
                                                    "Failed to write scene file: {}",
                                                    e
                                                ));
                                            }
                                        },
                                        Err(e) => {
                                            self.error_message = Some(format!(
                                                "Failed to serialize scene data: {}",
                                                e
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    });

                    if let Some(msg) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, msg);
                    }

                    ui.separator();

                    // Camera Editor
                    ui.collapsing("Camera", |ui| {
                        point3_editor(
                            ui,
                            "Position:",
                            &mut self.scene_data.camera.position,
                            &mut scene_changed,
                        );
                        point3_editor(
                            ui,
                            "Look At:",
                            &mut self.scene_data.camera.look_at,
                            &mut scene_changed,
                        );
                        vec3_editor(
                            ui,
                            "Up Vector:",
                            &mut self.scene_data.camera.up,
                            &mut scene_changed,
                        );
                        ui.horizontal(|ui| {
                            ui.label("FOV:");
                            scene_changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.scene_data.camera.fov)
                                        .speed(1.0),
                                )
                                .changed();

                            ui.label("Aspect Ratio:");
                            scene_changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.scene_data.camera.aspect_ratio)
                                        .speed(0.01),
                                )
                                .changed();
                        });

                        ui.horizontal(|ui| {
                            ui.label("Resolution:");
                            scene_changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.scene_data.camera.resolution.0)
                                        .speed(1.0),
                                )
                                .changed();
                            scene_changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.scene_data.camera.resolution.1)
                                        .speed(1.0),
                                )
                                .changed();
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
                                        if ui
                                            .radio_value(
                                                &mut current_object_type,
                                                "Sphere",
                                                "Sphere",
                                            )
                                            .changed()
                                        {
                                            *object = ObjectData::Sphere(SphereData::default());
                                            scene_changed = true;
                                        }
                                        if ui
                                            .radio_value(&mut current_object_type, "Plane", "Plane")
                                            .changed()
                                        {
                                            *object = ObjectData::Plane(PlaneData::default());
                                            scene_changed = true;
                                        }
                                        if ui
                                            .radio_value(&mut current_object_type, "Cube", "Cube")
                                            .changed()
                                        {
                                            *object = ObjectData::Cube(CubeData::default());
                                            scene_changed = true;
                                        }
                                        if ui
                                            .radio_value(
                                                &mut current_object_type,
                                                "Cylinder",
                                                "Cylinder",
                                            )
                                            .changed()
                                        {
                                            *object = ObjectData::Cylinder(CylinderData::default());
                                            scene_changed = true;
                                        }
                                    });

                                    match object {
                                        ObjectData::Sphere(sphere) => {
                                            point3_editor(
                                                ui,
                                                "Center:",
                                                &mut sphere.center,
                                                &mut scene_changed,
                                            );
                                            ui.horizontal(|ui| {
                                                ui.label("Radius:");
                                                scene_changed |= ui
                                                    .add(
                                                        egui::DragValue::new(&mut sphere.radius)
                                                            .speed(0.1),
                                                    )
                                                    .changed();
                                            });
                                            ui.group(|ui| {
                                                ui.label("Texture:");
                                                texture_editor(
                                                    ui,
                                                    &mut sphere.material.texture,
                                                    &mut scene_changed,
                                                    ctx,
                                                    &mut self.image_previews,
                                                );
                                            });
                                            material_editor(
                                                ui,
                                                &mut sphere.material,
                                                &mut scene_changed,
                                            );
                                        }
                                        ObjectData::Plane(plane) => {
                                            point3_editor(
                                                ui,
                                                "Center:",
                                                &mut plane.center,
                                                &mut scene_changed,
                                            );
                                            vec3_editor(
                                                ui,
                                                "Size:",
                                                &mut plane.size,
                                                &mut scene_changed,
                                            );
                                            ui.group(|ui| {
                                                ui.label("Texture:");
                                                texture_editor(
                                                    ui,
                                                    &mut plane.material.texture,
                                                    &mut scene_changed,
                                                    ctx,
                                                    &mut self.image_previews,
                                                );
                                            });
                                            material_editor(
                                                ui,
                                                &mut plane.material,
                                                &mut scene_changed,
                                            );
                                        }
                                        ObjectData::Cube(cube) => {
                                            point3_editor(
                                                ui,
                                                "Center:",
                                                &mut cube.center,
                                                &mut scene_changed,
                                            );
                                            ui.horizontal(|ui| {
                                                ui.label("Size:");
                                                scene_changed |= ui
                                                    .add(
                                                        egui::DragValue::new(&mut cube.size)
                                                            .speed(0.1),
                                                    )
                                                    .changed();
                                            });
                                            ui.group(|ui| {
                                                ui.label("Texture:");
                                                texture_editor(
                                                    ui,
                                                    &mut cube.material.texture,
                                                    &mut scene_changed,
                                                    ctx,
                                                    &mut self.image_previews,
                                                );
                                            });
                                            material_editor(
                                                ui,
                                                &mut cube.material,
                                                &mut scene_changed,
                                            );
                                        }
                                        ObjectData::Cylinder(cylinder) => {
                                            point3_editor(
                                                ui,
                                                "Center:",
                                                &mut cylinder.center,
                                                &mut scene_changed,
                                            );
                                            ui.horizontal(|ui| {
                                                ui.label("Radius:");
                                                scene_changed |= ui
                                                    .add(
                                                        egui::DragValue::new(&mut cylinder.radius)
                                                            .speed(0.1),
                                                    )
                                                    .changed();
                                                ui.label("Height:");
                                                scene_changed |= ui
                                                    .add(
                                                        egui::DragValue::new(&mut cylinder.height)
                                                            .speed(0.1),
                                                    )
                                                    .changed();
                                            });
                                            ui.group(|ui| {
                                                ui.label("Texture:");
                                                texture_editor(
                                                    ui,
                                                    &mut cylinder.material.texture,
                                                    &mut scene_changed,
                                                    ctx,
                                                    &mut self.image_previews,
                                                );
                                            });
                                            material_editor(
                                                ui,
                                                &mut cylinder.material,
                                                &mut scene_changed,
                                            );
                                        }
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
                                self.scene_data
                                    .objects
                                    .push(ObjectData::Sphere(SphereData::default()));
                                scene_changed = true;
                            }
                            if ui.button("Add Plane").clicked() {
                                self.scene_data
                                    .objects
                                    .push(ObjectData::Plane(PlaneData::default()));
                                scene_changed = true;
                            }
                            if ui.button("Add Cube").clicked() {
                                self.scene_data
                                    .objects
                                    .push(ObjectData::Cube(CubeData::default()));
                                scene_changed = true;
                            }
                            if ui.button("Add Cylinder").clicked() {
                                self.scene_data
                                    .objects
                                    .push(ObjectData::Cylinder(CylinderData::default()));
                                scene_changed = true;
                            }
                        });
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                ViewType::ThreeD => self.draw_scene_3d(ui),
                _ => self.draw_scene_2d(ui, self.current_view),
            }
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_view, ViewType::TopDown, "Top-Down (X-Z)");
                ui.selectable_value(&mut self.current_view, ViewType::Front, "Front (X-Y)");
                ui.selectable_value(&mut self.current_view, ViewType::Side, "Side (Y-Z)");
                ui.selectable_value(&mut self.current_view, ViewType::ThreeD, "3D");
            });
        });

        if scene_changed {
            self.update_json_string();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Scene Editor",
        options,
        Box::new(|_cc| Ok(Box::new(SceneEditorApp::default()))),
    )
}
