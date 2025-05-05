//! Game project.
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*}, event::{Event, WindowEvent}, gui::message::{KeyCode, UiMessage}, keyboard::PhysicalKey, plugin::{Plugin, PluginContext, PluginRegistrationContext}, scene::Scene
};
use std::path::Path;

// Re-export the engine.
pub use fyrox;

pub mod player;
use crate::player::Player;

#[derive(Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn register(&self, _context: PluginRegistrationContext) {
        _context.serialization_context.script_constructors.add::<Player>("Player");
        // Register your scripts here.
    }
    
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
    ) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
