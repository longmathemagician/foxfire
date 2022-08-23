use std::path::PathBuf;
use std::time::Instant;

use crate::types::{Direction, NewImageContainer};
use crate::{platform_api_calls, AppState};
use druid::commands::OPEN_FILE;
use druid::{
    AppDelegate, Application, Command, DelegateCtx, Env, Handled, Selector, SingleUse, Target,
    WindowHandle, WindowId,
};

pub const REDRAW_IMAGE: Selector<()> = Selector::new("redraw_image");

pub const IMAGE_LOAD_FAILURE: Selector<PathBuf> = Selector::new("image_load_failure");
pub const IMAGE_LOAD_SUCCESS: Selector<SingleUse<NewImageContainer>> =
    Selector::new("image_loaded");

pub const FULLSCREEN_VIEW: Selector<Instant> = Selector::new("fullscreen_view");

pub const ROTATE_LEFT: Selector<Instant> = Selector::new("rotate_left");
pub const ROTATE_RIGHT: Selector<Instant> = Selector::new("rotate_right");

pub const ZOOM_IMAGE: Selector<Instant> = Selector::new("zoom_image");
pub const RECENTER_IMAGE: Selector<Instant> = Selector::new("recenter_image");

pub const DELETE_IMAGE: Selector<Instant> = Selector::new("delete_image");
pub const LOAD_NEW_IMAGE: Selector<Instant> = Selector::new("load_new_image");

pub const NEXT_IMAGE: Selector<Instant> = Selector::new("next_image");
pub const PREV_IMAGE: Selector<Instant> = Selector::new("prev_image");

pub struct Delegate;

impl Delegate {
    pub fn new() -> Self {
        Self
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(image_wrapper) = cmd.get(IMAGE_LOAD_SUCCESS) {
            data.set_current_image(image_wrapper.take());
            data.set_loading_state(false);
            Handled::Yes
        } else if let Some(image_path) = cmd.get(IMAGE_LOAD_FAILURE) {
            data.set_loading_state(false);
            data.image_load_failure(image_path);
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(NEXT_IMAGE) {
            data.set_loading_state(true);
            data.load_next_image(command_timestamp);
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(PREV_IMAGE) {
            data.set_loading_state(true);
            data.load_prev_image(command_timestamp);
            Handled::Yes
        } else if cmd.get(RECENTER_IMAGE).is_some() {
            data.set_image_center_state(true);
            Handled::Yes
        } else if cmd.get(ZOOM_IMAGE).is_some() {
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(ROTATE_LEFT) {
            data.set_loading_state(true);
            data.rotate_in_memory(Direction::Left, command_timestamp);
            data.set_loading_state(false);
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(ROTATE_RIGHT) {
            data.rotate_in_memory(Direction::Right, command_timestamp);
            Handled::Yes
        } else if cmd.get(LOAD_NEW_IMAGE).is_some() {
            data.show_file_load_dialog();
            Handled::Yes
        } else if cmd.get(REDRAW_IMAGE).is_some() {
            Handled::No // Pass down to container widget's event handler
        } else if let Some(file_info) = cmd.get(OPEN_FILE) {
            let file_path = file_info.path.to_str();
            if let Some(path_string) = file_path {
                data.startup(path_string.to_string());
            } else {
                println!("Failed to parse image path")
            }

            Handled::Yes
        } else {
            Handled::No
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        platform_api_calls(id);
        _data.set_window_id(id);
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        _data.close_current_image();
        Application::global().quit()
    }
}
