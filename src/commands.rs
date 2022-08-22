use crate::types::{Direction, NewImageContainer};
use crate::{platform_api_calls, AppState};
use druid::commands::OPEN_FILE;
use druid::{
    AppDelegate, Application, Command, DelegateCtx, Env, Handled, Selector, SingleUse, Target,
    WindowHandle, WindowId,
};

pub const REDRAW_IMAGE: Selector<bool> = Selector::new("redraw_image");

pub const IMAGE_LOADING_STATE: Selector<bool> = Selector::new("image_loading_state");
pub const IMAGE_LOADED: Selector<SingleUse<NewImageContainer>> = Selector::new("image_loaded");

pub const FULLSCREEN_VIEW: Selector<bool> = Selector::new("fullscreen_view");

pub const ROTATE_LEFT: Selector<bool> = Selector::new("rotate_left");
pub const ROTATE_RIGHT: Selector<bool> = Selector::new("rotate_right");

pub const ZOOM_IMAGE: Selector<bool> = Selector::new("zoom_image");
pub const RECENTER_IMAGE: Selector<bool> = Selector::new("recenter_image");

pub const DELETE_IMAGE: Selector<bool> = Selector::new("delete_image");
pub const LOAD_NEW_IMAGE: Selector<bool> = Selector::new("load_new_image");

pub const NEXT_IMAGE: Selector<bool> = Selector::new("next_image");
pub const PREV_IMAGE: Selector<bool> = Selector::new("prev_image");

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
        if let Some(image_wrapper) = cmd.get(IMAGE_LOADED) {
            data.set_current_image(image_wrapper.take());
            data.set_loading_state(false);
            Handled::Yes
        } else if let Some(state) = cmd.get(IMAGE_LOADING_STATE) {
            data.set_loading_state(*state);
            Handled::Yes
        } else if let Some(true) = cmd.get(NEXT_IMAGE) {
            data.set_loading_state(true);
            data.load_next_image();
            Handled::Yes
        } else if let Some(true) = cmd.get(PREV_IMAGE) {
            data.set_loading_state(true);
            data.load_prev_image();
            Handled::Yes
        } else if let Some(true) = cmd.get(RECENTER_IMAGE) {
            data.set_image_center_state(true);
            Handled::Yes
        } else if let Some(true) = cmd.get(ZOOM_IMAGE) {
            Handled::Yes
        } else if let Some(true) = cmd.get(ROTATE_LEFT) {
            data.set_loading_state(true);
            data.rotate_in_memory(Direction::Left);
            data.set_loading_state(false);
            Handled::Yes
        } else if let Some(true) = cmd.get(ROTATE_RIGHT) {
            data.rotate_in_memory(Direction::Right);
            Handled::Yes
        } else if let Some(true) = cmd.get(LOAD_NEW_IMAGE) {
            data.show_file_load_dialog();
            Handled::Yes
        } else if let Some(true) = cmd.get(REDRAW_IMAGE) {
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
