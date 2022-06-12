use crate::app_state::*;
use crate::button_data::*;
use crate::button_widget::*;
use crate::image_container::*;
use crate::toolbar_data::*;
use druid::kurbo::BezPath;
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::Svg;
use druid::widget::SvgData;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use druid::{Data, WidgetPod};
use image::Rgb;
use std::sync::Arc;

// #[derive(Clone, Data)]
pub struct ToolbarWidget {
    fullscreen_button: WidgetPod<ThemedButtonState, ThemedButton>,
    next_button: WidgetPod<ThemedButtonState, ThemedButton>,
    prev_button: WidgetPod<ThemedButtonState, ThemedButton>,
    rotate_right_button: WidgetPod<ThemedButtonState, ThemedButton>,
    rotate_left_button: WidgetPod<ThemedButtonState, ThemedButton>,
    delete_button: WidgetPod<ThemedButtonState, ThemedButton>,
    recenter_button: WidgetPod<ThemedButtonState, ThemedButton>,
    zoom_button: WidgetPod<ThemedButtonState, ThemedButton>,
    controls_outline: WidgetPod<bool, Svg>,
    controls_outline_dark: WidgetPod<bool, Svg>,
}
impl ToolbarWidget {
    pub fn new() -> Self {
        let fullscreen =
            match include_str!("../resources/buttons/fullscreen.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let fullscreen_hot =
            match include_str!("../resources/buttons/fullscreen_hot.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let fullscreen_active =
            match include_str!("../resources/buttons/fullscreen_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };


        let fullscreen_mask_img = image::load_from_memory(include_bytes!("../resources/buttons/fullscreen_mask.bmp")).unwrap();
        let fullscreen_mask_raw_pixels = fullscreen_mask_img.as_rgb8().unwrap();
        let mut fullscreen_mask = vec![false; 64 * 64];
        for i in 0..64 {
            for j in 0..64 {
                match fullscreen_mask_raw_pixels[(i as u32, j as u32)] {
                    Rgb([0, 0, 0]) => fullscreen_mask[i * 63 + j] = false,
                    _ => fullscreen_mask[i * 63 + j] = true,
                }
            }
        }
        let fullscreen_mask_ref = Arc::new(fullscreen_mask);

        let next_mask_img = image::load_from_memory(include_bytes!("../resources/buttons/next_mask.bmp")).unwrap();
        let next_mask_raw_pixels = next_mask_img.as_rgb8().unwrap();
        let mut next_mask = vec![false; 68 * 32];
        for i in 0..32 {
            for j in 0..68 {
                match next_mask_raw_pixels[(j as u32, i as u32)] {
                    Rgb([0, 0, 0]) => next_mask[i * 67 + j] = false,
                    _ => next_mask[i * 67 + j] = true,
                }
            }
        }
        let next_mask_ref = Arc::new(next_mask);

        let prev_mask_img = image::load_from_memory(include_bytes!("../resources/buttons/prev_mask.bmp")).unwrap();
        let prev_mask_raw_pixels = prev_mask_img.as_rgb8().unwrap();
        let mut prev_mask = vec![false; 68 * 32];
        for i in 0..32 {
            for j in 0..68 {
                match prev_mask_raw_pixels[(j as u32, i as u32)] {
                    Rgb([0, 0, 0]) => prev_mask[i * 67 + j] = false,
                    _ => prev_mask[i * 67 + j] = true,
                }
            }
        }
        let prev_mask_ref = Arc::new(prev_mask);

        let small_mask_img = image::load_from_memory(include_bytes!("../resources/buttons/small_mask.bmp")).unwrap();
        let small_mask_raw_pixels = small_mask_img.as_rgb8().unwrap();
        let mut small_mask = vec![false; 32 * 32];
        for i in 0..32 {
            for j in 0..32 {
                match small_mask_raw_pixels[(i as u32, j as u32)] {
                    Rgb([0, 0, 0]) => small_mask[i * 31 + j] = false,
                    _ => small_mask[i * 31 + j] = true,
                }
            }
        }
        let small_mask_ref = Arc::new(small_mask);

        let next = match include_str!("../resources/buttons/next.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let next_hot = match include_str!("../resources/buttons/next_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let next_active =
            match include_str!("../resources/buttons/next_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };

        let prev = match include_str!("../resources/buttons/prev.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let prev_hot = match include_str!("../resources/buttons/prev_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let prev_active =
            match include_str!("../resources/buttons/prev_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let rot_r = match include_str!("../resources/buttons/rot_r.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let rot_r_hot = match include_str!("../resources/buttons/rot_r_hot.svg").parse::<SvgData>()
        {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let rot_r_active =
            match include_str!("../resources/buttons/rot_r_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let rot_l = match include_str!("../resources/buttons/rot_l.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let rot_l_hot = match include_str!("../resources/buttons/rot_l_hot.svg").parse::<SvgData>()
        {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let rot_l_active =
            match include_str!("../resources/buttons/rot_l_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let del = match include_str!("../resources/buttons/del.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let del_hot = match include_str!("../resources/buttons/del_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let del_active =
            match include_str!("../resources/buttons/del_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let recenter = match include_str!("../resources/buttons/recenter.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let recenter_hot =
            match include_str!("../resources/buttons/recenter_hot.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let recenter_active =
            match include_str!("../resources/buttons/recenter_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let zoom = match include_str!("../resources/buttons/zoom.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let zoom_hot = match include_str!("../resources/buttons/zoom_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let zoom_active =
            match include_str!("../resources/buttons/zoom_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let controls_outline =
            match include_str!("../resources/buttons/outline.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let controls_outline_dark =
            match include_str!("../resources/buttons/outline_dark.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        Self {
            fullscreen_button: WidgetPod::new(ThemedButton::new(
                Size::new(64., 64.),
                fullscreen,
                fullscreen_hot,
                fullscreen_active,
                Arc::clone(&fullscreen_mask_ref),
            )),
            next_button: WidgetPod::new(ThemedButton::new(
                Size::new(68., 32.),
                next,
                next_hot,
                next_active,
                Arc::clone(&next_mask_ref),
            )),
            prev_button: WidgetPod::new(ThemedButton::new(
                Size::new(68., 32.),
                prev,
                prev_hot,
                prev_active,
                Arc::clone(&prev_mask_ref),
            )),
            rotate_right_button: WidgetPod::new(ThemedButton::new(
                Size::new(32., 32.),
                rot_r,
                rot_r_hot,
                rot_r_active,
                Arc::clone(&small_mask_ref),
            )),
            rotate_left_button: WidgetPod::new(ThemedButton::new(
                Size::new(32., 32.),
                rot_l,
                rot_l_hot,
                rot_l_active,
                Arc::clone(&small_mask_ref),
            )),
            delete_button: WidgetPod::new(ThemedButton::new(
                Size::new(32., 32.),
                del,
                del_hot,
                del_active,
                Arc::clone(&small_mask_ref),
            )),
            recenter_button: WidgetPod::new(ThemedButton::new(
                Size::new(32., 32.),
                recenter,
                recenter_hot,
                recenter_active,
                Arc::clone(&small_mask_ref),
            )),
            zoom_button: WidgetPod::new(ThemedButton::new(
                Size::new(32., 32.),
                zoom,
                zoom_hot,
                zoom_active,
                Arc::clone(&small_mask_ref),
            )),
            controls_outline: WidgetPod::new(Svg::new(controls_outline)),
            controls_outline_dark: WidgetPod::new(Svg::new(controls_outline_dark)),
        }
    }
}

impl Widget<ToolbarState> for ToolbarWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut ToolbarState, _env: &Env) {
        self.fullscreen_button
            .event(_ctx, _event, &mut _data.fullscreen_button, _env);
        self.next_button
            .event(_ctx, _event, &mut _data.next_button, _env);
        self.prev_button
            .event(_ctx, _event, &mut _data.prev_button, _env);
        self.rotate_right_button
            .event(_ctx, _event, &mut _data.rotate_right_button, _env);
        self.rotate_left_button
            .event(_ctx, _event, &mut _data.rotate_left_button, _env);
        self.delete_button
            .event(_ctx, _event, &mut _data.delete_button, _env);
        self.recenter_button
            .event(_ctx, _event, &mut _data.recenter_button, _env);
        self.zoom_button
            .event(_ctx, _event, &mut _data.zoom_button, _env);

        // Single if branch because only one event should occur per event call
        if _data.next_button.has_event() {
            _data.next_button.clear_event();
            _data.set_right(true);
        } else if _data.prev_button.has_event() {
            _data.prev_button.clear_event();
            _data.set_left(true);
        } else if _data.recenter_button.has_event() {
            _data.recenter_button.clear_event();
            _data.set_recenter(true);
        } else if _data.rotate_left_button.has_event() {
            _data.rotate_left_button.clear_event();
            _data.set_rotate_left(true);
        } else if _data.rotate_right_button.has_event() {
            _data.rotate_right_button.clear_event();
            _data.set_rotate_right(true);
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &ToolbarState,
        _env: &Env,
    ) {
        self.fullscreen_button
            .lifecycle(_ctx, _event, &_data.fullscreen_button, _env);
        self.next_button
            .lifecycle(_ctx, _event, &_data.next_button, _env);
        self.prev_button
            .lifecycle(_ctx, _event, &_data.prev_button, _env);
        self.rotate_right_button
            .lifecycle(_ctx, _event, &_data.rotate_right_button, _env);
        self.rotate_left_button
            .lifecycle(_ctx, _event, &_data.rotate_left_button, _env);
        self.delete_button
            .lifecycle(_ctx, _event, &_data.delete_button, _env);
        self.recenter_button
            .lifecycle(_ctx, _event, &_data.recenter_button, _env);
        self.zoom_button
            .lifecycle(_ctx, _event, &_data.zoom_button, _env);
        if let LifeCycle::WidgetAdded = _event {
            self.controls_outline.lifecycle(_ctx, _event, &false, _env);
            self.controls_outline_dark
                .lifecycle(_ctx, _event, &false, _env);
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &ToolbarState,
        _data: &ToolbarState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ToolbarState,
        _env: &Env,
    ) -> Size {
        self.controls_outline
            .layout(_layout_ctx, &bc.loosen(), &false, _env);
        self.controls_outline_dark
            .layout(_layout_ctx, &bc.loosen(), &false, _env);
        let controls_outline_origin = Point::new(
            bc.max().width / 2. - 382.733 / 2. + 18.,
            bc.max().height / 2. - 32.,
        );
        self.controls_outline
            .set_origin(_layout_ctx, &false, _env, controls_outline_origin);
        self.controls_outline_dark
            .set_origin(_layout_ctx, &false, _env, controls_outline_origin);

        self.fullscreen_button
            .layout(_layout_ctx, &bc.loosen(), &_data.fullscreen_button, _env);
        let fullscreen_button_origin =
            Point::new(bc.max().width / 2. - 64. / 2., bc.max().height / 2. - 32.);
        self.fullscreen_button.set_origin(
            _layout_ctx,
            &_data.fullscreen_button,
            _env,
            fullscreen_button_origin,
        );

        self.next_button
            .layout(_layout_ctx, &bc.loosen(), &_data.next_button, _env);
        let next_button_origin = Point::new(
            bc.max().width / 2. - 64. / 2. + 54.,
            bc.max().height / 2. - 16.,
        );
        self.next_button
            .set_origin(_layout_ctx, &_data.next_button, _env, next_button_origin);

        self.prev_button
            .layout(_layout_ctx, &bc.loosen(), &_data.prev_button, _env);
        let prev_button_origin = Point::new(
            bc.max().width / 2. - 64. / 2. - 58.,
            bc.max().height / 2. - 16.,
        );
        self.prev_button
            .set_origin(_layout_ctx, &_data.prev_button, _env, prev_button_origin);

        self.rotate_right_button.layout(
            _layout_ctx,
            &bc.loosen(),
            &_data.rotate_right_button,
            _env,
        );
        let rotate_right_button_origin = Point::new(
            bc.max().width / 2. - 16. / 2. + 68. + 2. * 32. + 2. * 4.,
            bc.max().height / 2. - 16.,
        );
        self.rotate_right_button.set_origin(
            _layout_ctx,
            &_data.rotate_right_button,
            _env,
            rotate_right_button_origin,
        );

        self.rotate_left_button
            .layout(_layout_ctx, &bc.loosen(), &_data.rotate_left_button, _env);
        let rotate_left_button_origin = Point::new(
            bc.max().width / 2. - 16. / 2. + 68. + 1. * 32. + 1. * 4.,
            bc.max().height / 2. - 16.,
        );
        self.rotate_left_button.set_origin(
            _layout_ctx,
            &_data.rotate_left_button,
            _env,
            rotate_left_button_origin,
        );

        self.delete_button
            .layout(_layout_ctx, &bc.loosen(), &_data.delete_button, _env);
        let delete_button_origin = Point::new(
            bc.max().width / 2. - 16. / 2. + 68. + 3. * 32. + 4. * 4.,
            bc.max().height / 2. - 16.,
        );
        self.delete_button.set_origin(
            _layout_ctx,
            &_data.delete_button,
            _env,
            delete_button_origin,
        );

        self.recenter_button
            .layout(_layout_ctx, &bc.loosen(), &_data.recenter_button, _env);
        let recenter_button_origin = Point::new(
            bc.max().width / 2. - 16. / 2. - 68. - 1. * 32. - 5. * 4.,
            bc.max().height / 2. - 16.,
        );
        self.recenter_button.set_origin(
            _layout_ctx,
            &_data.recenter_button,
            _env,
            recenter_button_origin,
        );

        self.zoom_button
            .layout(_layout_ctx, &bc.loosen(), &_data.zoom_button, _env);
        let zoom_button_origin = Point::new(
            bc.max().width / 2. - 16. / 2. - 68. - 2. * 32. - 6. * 4.,
            bc.max().height / 2. - 16.,
        );
        self.zoom_button
            .set_origin(_layout_ctx, &_data.zoom_button, _env, zoom_button_origin);

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ToolbarState, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        if data.dark_theme_enabled {
            let fill_color = Color::rgba(0.2, 0.2, 0.2, 0.5);
            ctx.fill(rect, &fill_color);
            self.controls_outline_dark.paint(ctx, &false, env);
        } else {
            let fill_color = Color::rgba(1., 1., 1., 0.5);
            ctx.fill(rect, &fill_color);
            self.controls_outline.paint(ctx, &false, env);
        };

        self.fullscreen_button
            .paint(ctx, &data.fullscreen_button, env);
        self.next_button.paint(ctx, &data.next_button, env);
        self.prev_button.paint(ctx, &data.prev_button, env);
        self.rotate_right_button
            .paint(ctx, &data.rotate_right_button, env);
        self.rotate_left_button
            .paint(ctx, &data.rotate_left_button, env);
        self.delete_button.paint(ctx, &data.delete_button, env);
        self.recenter_button.paint(ctx, &data.recenter_button, env);
        self.zoom_button.paint(ctx, &data.zoom_button, env);
    }
}
