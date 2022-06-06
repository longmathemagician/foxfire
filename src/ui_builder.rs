use druid::piet::Color;
use druid::widget::{Align, Button, Container, Label, Padding, Split};
use druid::Widget;

use crate::app_state::*;
use crate::container::*;
use crate::image_widget::*;
use crate::toolbar_widget::*;

pub fn build_ui() -> impl Widget<AppState> {
    ContainerWidget::new()
}
