use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  /// class
  pub classes: String,
  ///inner text
  pub title: String,
}

pub struct H1Text {
  props: Props
}

impl Component for H1Text {
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
          <h1 class={ self.render_classes() }>{ self.render_title() }</h1>
        }
    }
}

impl H1Text {
  fn render_title(&self) -> &String {
    &self.props.title
  }

  fn render_classes(&self) -> &String {
    &self.props.classes
  }
}