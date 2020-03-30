use std::fmt;
use yew::prelude::*;

pub struct App {
    display: String,
    link: ComponentLink<Self>,
}

#[derive(Clone)]
pub enum ButtonValue {
    Number(u8),
    Plus,
    Minus,
    Division,
    Redo,
    Equal,
    Multiplication,
    Clear,
    Dot,
    Percent,
}

impl fmt::Display for ButtonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ButtonValue::Number(v) => write!(f, "{}", v),
            ButtonValue::Plus => write!(f, "{}", "+"),
            ButtonValue::Minus => write!(f, "{}", "-"),
            ButtonValue::Division => write!(f, "{}", "÷"),
            ButtonValue::Redo => write!(f, "{}", "↰"),
            ButtonValue::Equal => write!(f, "{}", "="),
            ButtonValue::Multiplication => write!(f, "{}", "×"),
            ButtonValue::Clear => write!(f, "{}", "C"),
            ButtonValue::Dot => write!(f, "{}", "."),
            ButtonValue::Percent => write!(f, "{}", "%"),
        }
    }
}

pub enum Msg {
    PushButton(ButtonValue),
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
            Msg::PushButton(v) => {
                self.display.push_str(&v.to_string());
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
            <div class="calc calc--device">
                <div class="calc__display--normal">
                    <input type="text" maxlength="20" class="calc__display-input calc__display-input--normal" value={self.display.clone()} oninput=self.link.callback(|e:InputData|Msg::ModifiedDisplay(e.value))/>
                </div>
                <div class="calc__buttons--normal calc__buttons--device">
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_button(&ButtonValue::Clear,4)}
                        { self.render_button(&ButtonValue::Division,4)}
                        { self.render_button(&ButtonValue::Multiplication,4)}
                        <div class="calc__button pure-u-1-4">
                        </div>
                    </div>
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_buttons(&[ButtonValue::Number(7),ButtonValue::Number(8),ButtonValue::Number(9),ButtonValue::Redo])}
                    </div>
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_buttons(&[ButtonValue::Number(4),ButtonValue::Number(5),ButtonValue::Number(6),ButtonValue::Minus])}
                    </div>
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_buttons(&[ButtonValue::Number(1),ButtonValue::Number(2),ButtonValue::Number(3),ButtonValue::Plus])}
                    </div>
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_buttons(&[ButtonValue::Number(0),ButtonValue::Dot,ButtonValue::Percent,ButtonValue::Equal])}
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn render_buttons(&self, values: &[ButtonValue]) -> Html {
        html! {
            { for values.iter().map(|v|{
                self.render_button(v,values.len())
            })}
        }
    }

    fn render_button(&self, v: &ButtonValue, len: usize) -> Html {
        let onclick_value = v.clone();
        html! {
            <div class={format!("{}{} {}","pure-u-1-", len,"calc__buttons-unit--normal")}>
                <button class="calc__button calc__button--normal" type="button"  onclick=self.link.callback(move |_|Msg::PushButton(onclick_value.clone()))>{v}</button>
            </div>
        }
    }
}
