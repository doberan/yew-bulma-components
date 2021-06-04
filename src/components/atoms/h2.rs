use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  /// class
  pub classes: String,
  pub text: String,
}

pub struct Text {
  props: Props
}

impl Component for Text {
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
          <h2 class={ self.props.classes.clone() }>{ self.props.text.clone() }</h2>
        }
    }
}
