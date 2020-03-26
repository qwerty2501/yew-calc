use yew::prelude::*;

pub struct App {
    display: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    PushValue(char),
    ModifiedDisplay(String),
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PushValue(v) => {
                self.display.push(v);
                true
            }
            Msg::ModifiedDisplay(v) => {
                self.display = v;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="calc">
                <input type="text" class="calc__display--normal" value={self.display.clone()} oninput=self.link.callback(|e:InputData|Msg::ModifiedDisplay(e.value))/>
                <div class="calc__buttons--normal">
                    <div class="pure-g">
                        { self.render_button('C',4)}
                        { self.render_button('÷',4)}
                        { self.render_button('×',4)}
                        <div class="calc__button pure-u-1-4">
                        </div>
                    </div>
                    <div class="pure-g">
                        { self.render_buttons(&['7','8','9','↰'])}
                    </div>
                    <div class="pure-g">
                        { self.render_buttons(&['4','5','6','-'])}
                    </div>
                    <div class="pure-g">
                        { self.render_buttons(&['1','2','3','+'])}
                    </div>
                    <div class="pure-g">
                        { self.render_buttons(&['0','.','%','='])}
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn render_buttons(&self, values: &[char]) -> Html {
        html! {
            { for values.iter().map(|&v|{
                self.render_button(v,values.len())
            })}
        }
    }

    fn render_button(&self, v: char, len: usize) -> Html {
        html! {
            <div class={format!("{}{}","pure-u-1-", len)}>
                <button class="calc__button calc__button--normal" type="button"  onclick=self.link.callback(move |_|Msg::PushValue(v))>{v}</button>
            </div>
        }
    }
}
