# Scene Elements
The scene consists of:
- [Camera](#camera)
- [Background](#background)
- [Objects](#objects)
- [Materials](#materials)
- [Particle Systems](#particle-systems)

Read about the [**basic types**](README_basic_types.md) (Vec3, Point3, Color, Texture) that you will be need to define for the scene elements.

## **Camera**
The camera defines the viewer's perspective—essentially acting as a window into the 3D scene.

To update camera settings (poisition, direction, orientation, fov), you can use `Scene.camera_mut().set`
```rust
    scene.camera_mut().set(
        origin: Vec3            // Camera position
        look_at: Vec3,          // Look at position
        vup: Vec3,              // controls camera orientation
        vfov: f32,              // field of vision
        focal_length: f32,      // focal length
        resolution: (u32, u32)  // resolution
    );
```

Example:
```rust
    // we modify the default scene camera
    scene.camera_mut().set(
        Vec3::new(-3.0, 3.0, -3.0), // put the camera on the opposite side
        Vec3::ZERO,
        -Point3::Y,                 // orientate the camera downwards
        60.0,
        1.0,
        (400, 300)
    );
```
With this, we create something like a mirror image of the default image.

![Rendered output](/demo_images/demo2.png)

## **Background**
You can set the bckground with the different `Texture`types.
```rust
    scene.set_background(Texture::SolidColor(Color::BLACK));
```
The default background is just black. You can use any of the `Texture`type for the background.
```rust
    scene.set_background(
        Texture::Gradient(
            Color::BLUE,
            Color::CYAN,
            std::f32::consts::FRAC_PI_2));
```
![Rendered output](/demo_images/demo7.png)

## **Objects**
The program supports four basic object types:
- 2D plane
- Cube
- Cylinder
- Sphere

### Adding objects
To add an object to the scene, you can use:
```rust
    scene.add_object(Object)
```
Example:
```rust
    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
    ));
```

### 2D plane
A flat surface aligned with the X and Z axes.
```rust
    Plane::new(
        center: Point3,   // Center position of the plane
        size: Vec3,       // Size vector defining width (X), height (Y), and depth (Z)
        texture: Texture, // Texture applied to the plane surface
    );
```
![Rendered output](/demo_images/demo3.png)

### Cube
A six-sided box defined by its center and uniform side length.
```rust
    Cube::new(
        center: Point3,   // Center position of the cube
        size: f32,        // Length of each edge (all sides are equal)
        texture: Texture, // Texture applied to the cube surfaces
    );
```
![Rendered output](/demo_images/demo4.png)

### Cylinder
A vertical cylinder defined by its base center, radius, and height.
```rust
    Cylinder::new(
        base_center: Point3, // Center of the bottom circular face
        radius: f32,         // Radius of the cylinder's circular base
        height: f32,         // Height of the cylinder
        texture: Texture,    // Texture applied to the cylinder surface
    );
```
![Rendered output](/demo_images/demo5.png)

### Sphere
A perfectly round 3D object defined by its center and radius.
```rust
    Sphere::new(
        center: Point3,   // Center position of the sphere
        radius: f32,      // Radius of the sphere (distance from center to surface)
        texture: Texture, // Texture applied to the sphere surface
    );
```
![Rendered output](/demo_images/demo6.png)

## **Materials**

Materials define how objects interact with light in your ray tracer. Each material has five key properties that control appearance and behavior:

```rust
pub struct Material {
    pub texture: Texture,         // Base color or texture pattern
    pub diffuse: f32,             // Matte surface scattering (0.0 - 1.0)
    pub reflectivity: f32,        // Mirror-like reflections (0.0 - 1.0)
    pub transparency: f32,        // Light transmission (0.0 - 1.0)
    pub index_of_refraction: f32, // How much light bends when passing through
    pub emission: Option<Color>,  // Light emission (None or Some(Color))
}
```

### **Diffuse Property**
Controls how much light scatters randomly from the surface (matte appearance).

```rust
// Completely matte (chalk, paper)
diffuse: 1.0,

// Partially matte (worn metal)
diffuse: 0.3,

// No diffuse scattering (mirror, glass)
diffuse: 0.0,
```

**Visual Effect:** Higher values create softer, more natural-looking surfaces that scatter light evenly in all directions.

### **Reflectivity Property**
Controls mirror-like reflections from the surface.

```rust
// Perfect mirror
reflectivity: 1.0,

// Polished metal
reflectivity: 0.8,

// Matte surface (no reflections)
reflectivity: 0.0,
```

**Visual Effect:** Higher values create clearer reflections of other objects in the scene.

### **Transparency & Index of Refraction**
Work together to create glass-like materials that bend and transmit light.

#### **Transparency:**
```rust
// Completely opaque (solid objects)
transparency: 0.0,

// Semi-transparent (tinted glass)
transparency: 0.5,

// Fully transparent (clear glass)
transparency: 1.0,
```

#### **Index of Refraction:**
Controls how much light bends when passing through transparent materials.

```rust
// Air
index_of_refraction: 1.0,

// Water
index_of_refraction: 1.33,

// Glass
index_of_refraction: 1.5,

// Diamond
index_of_refraction: 2.4,
```

**Visual Effect:** Higher values create more dramatic light bending and stronger reflections at grazing angles.

### **Emission Property**
Makes objects act as light sources (area lights).

```rust
// Non-emissive object
emission: None,

// Soft white light
emission: Some(Color::WHITE * 5.0),

// Bright colored light
emission: Some(Color::ORANGE * 20.0),

// Very bright light source
emission: Some(Color::WHITE * 50.0),
```

**Visual Effect:** Emissive objects illuminate other objects in the scene. Higher multiplier values create brighter lights.

### **Material Examples**

#### **Matte Colored Surface:**
```rust
Material {
    texture: Texture::SolidColor(Color::RED),
    diffuse: 1.0,         // Completely matte
    reflectivity: 0.0,    // No reflections
    transparency: 0.0,    // Opaque
    index_of_refraction: 0.0,
    emission: None,       // Not a light source
}
```

#### **Polished Metal:**
```rust
Material {
    texture: Texture::SolidColor(Color::SILVER),
    diffuse: 0.1,         // Slight matte component
    reflectivity: 0.9,    // Very reflective
    transparency: 0.0,    // Opaque
    index_of_refraction: 0.0,
    emission: None,
}
```

#### **Clear Glass:**
```rust
Material {
    texture: Texture::SolidColor(Color::WHITE),
    diffuse: 0.0,         // No diffuse scattering
    reflectivity: 0.0,    // Handled by transparency
    transparency: 1.0,    // Fully transparent
    index_of_refraction: 1.5,  // Glass refraction
    emission: None,
}
```

#### **Area Light:**
```rust
Material {
    texture: Texture::SolidColor(Color::WHITE),
    diffuse: 0.0,         // Lights don't scatter
    reflectivity: 0.0,    // Lights don't reflect
    transparency: 0.0,    // Lights are solid
    index_of_refraction: 0.0,
    emission: Some(Color::WHITE * 15.0),  // Bright white light
}
```

#### **Realistic Materials:**
```rust
// Frosted glass
Material {
    texture: Texture::SolidColor(Color::WHITE),
    diffuse: 0.1,         // Slight frosting
    reflectivity: 0.1,    // Some surface reflection
    transparency: 0.8,    // Mostly transparent
    index_of_refraction: 1.5,
    emission: None,
}

// Colored glass
Material {
    texture: Texture::SolidColor(Color::BLUE),
    diffuse: 0.0,
    reflectivity: 0.0,
    transparency: 0.9,
    index_of_refraction: 1.5,
    emission: None,
}
```

### **Material Tips**
- **Higher diffuse + lower reflectivity** = Natural matte surfaces
- **Lower diffuse + higher reflectivity** = Shiny metallic surfaces  
- **High transparency + appropriate IOR** = Realistic glass/water
- **Emission values** should be much brighter than surface colors (multiply by 5-50)
- **Combine properties** for complex materials (slightly reflective matte surfaces, tinted glass, etc.)

## **Particle Systems**

Generate multiple objects distributed randomly within a defined space, with automatic collision avoidance.

```rust
use crate::particle_system::ParticleSys;

let particle_system = ParticleSys::new(
    Point3::new(-5.0, 0.0, -5.0),  // min bounds
    Point3::new(5.0, 5.0, 5.0),    // max bounds  
    50,                            // particle count
    |index, position| {            // pattern function
        Box::new(Sphere::new(position, 0.2, Texture::SolidColor(Color::RED)))
    },
    0.5                           // minimum distance between particles
);

// Add all generated particles to scene
for particle in particle_system.generate() {
    scene.add_object(particle);
}
```

The pattern function receives the particle index and position, allowing you to create different object types or vary properties based on position or index.
