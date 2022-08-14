# tailwind-yew-Starter
Starter for yew( Frontend with rust ) + TailwindCSS (CSS Library)
# HOW TO START ?
1- CREATE THE YEW PROJECT .

2- THEN ADD THE TAILWINDCSS LIBRARY .

# 1- CREATE THE YEW PROJECT :
 
 ```
 rustup target add wasm32-unknown-unknown
```

 ```
cargo install trunk
```

 ```
cargo install trunk
```
```
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

# CREARE THE APP :-
```
cargo new yew-app
cd yew-app
```
 # TEST rUN :
 ```
 cargo run
 ```
# CREATE Cargo.toml :-
```
[package]
name = "yew-app"
version = "0.1.0"
edition = "2018"

[dependencies]
# you can check the latest version here: https://crates.io/crates/yew
yew = "0.19"  // CHECK THE LATEST ONE !!!!
```
# CREATE index.html:-

```
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

# REPLACE main.rs FILE :-

```

use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```
# SEE IF IT WORKS ?
```
trunk serve
```
___

-  SO IF EVERY THING IS OK THEN LET'S CONNECT THE TAILWINDCSS 

# 2- ADD TAILWINDCSS TO YEW

# INSTALL THE CLI :
```
npm install -D tailwindcss
npx tailwindcss init
```
# REPLACE tailwind.config.js :-
```
/** @type {import('tailwindcss').Config} */ 
module.exports = {
  content: ["./src/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```
# CREATE A FOLDER CALLED "STYLES" THEN ADD input.css FILE:
- PUT THIS INSIDE THE input.css FILE .
```
@tailwind base;
@tailwind components;
@tailwind utilities;
```

# RUN THIS COMMAND 
```
npx tailwindcss -i ./src/styles/input.css -o ./src/styles/output.css --watch
```

- then CSS FIlE is done 
- just a final step !!!
- hook the output.css file tho the index.html that we have created in the root (by using this link )!!
```
    <link data-trunk rel="css" href="src/styles/output.css"/>

```
we are done ! ☺️️

