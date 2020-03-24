use yew::prelude::*;

pub struct App {
    display: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    PushButton(&'static str),
}

impl Component for App {
    type Properties = ();
    type Message = Msg;
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            display: String::new(),
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div >
                <input type="text" value={self.display.clone()} />
                <div class="pure-g">
                    <div class="pure-u-1-3">
                        <button class=".calc__button" type="button" name="1" value="1" onclick=self.link.callback(|_|Msg::PushButton(""))>{1}</button>
                    </div>
                    <div class="pure-u-1-3"></div>
                    <div class="pure-u-1-3"></div>
                </div>
            </div>
        }
    }
}
