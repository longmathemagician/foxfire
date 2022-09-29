use crate::app_state::AppState;
use crate::button_widget::*;
use crate::commands::{
    DELETE_IMAGE, NEXT_IMAGE, PREV_IMAGE, REALSIZE_IMAGE, RECENTER_IMAGE, ROTATE_LEFT,
    ROTATE_RIGHT, ZOOM_IMAGE,
};
use crate::types::DisplayState;
use crate::TOGGLE_BLUR;
use druid::widget::prelude::*;
use druid::widget::Svg;
use druid::widget::SvgData;
use druid::{Color, LocalizedString, Menu, MenuItem, Point, WidgetPod};

pub struct ToolbarWidget {
    buttons: Vec<WidgetPod<bool, ThemedButton>>,
    controls_outline: WidgetPod<bool, Svg>,
    controls_outline_dark: WidgetPod<bool, Svg>,
}

impl ToolbarWidget {
    pub fn new() -> Self {
        // Load button data
        let mut buttons = Vec::new();

        let zoom_button = WidgetPod::new(ThemedButton::new(
            Some(ZOOM_IMAGE),
            None,
            Size::new(32., 32.),
            Point::new(8. + 68. + 2. * 32. + 6. * 4., 16.),
            [
                include_str!("../resources/buttons/zoom/button.svg"),
                include_str!("../resources/buttons/zoom/hot.svg"),
                include_str!("../resources/buttons/zoom/active.svg"),
                include_str!("../resources/buttons/zoom/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/generic/small_button_mask").to_vec(),
        ));
        buttons.push(zoom_button);

        let recenter_button = WidgetPod::new(ThemedButton::new(
            Some(RECENTER_IMAGE),
            Some(REALSIZE_IMAGE),
            Size::new(32., 32.),
            Point::new(8. + 68. + 1. * 32. + 5. * 4., 16.),
            [
                include_str!("../resources/buttons/recenter/button.svg"),
                include_str!("../resources/buttons/recenter/hot.svg"),
                include_str!("../resources/buttons/recenter/active.svg"),
                include_str!("../resources/buttons/recenter/disabled.svg"),
                include_str!("../resources/buttons/realsize/button.svg"),
                include_str!("../resources/buttons/realsize/hot.svg"),
                include_str!("../resources/buttons/realsize/active.svg"),
                include_str!("../resources/buttons/realsize/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/generic/small_button_mask").to_vec(),
        ));
        buttons.push(recenter_button);

        let prev_button = WidgetPod::new(ThemedButton::new(
            Some(PREV_IMAGE),
            None,
            Size::new(68., 32.),
            Point::new(32. + 58., 16.),
            [
                include_str!("../resources/buttons/prev/button.svg"),
                include_str!("../resources/buttons/prev/hot.svg"),
                include_str!("../resources/buttons/prev/active.svg"),
                include_str!("../resources/buttons/prev/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/prev/mask").to_vec(),
        ));
        buttons.push(prev_button);

        let fullscreen_button = WidgetPod::new(ThemedButton::new(
            None,
            None,
            Size::new(64., 64.),
            Point::new(32., 32.),
            [
                include_str!("../resources/buttons/fullscreen/button.svg"),
                include_str!("../resources/buttons/fullscreen/hot.svg"),
                include_str!("../resources/buttons/fullscreen/active.svg"),
                include_str!("../resources/buttons/fullscreen/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/fullscreen/mask").to_vec(),
        ));
        buttons.push(fullscreen_button);

        let next_button = WidgetPod::new(ThemedButton::new(
            Some(NEXT_IMAGE),
            None,
            Size::new(68., 32.),
            Point::new(32. - 54., 16.),
            [
                include_str!("../resources/buttons/next/button.svg"),
                include_str!("../resources/buttons/next/hot.svg"),
                include_str!("../resources/buttons/next/active.svg"),
                include_str!("../resources/buttons/next/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/next/mask").to_vec(),
        ));
        buttons.push(next_button);

        let rot_l_button = WidgetPod::new(ThemedButton::new(
            Some(ROTATE_LEFT),
            None,
            Size::new(32., 32.),
            Point::new(8. - (68. + 1. * 32. + 1. * 4.), 16.),
            [
                include_str!("../resources/buttons/rot_l/button.svg"),
                include_str!("../resources/buttons/rot_l/hot.svg"),
                include_str!("../resources/buttons/rot_l/active.svg"),
                include_str!("../resources/buttons/rot_l/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/generic/small_button_mask").to_vec(),
        ));
        buttons.push(rot_l_button);

        let rot_r_button = WidgetPod::new(ThemedButton::new(
            Some(ROTATE_RIGHT),
            None,
            Size::new(32., 32.),
            Point::new(8. - (68. + 2. * 32. + 2. * 4.), 16.),
            [
                include_str!("../resources/buttons/rot_r/button.svg"),
                include_str!("../resources/buttons/rot_r/hot.svg"),
                include_str!("../resources/buttons/rot_r/active.svg"),
                include_str!("../resources/buttons/rot_r/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/generic/small_button_mask").to_vec(),
        ));
        buttons.push(rot_r_button);

        let delete_button = WidgetPod::new(ThemedButton::new(
            Some(DELETE_IMAGE),
            None,
            Size::new(32., 32.),
            Point::new(8. - (68. + 3. * 32. + 4. * 4.), 16.),
            [
                include_str!("../resources/buttons/del/button.svg"),
                include_str!("../resources/buttons/del/hot.svg"),
                include_str!("../resources/buttons/del/active.svg"),
                include_str!("../resources/buttons/del/disabled.svg"),
            ]
            .to_vec(),
            include_bytes!("../resources/buttons/generic/small_button_mask").to_vec(),
        ));
        buttons.push(delete_button);

        // Load control outline data
        let controls_outline = include_str!("../resources/buttons/outline.svg")
            .parse::<SvgData>()
            .unwrap();
        let controls_outline_dark = include_str!("../resources/buttons/outline_dark.svg")
            .parse::<SvgData>()
            .unwrap();

        Self {
            buttons,
            controls_outline: WidgetPod::new(Svg::new(controls_outline)),
            controls_outline_dark: WidgetPod::new(Svg::new(controls_outline_dark)),
        }
    }
}

impl Widget<AppState> for ToolbarWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        // Pass events to buttons
        for button in self.buttons.iter_mut() {
            button.event(ctx, event, &mut false, env);
        }
        if !ctx.is_handled() {
            if let Event::MouseDown(mouse_event) = event {
                if mouse_event.button.is_right() {
                    let context_menu = generate_menu(data);
                    ctx.show_context_menu(context_menu, mouse_event.pos)
                }
            }
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        // Pass all lifecycle events to buttons
        for button in self.buttons.iter_mut() {
            button.lifecycle(_ctx, _event, &false, _env);
        }

        // Pass only the widget added lifecycle event to outline widgets
        if let LifeCycle::WidgetAdded = _event {
            self.controls_outline.lifecycle(_ctx, _event, &false, _env);
            self.controls_outline_dark
                .lifecycle(_ctx, _event, &false, _env);

            if !_data.has_image() {
                for button in self.buttons.iter_mut() {
                    button.widget_mut().disable();
                }
            }
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, _env: &Env) {
        // Not efficient, but temporary until we find a way to not miss updates
        if data.has_image() && !data.has_image_error() {
            for button in self.buttons.iter_mut() {
                button.widget_mut().enable();
            }
            if data.get_image_list_size() == 1 {
                // Disable the next & previous buttons if there is only one image
                self.buttons[2].widget_mut().disable();
                self.buttons[4].widget_mut().disable();
            }
            // TODO: Fix this to work with the new state tracking
            // if _data.get_image_center_state() {
            //     self.buttons[1].widget_mut().disable();
            // }
            // let display_state = data.get_display_state();
            match data.get_display_state() {
                DisplayState::Centered(_) => self.buttons[1].widget_mut().set_command_index(1),
                DisplayState::RealSize(_) => self.buttons[1].widget_mut().set_command_index(0),
                DisplayState::Zoomed(_) => self.buttons[1].widget_mut().set_command_index(0),
            }
        } else {
            for button in self.buttons.iter_mut() {
                button.widget_mut().disable();
            }
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let widget_center = Point::new(bc.max().width / 2., bc.max().height / 2.);

        // Set control outline widget position
        self.controls_outline
            .layout(_layout_ctx, &bc.loosen(), &false, _env);
        self.controls_outline_dark
            .layout(_layout_ctx, &bc.loosen(), &false, _env);
        let controls_outline_origin =
            Point::new(widget_center.x - 382.733 / 2. + 18., widget_center.y - 32.);
        self.controls_outline
            .set_origin(_layout_ctx, &false, _env, controls_outline_origin);
        self.controls_outline_dark
            .set_origin(_layout_ctx, &false, _env, controls_outline_origin);

        // Set button widget positions
        for button in self.buttons.iter_mut() {
            button.layout(_layout_ctx, &bc.loosen(), &false, _env);
            let offset = button.widget().get_offset();
            let origin = Point::new(widget_center.x - offset.x, widget_center.y - offset.y);
            button.set_origin(_layout_ctx, &false, _env, origin);
        }

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let container_rect = ctx.size().to_rect();

        // Paint the toolbar background
        if data.dark_theme_enabled {
            let fill_color = Color::rgba(0.2, 0.2, 0.2, 0.5);
            ctx.fill(container_rect, &fill_color);
            self.controls_outline_dark.paint(ctx, &false, env);
        } else {
            let fill_color = Color::rgba(1., 1., 1., 0.5);
            ctx.fill(container_rect, &fill_color);
            self.controls_outline.paint(ctx, &false, env);
        };

        // Paint the buttons
        for button in self.buttons.iter_mut() {
            button.paint(ctx, &false, env);
        }
    }
}

fn generate_menu(data: &AppState) -> Menu<AppState> {
    const ABOUT_STR: &str = concat!("Foxfire v", env!("CARGO_PKG_VERSION"));
    let blur_state = data.blur_enabled();
    let filtering_state = data.image_filtering_enabled();
    Menu::empty()
        .entry(MenuItem::new(LocalizedString::new(ABOUT_STR)).enabled(false))
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Enable blur effects"))
                .selected(blur_state)
                .command(TOGGLE_BLUR)
                .on_activate(|_ctx, data: &mut AppState, _env| data.blur_enable_toggle()),
        )
        .entry(
            MenuItem::new(LocalizedString::new("Enable bilinear filtering"))
                .selected(filtering_state)
                .on_activate(|_ctx, data: &mut AppState, _env| {
                    data.image_filtering_enable_toggle()
                }),
        )
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Quit"))
                .on_activate(|_ctx, data: &mut AppState, _env| data.exit()),
        )
}
