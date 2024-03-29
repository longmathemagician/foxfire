use std::path::PathBuf;
use std::time::Instant;

use crate::types::{Direction, DisplayState, NewImageContainer};
use crate::{platform_api_calls, AppState};
use druid::commands::OPEN_FILE;
use druid::{
    AppDelegate, Command, DelegateCtx, Env, Handled, Selector, SingleUse, Target, WindowHandle,
    WindowId,
};

pub const REDRAW_IMAGE: Selector<()> = Selector::new("redraw_image");

pub const TOGGLE_BLUR: Selector<()> = Selector::new("toggle_blur");

pub const IMAGE_LOAD_FAILURE: Selector<PathBuf> = Selector::new("image_load_failure");
pub const IMAGE_LOAD_SUCCESS: Selector<SingleUse<NewImageContainer>> =
    Selector::new("image_loaded");

pub const FULLSCREEN_VIEW: Selector<Instant> = Selector::new("fullscreen_view");

pub const ROTATE_LEFT: Selector<Instant> = Selector::new("rotate_left");
pub const ROTATE_RIGHT: Selector<Instant> = Selector::new("rotate_right");
pub const IMAGE_ROTATION_COMPLETE: Selector<SingleUse<NewImageContainer>> =
    Selector::new("image_rotated");

pub const ZOOM_IMAGE: Selector<Instant> = Selector::new("zoom_image");
pub const RECENTER_IMAGE: Selector<Instant> = Selector::new("recenter_image");
pub const REALSIZE_IMAGE: Selector<Instant> = Selector::new("realsize_image");

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
            data.load_next_image(command_timestamp);
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(PREV_IMAGE) {
            data.load_prev_image(command_timestamp);
            Handled::Yes
        }
        // The next three events are also partially handled by the ContainerWidget
        else if cmd.get(ZOOM_IMAGE).is_some() {
            data.set_display_state(DisplayState::Zoomed(true));
            Handled::No
        } else if cmd.get(RECENTER_IMAGE).is_some() {
            data.set_display_state(DisplayState::Centered(true));
            Handled::No
        } else if cmd.get(REALSIZE_IMAGE).is_some() {
            data.set_display_state(DisplayState::RealSize(true));
            Handled::No
        } else if cmd.get(FULLSCREEN_VIEW).is_some() {
            data.show_fullscreen_slideshow();
            Handled::Yes
        } else if cmd.get(DELETE_IMAGE).is_some() {
            data.delete_image();
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(ROTATE_LEFT) {
            data.rotate_in_memory(Direction::Left, command_timestamp);
            Handled::Yes
        } else if let Some(command_timestamp) = cmd.get(ROTATE_RIGHT) {
            data.rotate_in_memory(Direction::Right, command_timestamp);
            Handled::Yes
        } else if let Some(image_wrapper) = cmd.get(IMAGE_ROTATION_COMPLETE) {
            data.set_current_image(image_wrapper.take());
            data.set_rotating_state(false);
            data.redraw_widgets();
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
        } else if cmd.get(TOGGLE_BLUR).is_some() {
            data.blur_enable_toggle();
            Handled::Yes
        } else {
            Handled::No
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        platform_api_calls(id);
        data.set_window_id(id);
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        data.exit();
    }
}
