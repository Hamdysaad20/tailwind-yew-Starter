use yew::prelude::*;

enum Msg {
    AddOne,
    REOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 12,
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

            Msg::REOne => {
                self.value -= 1;
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
                <button class="h-12 w-12 " onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <h1 class="bg-red-400 text-2xl p-12 m-10 "> { self.value }</h1>
                            <button class="h-12 w-12 " onclick={link.callback(|_| Msg::REOne)}>{ "-1" }</button>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
