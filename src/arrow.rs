use crate::vec3::Vec3;

// ref (1.17.1): net/minecraft/entity/projectile/ArrowEntity.java
// ref (wiki): https://minecraft.wiki/w/Arrow#Movement
#[derive(Copy, Clone, Debug)]
pub struct Arrow {
    pub velocity: Vec3<f64>,
    pub pos: Vec3<f64>,
}

impl Arrow {
    pub fn new(x: f64, y: f64, z: f64, speed: f64) -> Self {
        Self {
            velocity: Vec3::new(x,y,z).normalize() * speed,
            pos: Vec3::new(0.0,0.0,0.0),
        }
    }

    pub fn tick(&mut self) {
        let lv = self.velocity;
        let pos = self.pos;
        self.pos = pos.add(lv);
        self.velocity = lv * 0.99 - Vec3::<f64>::new(0.0, 0.05, 0.0);
    }
}
