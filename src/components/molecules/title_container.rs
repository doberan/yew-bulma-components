use yew::prelude::*;
use yewtil::NeqAssign;
use crate::components::atoms::{
  H1Text,
  H2Text,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  /// class
  pub title_text: String,
  pub title_classes: String,
  pub subtitle_text: String,
  pub subtitle_classes: String,
  pub container_classes: String,
}

pub struct TitleContainer {
  props: Props
}

impl Component for TitleContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
          props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
          <div class={ format!("hero-body container pb-0 {}", self.props.container_classes.clone()) }>
            <H1Text classes={ format!("title {}", self.props.title_classes.clone()) } text={ self.props.title_text.clone() } />
            <H2Text classes={ format!("subtitle {}", self.props.subtitle_classes.clone()) } text={ self.props.subtitle_text.clone() } />
          </div>
        }
    }
}
