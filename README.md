# yew-bulma-components

## Introduction

clone project and Create a new project.

```bash
git clone https://github.com/doberan/yew-bulma-components.git
cargo new --bin sample-yew-homepage
cd sample-yew-homepage
touch index.html
```

edit Cargo.toml.

```toml
[package]
authors = []
edition = "2018"
name = "sample_yew_homepage"
version = "0.1.0"
[dependencies]
wasm-bindgen = "0.2.74"
yew = "0.18.0"
yew_bulma_components = "0.1.0"
```

edit index.html.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Sample Yew Project.</title>
    <base data-trunk-public-url />
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css" />
  </head>
</html>
```

edit main.rs

```rust
extern crate yew_bulma_components;
use yew::prelude::*;
use yew_bulma_components::components::atoms::h1;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <h1::Text classes="title" title="Sample homepage" />
        }
    }
}

fn main() {
    yew::start_app::<Home>();
}

```

execute serve.

```bash
trunk serve
```
