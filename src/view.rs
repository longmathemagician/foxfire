use druid::piet::Color;
use druid::Widget;
use druid::widget::{Align, Container, Label, Button, Padding, Split};

use crate::data::*;
use crate::image_widget::*;
use crate::container::*;
use crate::toolbar_widget::*;

pub fn build_ui() -> impl Widget<AppState> {
	ContainerWidget::new()
}