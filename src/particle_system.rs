use crate::game_rand::GameRand;
use crate::vector::*;

use std::mem::size_of;

pub enum ColorScheme {
    Fire,
    Ice,
    Smoke,
    Rainbow,
}

struct Particle {
    pos: vec3,
    size: f32,

    dir: vec3,
    life: f32,
    invInitialLife: f32,
}

struct PointForce {
    pos: vec3,
    strength: f32,
    linearAttenuation: f32,
    quadraticAttenuation: f32,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    pointForces: Vec<PointForce>,
    directionalForce: vec3,

    lastTime: f32, 
    particleCredit: f32,

    colors: [vec4; 12],
    pos: vec3,

    spawnRate: f32,
    speed: f32,
    speedSpread: f32,
    size: f32,
    sizeSpread: f32,
    life: f32,
    lifeSpread: f32,
    frictionFactor: f32,

    vertexArray: Vec<u8>,
    indexArray: Vec<u16>,

    rand: GameRand, // DT_TODO: Pass this as a parameter?
}

impl ParticleSystem {

    pub fn new() -> ParticleSystem {
        let mut p = ParticleSystem {
            particles : Vec::with_capacity(20),
            pointForces : Vec::with_capacity(2),
            directionalForce : vec3(0.0,0.0, 0.0),
            lastTime : 0.0,
            particleCredit : 0.0,

            colors : [vec4(0.0,0.0,0.0,0.0); 12], 
            pos : vec3(0.0, 0.0, 0.0),
        
            spawnRate : 10.0,
            speed : 100.0,
            speedSpread : 25.0,
        
            size : 100.0,
            sizeSpread : 10.0,
        
            life : 2.5,
            lifeSpread : 0.5,
        
            frictionFactor : 0.7,
        
            vertexArray : Vec::new(),
            indexArray : Vec::new(),

            rand : GameRand::new(2356),
        };
        p.set_color_scheme(ColorScheme::Fire);
        p
    }

    fn random(&mut self, mean: f32, diff: f32) -> f32 {
        let r: f32 = 2.0 * self.rand.next_random01() - 1.0;
        return mean + r * r.abs() * diff;
    }

    fn get_position(&self) -> &vec3 {
        &self.pos
    }

    pub fn get_particle_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn set_position(&mut self, position: &vec3) {
        self.pos = *position;
    }

    pub fn set_spawn_rate(&mut self, spawn_rate: f32) {
        self.spawnRate = spawn_rate;
    }

    pub fn set_speed(&mut self, meanSpeed: f32, spread: f32) {
        self.speed = meanSpeed;
        self.speedSpread = spread;
    }

    pub fn set_life(&mut self, meanLife: f32, spread: f32) {
        self.life = meanLife;
        self.lifeSpread = spread;
    }

    pub fn setSize(&mut self, meanSize: f32, spread: f32) {
        self.size = meanSize;
        self.sizeSpread = spread;
    }

    pub fn set_directional_force(&mut self, df: &vec3) {
        self.directionalForce = *df;
    }

    pub fn add_point_force(&mut self, pf: PointForce) {
        self.pointForces.push(pf);
    }

    pub fn set_point_force(&mut self, force: i32, pf: PointForce) {
        self.pointForces[force as usize] = pf;
    }

    pub fn set_friction_factor(&mut self, friction: f32) {
        self.frictionFactor = friction;
    }

    pub fn setColor(&mut self, color: i32, col: vec4) {
        self.colors[color as usize] = col;
    }

    pub fn set_color_scheme(&mut self, colorScheme: ColorScheme) {
        match colorScheme {
            ColorScheme::Fire => {
                for i in 0..4 {
                    let f = i as f32;
                    self.colors[i] = vec4(f / 4.0, 0.0, 0.0, 0.0);
                    self.colors[i + 4] = vec4(1.0, f / 4.0, 0.0, 0.0);
                    self.colors[i + 8] = vec4((3.0 - f) / 3.0, (3.0 - f) / 3.0, 1.0, 0.0);
                }
            }
            ColorScheme::Ice => {
                for i in 0..6 {
                    let f = i as f32;
                    self.colors[i] = vec4(0.0, 0.0, f / 6.0, 0.0);
                    self.colors[i + 6] = vec4(f / 5.0, 1.0, 1.0, 0.0);
                }
            }
            ColorScheme::Smoke => {
                for i in 0..12 {
                    let f: f32 = i as f32 / 44.0;
                    self.colors[i] = vec4(f, f, f, f);
                }
            }
            ColorScheme::Rainbow => {
                self.colors[0] = vec4(0.0, 0.0, 0.0, 0.0);
                self.colors[1] = vec4(0.0, 0.0, 0.25, 0.0);
                self.colors[2] = vec4(0.0, 0.0, 0.5, 0.0);
                self.colors[3] = vec4(0.0, 0.0, 1.0, 0.0);
                self.colors[4] = vec4(0.0, 0.5, 1.0, 0.0);
                self.colors[5] = vec4(0.0, 1.0, 1.0, 0.0);
                self.colors[6] = vec4(0.0, 1.0, 0.5, 0.0);
                self.colors[7] = vec4(0.0, 1.0, 0.0, 0.0);
                self.colors[8] = vec4(0.5, 1.0, 0.0, 0.0);
                self.colors[9] = vec4(1.0, 1.0, 0.0, 0.0);
                self.colors[10] = vec4(1.0, 0.5, 0.0, 0.0);
                self.colors[11] = vec4(1.0, 0.0, 0.0, 0.0);
            }
        }
    }

    fn update_particle(p: &mut Particle, time: f32) {
        p.pos += p.dir * time;
    }

    pub fn update(&mut self, timeStamp: f32) {
        let time = timeStamp - self.lastTime;
        self.lastTime = timeStamp;

        self.particleCredit += time * self.spawnRate;
        let len = self.particleCredit as u32;
        self.particleCredit -= len as f32;

        for _ in 0..len {
            let life = self.random(self.life, self.lifeSpread);
            let p = Particle {
                pos: self.pos,
                dir: normalize(&vec3(self.random(0.0, 0.3), 1.0, self.random(0.0, 0.3)))
                    * self.random(self.speed, self.speedSpread),

                size: self.random(self.size, self.sizeSpread),
                life,
                invInitialLife: 1.0 / life,
                //depth : 0.0,
            };
            self.particles.push(p);
        }

        let friction = self.frictionFactor.powf(time);

        self.particles.retain_mut(|p| {
            p.life -= time;
            if p.life < 0.0 {
                return false;
            }

            let mut v = vec3(0.0, 0.0, 0.0);
            for f in &self.pointForces {
                let dir = f.pos - p.pos;
                let dist = dot(&dir, &dir);
                v += dir
                    * (f.strength
                        / (1.0
                            + dist.sqrt() * f.linearAttenuation
                            + dist * f.quadraticAttenuation));
            }

            p.dir += (self.directionalForce + v) * time;
            p.dir *= friction;

            Self::update_particle(p, time);
            true
        });
    }

    pub fn updateTime(&mut self, timeStamp: f32) {
        self.lastTime = timeStamp;
    }

    pub fn getIndexArray(&mut self) -> &[u16] {
        let old_size = self.indexArray.len();
        let new_size = self.particles.len() * 6;

        if new_size > old_size {
            self.indexArray.reserve(new_size);

            let start_index = (old_size / 6) as u16;
            let end_index = self.particles.len() as u16;
            for i in start_index..end_index {
                self.indexArray.push(4 * i);
                self.indexArray.push(4 * i + 1);
                self.indexArray.push(4 * i + 3);
                self.indexArray.push(4 * i + 3);
                self.indexArray.push(4 * i + 1);
                self.indexArray.push(4 * i + 2);
            }
        }

        &self.indexArray
    }

    unsafe fn copy_to_buffer<T>(buffer: *mut u8, data: T) -> *mut u8 {
        let data = &data as *const T as * const u8;

        buffer.copy_from_nonoverlapping(data, size_of::<T>());
        buffer.add(size_of::<T>())
    }

    pub fn getVertexArray(&mut self, dx: vec3, dy: vec3, useColors: bool, tex3d: bool) -> &[u8] {
        let mut vertexSize = size_of::<vec3>() + size_of::<vec2>();
        if useColors {
            vertexSize += size_of::<vec4>();
        }
        if tex3d {
            vertexSize += size_of::<f32>();
        }

        let size = self.particles.len() * vertexSize * 4;
        if size > self.vertexArray.len() {
            self.vertexArray.resize(size, 0);
        }

        const COORDS : [vec2; 4] = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
        let vect  = [-dx + dy, dx + dy, dx - dy, -dx - dy];

        let mut frac : f32 = 0.0;
        let mut color : vec4 = vec4(0.0,0.0,0.0,0.0);
        let destRange = self.vertexArray.as_mut_ptr_range();
        let mut destPtr = destRange.start;
        for p in &self.particles {

            if useColors || tex3d {
                frac = p.life * p.invInitialLife;
            }

            if useColors {
                let mut colFrac = 11.0 * frac;
                let colInt = colFrac as i32;
                colFrac -= colInt as f32;
    
                color = lerp(&self.colors[colInt as usize], &self.colors[colInt as usize + 1], colFrac);
            }

            for j in 0..4 {

                let pos = p.pos + vect[j] * p.size;
                unsafe {
                    destPtr = Self::copy_to_buffer(destPtr, pos);
                    destPtr = Self::copy_to_buffer(destPtr, COORDS[j]);

                    if tex3d {
                        destPtr = Self::copy_to_buffer(destPtr, 1.0 - frac);
                    }
        
                    if useColors {
                        destPtr = Self::copy_to_buffer(destPtr, color);
                    }
                }
            }
        }
        debug_assert!(destPtr <= destRange.end);

        &self.vertexArray
    }
}

