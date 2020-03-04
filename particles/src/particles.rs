use math::projection;
use math::vec2::*;
use math::{mat2x3, mat4::Mat4F32};
use rand::{thread_rng, Rng};
use rendering::*;
use std::cell::{Cell, RefCell};
use std::time::Instant;
use sys::input::*;

fn slice_bytes_len<T>(s: &[T]) -> usize {
    s.len() * std::mem::size_of::<T>()
}
#[derive(Copy, Clone, Debug)]
struct VertexPT {
    pos: Vec2F32,
    uv: Vec2F32,
}

mod physics {
    pub const ROTATION_STEP: f32 = 1.0f32;
    pub const MAX_PARTICLES: u32 = 1024;
    pub const GRAVITY_ACCEL: f32 = -9.8f32;
    pub const AIR_DENSITY: f32 = 1.23f32; // kg/m^3
    pub const DRAG_COEFF: f32 = 0.6f32;
    pub const WIND_SPEED: f32 = 10f32; // m/sec
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct ParticleGPU {
    transform: Mat4F32,
    texid: u32,
    pad: [u32; 3],
}

// changes every frame
#[derive(Copy, Clone, Debug)]
struct ParticlePhysics {
    speed: f32,
    rotation: f32, // angle of rotation in radians
    position: Vec2F32,
    velocity: Vec2F32,
    forces: Vec2F32,
}

impl ParticlePhysics {
    fn compute_loads(&mut self, gravity: Vec2F32) {
        self.forces = Vec2F32::default();
        self.forces += gravity;

        // //
        // // air drag
        // let vdrag = normalize(-self.velocity);
        // let fdrag = 0.5f32
        //     * physics::AIR_DENSITY
        //     * self.speed
        //     * (std::f32::consts::PI * 16f32 * 16f32)
        //     * physics::DRAG_COEFF;

        // let vdrag = vdrag * fdrag;
        // // dbg!(vdrag);
        // // dbg!(fdrag);

        // self.forces += vdrag;

        // //
        // // x direction wind
        // let wind = 0.5f32
        //     * physics::AIR_DENSITY
        //     * physics::WIND_SPEED
        //     * physics::WIND_SPEED
        //     * (std::f32::consts::PI * self.radius * self.radius)
        //     * physics::DRAG_COEFF;

        // self.forces.x += wind;
    }

    fn update_body_euler(&mut self, delta: f32, mass: f32) {
        let a = self.forces / mass;
        let dv = a * delta;
        self.velocity = self.velocity + dv;

        let ds = self.velocity * delta;
        self.position = self.position + ds;
        self.speed = self.velocity.len();
    }

    fn update_rotation(&mut self, delta: f32) {
        self.rotation += physics::ROTATION_STEP * delta;
        if self.rotation >= std::f32::consts::PI * 2f32 {
            self.rotation -= std::f32::consts::PI * 2f32;
        }
    }
}

struct Particle {
    radius: f32,
    mass: f32,
    gravity: Vec2F32,
    texid: u32,
}

struct PhysicsState {
    particle_prev_state: Vec<ParticlePhysics>,
    particle_curr_state: Vec<ParticlePhysics>,
    particles: Vec<Particle>,
    world_size: Vec2F32,
    accumulated_time: f32,
    delta_step: f32,
}

impl PhysicsState {
    const TARGET_FPS: i32 = 120;

    fn new(world_size: Vec2F32, particles: u32) -> Self {
        let mut rng = thread_rng();

        let particles_phys = (0..particles)
            .map(|_| ParticlePhysics {
                speed: 0f32,
                position: Vec2F32 {
                    x: rng.gen_range(0f32, world_size.x),
                    y: world_size.y,
                },
                velocity: Vec2F32::default(),
                forces: Vec2F32::default(),
                rotation: rng.gen_range(0f32, 2f32 * std::f32::consts::PI),
            })
            .collect::<Vec<_>>();

        Self {
            particles: (0..particles)
                .map(|_| {
                    //
                    // corelate mass with radius so larger balls are heavier
                    const PARTICLE_MASS_MULTIPLIER: f32 = 0.001f32;
                    let radius = rng.gen_range(16f32, 64f32);

                    Particle {
                        radius,
                        mass: radius * PARTICLE_MASS_MULTIPLIER,
                        texid: rng.gen_range(0u32, 3u32),
                        gravity: Vec2F32::new(0f32, physics::GRAVITY_ACCEL),
                    }
                })
                .collect(),
            particle_prev_state: particles_phys.clone(),
            particle_curr_state: particles_phys,
            world_size: world_size,
            accumulated_time: 0f32,
            delta_step: 1f32 / Self::TARGET_FPS as f32,
        }
    }

    fn integrate(&mut self, dt: f32) {
        (0..self.particle_curr_state.len()).for_each(|idx| {
            self.particle_prev_state[idx] = self.particle_curr_state[idx];

            let p = &mut self.particle_curr_state[idx];
            let pdata = &self.particles[idx];
            p.compute_loads(pdata.gravity);
            p.update_body_euler(dt, pdata.mass);
            p.update_rotation(dt);

            if p.position.x > self.world_size.x || p.position.y < 0f32 {
                //
                // reset particle
                let mut rng = thread_rng();

                *p = ParticlePhysics {
                    speed: 0f32,
                    position: Vec2F32 {
                        x: rng.gen_range(0f32, self.world_size.x),
                        y: self.world_size.y,
                    },
                    velocity: Vec2F32::default(),
                    forces: Vec2F32::default(),
                    rotation: rng.gen_range(0f32, 2f32 * std::f32::consts::PI),
                };
                //
                // also reset previous state otherwise it leads to incorrect positioning
                // for the first time the reset particle is drawn
                self.particle_prev_state[idx] = *p;
            }
        });
    }

    fn update(&mut self, frame_time: f32) -> f32 {
        self.accumulated_time += frame_time;

        while self.accumulated_time >= self.delta_step {
            //
            // update sim with delta step
            self.integrate(self.delta_step);
            self.accumulated_time -= self.delta_step;
        }

        self.accumulated_time / self.delta_step
    }
}

mod sprites {
    pub const CACODEMON_SPRITE_WIDTH: i32 = 220;
    pub const CACODEMON_SPRITE_HEIGHT: i32 = 240;
}

struct RenderingState {
    vertexbuffer: UniqueBuffer,
    indexbuffer: UniqueBuffer,
    instancebuffer: UniqueBuffer,
    vertexarray: UniqueVertexArray,
    vertshader: UniqueShaderProgram,
    fragshader: UniqueShaderProgram,
    pipeline: UniquePipeline,
    sprites: UniqueTexture,
    sampler: UniqueSampler,
    elements: u32,
}

impl RenderingState {
    fn load_cacodemons() -> Result<UniqueTexture, String> {
        // let
        let texarray = UniqueTexture::new(unsafe {
            let mut tex = 0u32;
            gl::CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut tex);
            gl::TextureStorage3D(
                tex,
                1,
                gl::RGBA8,
                sprites::CACODEMON_SPRITE_WIDTH,
                sprites::CACODEMON_SPRITE_HEIGHT,
                3,
            );

            tex
        })
        .ok_or_else(|| "Failed to create sprites texture array!".to_string())?;

        let mut sprite_buffer: Vec<u8> = vec![
            0u8;
            (sprites::CACODEMON_SPRITE_WIDTH * sprites::CACODEMON_SPRITE_HEIGHT)
                as usize
                * 4usize
        ];

        (0..3).for_each(|sprite_idx| {
            use png::Decoder;
            use std::fs::File;

            let sprite_path = format!("data/sprites/cacodemons/cacodemon{}.png", sprite_idx + 1);
            let sprite_file = File::open(sprite_path).unwrap();

            let decoder = Decoder::new(sprite_file);
            let (info, mut reader) = decoder.read_info().unwrap();
            debug_assert!(info.width == sprites::CACODEMON_SPRITE_WIDTH as u32);
            debug_assert!(info.height == sprites::CACODEMON_SPRITE_HEIGHT as u32);

            reader.next_frame(&mut sprite_buffer).unwrap();

            unsafe {
                gl::TextureSubImage3D(
                    *texarray,
                    0,
                    0,
                    0,
                    sprite_idx as i32,
                    sprites::CACODEMON_SPRITE_WIDTH,
                    sprites::CACODEMON_SPRITE_HEIGHT,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    sprite_buffer.as_ptr() as *const gl::types::GLvoid,
                );
            }
        });

        Ok(texarray)
    }

    /// Generates geometry (vertices and indices) for a circle centered around the origin with a radius of 1.
    fn generate_circle_geometry(tess_factor: u32, radius: f32) -> (Vec<Vec2F32>, Vec<u16>) {
        let step = std::f32::consts::PI * 2f32 / tess_factor as f32;

        let vertices = std::iter::once(Vec2F32::same(0f32))
            .chain((0..=tess_factor).map(|idx| {
                let theta = idx as f32 * step;
                Vec2F32::new(radius * theta.cos(), radius * theta.sin())
            }))
            .collect::<Vec<_>>();

        let indices = (0..tess_factor).fold(Vec::new(), |mut indices, idx| {
            indices.push(0u16);
            indices.push(idx as u16 + 1);
            indices.push(idx as u16 + 2);
            indices
        });

        (vertices, indices)
    }

    pub fn new() -> Result<RenderingState, String> {
        let quad_verts: [VertexPT; 4] = [
            VertexPT {
                pos: Vec2F32::new(-1f32, -1f32),
                uv: Vec2F32::new(0f32, 0f32),
            },
            VertexPT {
                pos: Vec2F32::new(-1f32, 1f32),
                uv: Vec2F32::new(0f32, 1f32),
            },
            VertexPT {
                pos: Vec2F32::new(1f32, 1f32),
                uv: Vec2F32::new(1f32, 1f32),
            },
            VertexPT {
                pos: Vec2F32::new(1f32, -1f32),
                uv: Vec2F32::new(1f32, 0f32),
            },
        ];

        let quad_indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let vertexbuffer = UniqueBuffer::new(unsafe {
            let mut buff = 0u32;
            gl::CreateBuffers(1, &mut buff);
            gl::NamedBufferStorage(
                buff,
                slice_bytes_len(&quad_verts) as isize,
                quad_verts.as_ptr() as *const _,
                0,
            );
            buff
        })
        .ok_or_else(|| "Failed to create vertex buffer".to_string())?;

        let indexbuffer = UniqueBuffer::new(unsafe {
            let mut buff = 0u32;
            gl::CreateBuffers(1, &mut buff);
            gl::NamedBufferStorage(
                buff,
                slice_bytes_len(&quad_indices) as isize,
                quad_indices.as_ptr() as *const _,
                0,
            );
            buff
        })
        .ok_or_else(|| "Failed to create index buffer".to_string())?;

        let instancebuffer = UniqueBuffer::new(unsafe {
            let mut buff = 0u32;
            gl::CreateBuffers(1, &mut buff);
            gl::NamedBufferStorage(
                buff,
                physics::MAX_PARTICLES as isize * std::mem::size_of::<ParticleGPU>() as isize,
                std::ptr::null(),
                gl::MAP_WRITE_BIT,
            );
            buff
        })
        .ok_or_else(|| "Failed to create instance buffer".to_string())?;

        let vertexarray = UniqueVertexArray::new(unsafe {
            let mut vao = 0u32;
            gl::CreateVertexArrays(1, &mut vao);
            gl::VertexArrayVertexBuffer(
                vao,
                0,
                *vertexbuffer,
                0,
                std::mem::size_of::<VertexPT>() as i32,
            );
            gl::VertexArrayElementBuffer(vao, *indexbuffer);

            gl::VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
            gl::VertexArrayAttribBinding(vao, 0, 0);
            gl::EnableVertexArrayAttrib(vao, 0);

            gl::VertexArrayAttribFormat(vao, 1, 2, gl::FLOAT, gl::FALSE, 8);
            gl::VertexArrayAttribBinding(vao, 1, 0);
            gl::EnableVertexArrayAttrib(vao, 1);

            vao
        })
        .ok_or_else(|| "Failed to create vertex array!".to_string())?;

        let vertshader = create_shader_program_from_string(
            include_str!("../../data/shaders/particles.vert"),
            ShaderType::Vertex,
        )?;

        let fragshader = create_shader_program_from_string(
            include_str!("../../data/shaders/particles.frag"),
            ShaderType::Fragment,
        )?;

        let pipeline = PipelineBuilder::new()
            .add_vertex_shader(&vertshader)
            .add_fragment_shader(&fragshader)
            .build()?;

        let sprites = Self::load_cacodemons()?;
        let sampler = SamplerBuilder::new().build()?;

        Ok(RenderingState {
            vertexbuffer,
            indexbuffer,
            instancebuffer,
            vertexarray,
            vertshader,
            fragshader,
            pipeline,
            sprites,
            sampler,
            elements: quad_indices.len() as u32,
        })
    }
}

pub struct ParticlesSim {
    phys: RefCell<PhysicsState>,
    draw: RenderingState,
    prev_time: Cell<Instant>,
    curr_time: Cell<Instant>,
}

impl ParticlesSim {
    pub fn new(width: i32, height: i32) -> Result<ParticlesSim, String> {
        let draw = RenderingState::new()?;
        Ok(ParticlesSim {
            phys: RefCell::new(PhysicsState::new(
                Vec2F32::new(width as f32, height as f32),
                physics::MAX_PARTICLES,
            )),
            draw,
            prev_time: Cell::new(Instant::now()),
            curr_time: Cell::new(Instant::now()),
        })
    }

    fn handler_loop_event(&self, _evt: LoopEventData) {
        let new_time = Instant::now();
        const MAX_FRAME_TIME: f32 = 0.25f32;
        let frame_time =
            MAX_FRAME_TIME.min((new_time - self.curr_time.get()).as_millis() as f32 * 0.001f32); // frame time is in seconds
        self.curr_time.set(new_time);

        let bounds = self.phys.borrow().world_size;
        let proj_matrix = projection::orthographic(0f32, 0f32, bounds.x, bounds.y, -1f32, 1f32);

        self.update(frame_time, &proj_matrix);
        self.draw();
    }

    fn draw(&self) {
        const CLEAR_COLOR: [f32; 4] = [0f32, 0f32, 0f32, 1f32];

        unsafe {
            gl::ClearNamedFramebufferfv(0, gl::COLOR, 0, CLEAR_COLOR.as_ptr());
            gl::ClearNamedFramebufferfi(0, gl::DEPTH_STENCIL, 0, 1f32, 0);

            gl::BindTextureUnit(0, *self.draw.sprites);
            gl::BindSampler(0, *self.draw.sampler);
            gl::BindVertexArray(*self.draw.vertexarray);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, *self.draw.instancebuffer);
            gl::BindProgramPipeline(*self.draw.pipeline);
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.draw.elements as i32,
                gl::UNSIGNED_SHORT,
                std::ptr::null(),
                self.phys.borrow().particles.len() as i32,
            );

            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, 0);
            gl::BindVertexArray(0);
            gl::BindProgramPipeline(0);
        }
    }

    fn update(&self, delta: f32, proj_view: &Mat4F32) {
        let frame_interp = self.phys.borrow_mut().update(delta);

        if let Some(vbmap) = UniqueBufferMapping::new(
            *self.draw.instancebuffer,
            gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT,
        ) {
            let num_particles = self.phys.borrow().particles.len();

            let instances = unsafe {
                std::slice::from_raw_parts_mut(vbmap.memory() as *mut ParticleGPU, num_particles)
            };

            let phys = self.phys.borrow();

            instances
                .iter_mut()
                .enumerate()
                .for_each(|(idx, gpu_particle)| {
                    let fixed_data = &phys.particles[idx];

                    let current_pos = Vec2F32 {
                        y: phys.world_size.y - phys.particle_curr_state[idx].position.y,
                        ..phys.particle_curr_state[idx].position
                    };

                    let previous_pos = Vec2F32 {
                        y: phys.world_size.y - phys.particle_prev_state[idx].position.y,
                        ..phys.particle_prev_state[idx].position
                    };

                    let translation =
                        current_pos * frame_interp + (1f32 - frame_interp) * previous_pos;

                    let previous_rot = phys.particle_prev_state[idx].rotation;
                    let current_rot = phys.particle_curr_state[idx].rotation;

                    let rotation =
                        current_rot * frame_interp + (1f32 - frame_interp) * previous_rot;
                    let particle_scale = fixed_data.radius;

                    use mat2x3::transforms;

                    let world_transform = transforms::translate(translation.x, translation.y)
                        * transforms::rotate(rotation)
                        * transforms::uniform_scale(particle_scale);

                    gpu_particle.transform = (*proj_view * world_transform.into()).transpose();
                    gpu_particle.texid = fixed_data.texid;
                });
        }
    }

    fn handler_resize_event(&self, re: WindowConfigureEventData) {
        self.phys.borrow_mut().world_size = Vec2F32::new(re.width as f32, re.height as f32);

        unsafe {
            gl::ViewportIndexedf(0, 0f32, 0f32, re.width as f32, re.height as f32);
        }
    }

    pub fn main_loop(&self, evt: &Event) {
        match evt {
            Event::Loop(el) => self.handler_loop_event(*el),
            Event::Configure(ec) => self.handler_resize_event(*ec),
            _ => {}
        }
    }
}
