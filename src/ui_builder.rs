use druid::Widget;

use crate::app_state::*;
use crate::container::*;

pub fn build_ui() -> impl Widget<AppState> {
    ContainerWidget::new()
}
