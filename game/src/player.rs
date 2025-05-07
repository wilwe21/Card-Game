
use fyrox::core::algebra::{ArrayStorage, Matrix, Translation};
use fyrox::event::KeyEvent;
use fyrox::graph::SceneGraph;
use fyrox::scene::dim2::rigidbody;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, UnitVector3, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    scene::{node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptTrait, ScriptDeinitContext},
    
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "79661ade-0f45-43b6-8c1d-b4452ff1b8d8")]
#[visit(optional)]
pub struct Player {
    #[reflect(hidden)]
    yaw: f32,

    #[reflect(hidden)]
    pitch: f32,

    curPitch: f32,
    curYaw: f32,

    moving: bool,
    moving_speed: f32,

    camera: Handle<Node>,

    z: f32,
    curZ: f32,
    zmoving: bool,
    zmoving_speed: f32,

    curtRot: String,
}

impl Player {
    fn camera_move(&mut self, key: PhysicalKey) {
        if self.moving || self.zmoving {
            return;
        }
        if let PhysicalKey::Code(code) = key {
            match self.curtRot.as_str() {
                "neutral" => {
                    match code {
                        KeyCode::KeyW => self.pose_board(),
                        KeyCode::ArrowUp => self.pose_board(),
                        KeyCode::KeyS => self.pose_cards(),
                        KeyCode::ArrowDown => self.pose_cards(),
                        KeyCode::KeyA => self.pose_left(),
                        KeyCode::ArrowLeft => self.pose_left(),
                        KeyCode::KeyD => self.pose_right(),
                        KeyCode::ArrowRight => self.pose_right(),
                        _ => (),
                    } 
                },
                "left" => {
                    match code {
                        KeyCode::KeyD => self.pose_neutral(),
                        KeyCode::ArrowRight => self.pose_neutral(),
                        _ => (),
                    }
                },
                "right" => {
                    match code {
                        KeyCode::KeyA => self.pose_neutral(),
                        KeyCode::ArrowLeft => self.pose_neutral(),
                        _ => (),
                    }
                },
                "cards" => {
                    match code {
                        KeyCode::KeyW => self.pose_neutral(),
                        KeyCode::ArrowUp => self.pose_neutral(),
                        _ => (),
                    }
                },
                "board" => {
                    match code {
                        KeyCode::KeyS => self.pose_neutral(),
                        KeyCode::ArrowDown => self.pose_neutral(),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn pose_neutral(&mut self) {
        if self.curtRot == "board".to_string() || 
            self.curtRot == "left".to_string() ||
            self.curtRot == "right".to_string() {
            if self.curtRot == "board".to_string() {
                self.zmoving_speed = 20_f32;
            } else {
                self.zmoving_speed = 5_f32;
            }
            self.zmoving = true;
            self.curZ = self.z;
            self.z = 0.0_f32;
        }
        self.curtRot = "neutral".to_string();
        self.moving = true;
        self.curPitch = self.pitch;
        self.curYaw = self.yaw;
        self.pitch = 0.0_f32;
        self.yaw = 0.0_f32;
    }

    fn pose_left(&mut self) {
        self.curtRot = "left".to_string();
        self.moving = true;
        self.moving_speed = 2.5_f32;
        self.curPitch = self.pitch;
        self.curYaw = self.yaw;
        self.pitch = 0.0_32;
        self.yaw = 20.0_f32;
        self.zmoving = true;
        self.zmoving_speed = 5_f32;
        self.curZ = self.z;
        self.z = 50.0_f32;
    }

    fn pose_right(&mut self) {
        self.curtRot = "right".to_string();
        self.moving = true;
        self.moving_speed = 2.5_f32;
        self.curPitch = self.pitch;
        self.curYaw = self.yaw;
        self.pitch = 0.0_32;
        self.yaw = -20.0_f32;
        self.zmoving = true;
        self.zmoving_speed = 5_f32;
        self.curZ = self.z;
        self.z = 50.0_f32;
    }

    fn pose_cards(&mut self) {
        self.curtRot = "cards".to_string();
        self.moving = true;
        self.moving_speed = 2.5_f32;
        self.curPitch = self.pitch;
        self.curYaw = self.yaw;
        self.pitch = 20.0_f32;
        self.yaw = 0.0_f32;
        self.curZ = self.z;
        self.z = 0.0_f32;
    }

    fn pose_board(&mut self) {
        self.curtRot = "board".to_string();
        self.moving = true;
        self.moving_speed = 5_f32;
        self.curPitch = self.pitch;
        self.curYaw = self.yaw;
        self.pitch = 90.0_f32;
        self.yaw = 0.0_f32;
        self.zmoving = true;
        self.zmoving_speed = 20_f32;
        self.curZ= self.z;
        self.z = 240_f32;
    }

}

impl ScriptTrait for Player {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        self.curtRot = "neutral".to_string();
        self.moving_speed = 1.0_f32;
        self.zmoving_speed = 5.0_f32;
        self.moving = false;
        self.zmoving = false;
        self.curPitch = 0.0_f32;
        self.curYaw = 0.0_f32;
        self.z = 0.0_f32;
        self.curZ = 0.0_f32;
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        match event {
            // Raw mouse input is responsible for camera rotation.
            Event::WindowEvent { event, ..} => {
                if let WindowEvent::KeyboardInput { device_id, event, is_synthetic } = event {
                    if event.state.is_pressed() {
                        self.camera_move(event.physical_key);
                    }
                }
            }
            _ => (),
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        let mut look_vector = Vector3::default();
        let mut side_vector = Vector3::default();
        if let Some(camera) = context.scene.graph.try_get_mut(self.camera) {
            if camera.is_rigid_body() {
                let ri = camera.as_rigid_body_mut();
                if self.zmoving {
                    println!("z:{}, curZ:{}", self.z, self.curZ);
                    if self.z > self.curZ {
                        ri.set_lin_vel(Vector3::new(0.0, 0.0, self.zmoving_speed));
                        self.curZ += self.zmoving_speed;
                    } else if self.z < self.curZ {
                        ri.set_lin_vel(Vector3::new(0.0, 0.0, -self.zmoving_speed));
                        self.curZ -= self.zmoving_speed;
                    } else {
                        ri.set_lin_vel(Vector3::new(0.0, 0.0, 0.0));
                        self.zmoving = false;
                    }
                }
            }

            look_vector = camera.look_vector();
            side_vector = camera.side_vector();

            let mut mov_yaw = self.yaw.to_radians();
            let mut mov_pitch = self.pitch.to_radians();
            if self.moving {
                if self.curYaw > self.yaw.floor() {
                    if self.curYaw -self.moving_speed > self.yaw.floor() {
                        mov_yaw = (self.curYaw.floor() - self.moving_speed).to_radians();
                        self.curYaw -= self.moving_speed;
                    } else {
                        //mov_yaw = self.yaw.floor();
                        //self.curYaw = self.yaw.floor();
                        self.moving = false;
                    }
                } else if self.curYaw < self.yaw.floor() {
                    if self.curYaw + self.moving_speed < self.yaw {
                        mov_yaw = (self.curYaw.floor() + self.moving_speed).to_radians();
                        self.curYaw += self.moving_speed;
                    } else {
                        //mov_yaw = self.yaw.floor();
                        //self.curYaw = self.yaw.floor()
                        self.moving = false;
                    }
                }
                if self.curPitch > self.pitch.floor() {
                    if self.curPitch.floor() - self.moving_speed > self.pitch.floor() {
                        mov_pitch = (self.curPitch.floor() - self.moving_speed).to_radians();
                        self.curPitch -= self.moving_speed;
                    } else {
                        //mov_pitch = self.pitch.floor();
                        //self.curPitch = self.pitch.floor();
                        self.moving = false;
                    }
                } else if self.curPitch < self.pitch.floor() {
                    if self.curPitch.floor() - self.moving_speed < self.pitch.floor() {
                        mov_pitch = (self.curPitch.floor() + self.moving_speed).to_radians();
                        self.curPitch += self.moving_speed;
                    } else {
                        //mov_pitch = self.pitch.floor();
                        //self.curPitch = self.pitch.floor();
                        self.moving = false;
                    }
                }

                if self.curYaw.floor() == self.yaw.floor() && self.curPitch.floor() == self.pitch.floor() {
                    self.moving = false;
                }
            }

            let yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), mov_yaw);
            let transform = camera.local_transform_mut();

            
            transform.set_rotation(
                UnitQuaternion::from_axis_angle(
                    &UnitVector3::new_normalize(yaw * Vector3::x()),
                    mov_pitch,
                ) * yaw,
            );
        }
    }
}
    
