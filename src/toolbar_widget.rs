use crate::data::*;
use crate::image_container::*;
use crate::toolbar_data::*;
use crate::button_widget::*;
use crate::button_data::*;
use druid::kurbo::BezPath;
use druid::{Data, WidgetPod};
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use std::sync::Arc;

// pub struct ContainerWidget {
//     image_widget: WidgetPod<AppState, ImageWidget>,
//     toolbar: WidgetPod<ToolbarState, ToolbarWidget>,
// }

// impl ContainerWidget {
//     pub fn new() -> Self {
//         Self {
//             image_widget: WidgetPod::new(ImageWidget {}),
//             toolbar: WidgetPod::new(ToolbarWidget {}),
//         }
//     }
// }

// #[derive(Clone, Data)]
pub struct ToolbarWidget {
    fullscreen_button: WidgetPod<ThemedButtonState, ThemedButton>
}
impl ToolbarWidget {
    pub fn new() -> Self {
        Self {
            fullscreen_button: WidgetPod::new(ThemedButton::new()),
        }
    }
}

impl Widget<ToolbarState> for ToolbarWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut ToolbarState, _env: &Env) {
        self.fullscreen_button.event(_ctx, _event, &mut _data.fullscreen_button, _env);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &ToolbarState, _env: &Env) {
        self.fullscreen_button.lifecycle(_ctx, _event, &_data.fullscreen_button, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &ToolbarState, _data: &ToolbarState, _env: &Env) {
        
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ToolbarState,
        _env: &Env,
    ) -> Size {
        self.fullscreen_button.layout(_layout_ctx, &bc.loosen(), &_data.fullscreen_button, _env);
        
        let button_origin = Point::new(bc.max().width/2. - 450. / 2., bc.max().height/2. - 32.);
        self.fullscreen_button.set_origin(_layout_ctx, &_data.fullscreen_button, _env, button_origin);

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
        let fill_color = Color::rgba(1., 1., 1., 0.8);
        ctx.fill(rect, &fill_color);

        self.fullscreen_button.paint(ctx, &data.fullscreen_button, env);
    }
}
