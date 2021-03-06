use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::{Color, Cursor, LocalizedString, Menu, MenuItem, Rect};
use image::EncodableLayout;

use crate::app_state::*;
use crate::events::*;
use crate::types::*;

pub struct ImageWidget {}

impl Widget<AppState> for ImageWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        let anchor = _data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        if let Event::Wheel(mouse_event) = _event {
            if image_container.event_queue.is_none() {
                let mouse_position =
                    Vec2D::from(mouse_event.window_pos.x, mouse_event.window_pos.y);
                image_container.event_queue = Some(MouseEvent::Zoom(ZoomEvent::new(
                    mouse_event.wheel_delta.y,
                    mouse_position,
                )));
                _data.image_centered = false;
            }
            _ctx.request_paint();
        } else if let Event::MouseDown(mouse_event) = _event {
            if image_container.event_queue.is_none() {
                let mouse_pos = Vec2D::from(mouse_event.window_pos.x, mouse_event.window_pos.y);
                if mouse_event.button.is_left() {
                    let new_drag_event = DragEvent::new(mouse_pos, false);
                    image_container.event_queue = Some(MouseEvent::Drag(new_drag_event));
                    // _ctx.set_cursor(&Cursor::Crosshair);
                    _data.image_centered = false;
                } else if mouse_event.button.is_right() {
                    _ctx.show_context_menu(generate_menu(), mouse_event.pos)
                }
            }
            _ctx.request_paint();
        } else if let Event::MouseMove(mouse_event) = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut image_container.event_queue {
                if !drag_event.is_finished() {
                    let current_pos =
                        Vec2D::from(mouse_event.window_pos.x, mouse_event.window_pos.y);
                    drag_event.set_delta(current_pos);
                    _ctx.request_paint();
                }
            }
        } else if let Event::MouseUp(_mouse_event) = _event {
            if let Some(active_event) = &mut image_container.event_queue {
                if let MouseEvent::Drag(drag_event) = active_event {
                    drag_event.complete();
                }
                _ctx.request_paint();
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
        if let LifeCycle::WidgetAdded = _event {
            let mut new_title = "Foxfire - ".to_string();
            new_title.push_str(&*_data.get_image_name());
            _ctx.window().set_title(&new_title);
            let anchor = _data.get_image_ref();
            let mut image_container = anchor.lock().unwrap();
            let size = image_container.get_size();
            let toolbar_height = _data.get_toolbar_height();
            let image_aspect_ratio = size.width / size.height;
            if (size.width < 800. && size.height < 800.)
                && (size.width > 320. && size.height > 240.)
            {
                let window_size = Size::new(size.width, size.height + toolbar_height);
                _ctx.window().set_size(window_size);
            } else if image_aspect_ratio > 0.5 && image_aspect_ratio < 3. {
                let match_aspect_ratio: Size =
                    Size::new(640., (640. / image_aspect_ratio) + toolbar_height);
                _ctx.window().set_size(match_aspect_ratio);
            }

            image_container.center_image(_ctx.window().get_size(), toolbar_height);
        } else if let LifeCycle::FocusChanged(false) | LifeCycle::HotChanged(false) = _event {
            let anchor = _data.get_image_ref();
            let mut image_container = anchor.lock().unwrap();
            let mut event_queue = &mut image_container.event_queue;
            if let Some(MouseEvent::Drag(drag_event)) = &mut event_queue {
                drag_event.complete();
                _ctx.request_paint();
            }
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        let anchor = _data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        if image_container.event_queue.is_some() {
            if let Some(MouseEvent::Drag(drag_event)) = &mut image_container.event_queue {
                if drag_event.is_finished() {
                    _ctx.set_cursor(&Cursor::Arrow);
                } else if drag_event.is_new() {
                    drag_event.mark_seen();
                    _ctx.set_cursor(&Cursor::Crosshair);
                }
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
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let container_size = ctx.size();
        let container_rect = container_size.to_rect();

        if data.dark_theme_enabled {
            ctx.fill(container_rect, &Color::BLACK);
        } else {
            ctx.fill(container_rect, &Color::WHITE);
        }

        let anchor = data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        let image_size = image_container.get_size();
        if !image_container.has_cache() {
            let image_rgba = image_container.get_image().clone().into_rgba8();
            let image_result = ctx.make_image(
                image_size.width as usize,
                image_size.height as usize,
                image_rgba.as_bytes(),
                ImageFormat::RgbaPremul,
            );
            image_container.set_cache(image_result.unwrap());
        }

        let mut image_transform = image_container.transform;

        const IMAGE_ORIGIN_NAMESPACE: Vec2D<f64> = Vec2D { x: 0.0, y: 0.0 };
        let image_corner_imagespace =
            IMAGE_ORIGIN_NAMESPACE + Vec2D::from(image_size.width, image_size.height);
        let mut drag_offset_screenspace = image_transform.screen_space_offset;
        if let Some(MouseEvent::Drag(drag_event)) = &image_container.event_queue {
            drag_offset_screenspace.x += drag_event.get_delta().x;
            drag_offset_screenspace.y += drag_event.get_delta().y;
            if drag_event.is_finished() {
                image_transform.screen_space_offset = drag_offset_screenspace;
                image_container.event_queue = None;
            }
        } else if let Some(MouseEvent::Zoom(zoom_event)) = &image_container.event_queue {
            let zoom_factor = -zoom_event.get_magnitude() / 500.;
            let cursor_position = zoom_event.get_position();
            let cursor_vec = Vec2D::from(cursor_position.x, cursor_position.y);
            let zoom_target_prescale = image_transform.affine_matrix.inverse()
                * (cursor_vec - drag_offset_screenspace)
                + image_transform.image_space_offset;

            image_transform.affine_matrix.scale(1. + zoom_factor);

            let zoom_target_postscale = image_transform.affine_matrix.inverse()
                * (cursor_vec - drag_offset_screenspace)
                + image_transform.image_space_offset;
            drag_offset_screenspace = drag_offset_screenspace
                + image_transform.affine_matrix * (zoom_target_postscale - zoom_target_prescale);
            image_transform.screen_space_offset = drag_offset_screenspace;

            image_container.event_queue = None;
        } else if let Some(MouseEvent::Click(_)) = &image_container.event_queue {
            image_container.event_queue = None;
        }

        let image_origin_screenspace = image_transform.affine_matrix
            * (IMAGE_ORIGIN_NAMESPACE - image_transform.image_space_offset)
            + drag_offset_screenspace;
        let image_corner_screenspace = image_transform.affine_matrix
            * (image_corner_imagespace - image_transform.image_space_offset)
            + drag_offset_screenspace;

        let image_viewport = Rect::new(
            IMAGE_ORIGIN_NAMESPACE.x,
            IMAGE_ORIGIN_NAMESPACE.y,
            image_corner_imagespace.x,
            image_corner_imagespace.y,
        );

        let container_viewport = Rect::new(
            image_origin_screenspace.x,
            image_origin_screenspace.y,
            image_corner_screenspace.x,
            image_corner_screenspace.y,
        );
        image_container.transform = image_transform;
        ctx.draw_image_area(
            image_container.get_cache(),
            image_viewport,
            container_viewport,
            InterpolationMode::NearestNeighbor,
        );
    }
}

fn generate_menu() -> Menu<AppState> {
    Menu::empty()
        .entry(MenuItem::new(LocalizedString::new("Open with...")).enabled(false))
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Set as desktop background"))
                .on_activate(|_ctx, data: &mut AppState, _env| data.set_as_wallpaper()),
        )
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Open file location"))
                .on_activate(|_ctx, data: &mut AppState, _env| data.open_folder()),
        )
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Rotate left")).on_activate(
                |_ctx, data: &mut AppState, _env| data.rotate_in_memory(Direction::Left),
            ),
        )
        .entry(
            MenuItem::new(LocalizedString::new("Rotate right")).on_activate(
                |_ctx, data: &mut AppState, _env| data.rotate_in_memory(Direction::Right),
            ),
        )
        .separator()
        .entry(
            MenuItem::new(LocalizedString::new("Copy"))
                .on_activate(|_ctx, data: &mut AppState, _env| data.copy_image_to_clipboard()),
        )
        .entry(MenuItem::new(LocalizedString::new("Delete")).enabled(false))
        .separator()
        .entry(MenuItem::new(LocalizedString::new("Properties")).enabled(false))
}
