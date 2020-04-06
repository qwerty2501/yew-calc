use crate::evaluator::*;
use crate::result::*;
use std::fmt;
use yew::prelude::*;

pub struct App {
    display: String,
    result: Result<Option<String>>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq)]
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

#[derive(PartialEq)]
pub enum Msg {
    PushButton(ButtonValue),
    ModifiedDisplay(String),
}

impl Component for App {
    type Properties = ();
    type Message = Msg;
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            display: String::default(),
            result: Ok(None),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PushButton(b) => match b {
                ButtonValue::Number(_)
                | ButtonValue::Plus
                | ButtonValue::Minus
                | ButtonValue::Division
                | ButtonValue::Multiplication
                | ButtonValue::Dot
                | ButtonValue::Percent => {
                    self.display.push_str(&b.to_string());
                    self.result = Ok(None);
                }
                ButtonValue::Redo => {
                    self.display.pop();
                    self.result = Ok(None);
                }
                ButtonValue::Clear => {
                    self.display = String::default();
                    self.result = Ok(None);
                }
                ButtonValue::Equal => {
                    self.calculate();
                }
            },
            Msg::ModifiedDisplay(v) => {
                self.display = v.clone();
                self.result = Ok(None);
            }
        };
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="calc calc--device">
                <div class="calc__display--normal">
                    <form  onsubmit=self.link.callback(|e:Event|{
                        e.prevent_default();
                        Msg::PushButton(ButtonValue::Equal)
                    })>
                        <input type="text" maxlength="20" class="calc__display-input calc__display-input--normal"
                            value=if let Ok(Some(new_display)) = &self.result{
                                new_display.clone()
                            } else{
                                self.display.clone()
                            } oninput=self.link.callback(|e:InputData|Msg::ModifiedDisplay(e.value))/>
                    </form>
                    {
                        match &self.result{
                            Err(err) => html!{
                                <p class="calc__display-sub calc__display-sub--error" >{err}</p>
                            },
                            _ => html!{}
                        }
                    }
                </div>
                <div class="calc__buttons--normal">
                    <div class="pure-g calc__buttons-group--normal">
                        { self.render_button(&ButtonValue::Clear,4)}
                        { self.render_button(&ButtonValue::Division,4)}
                        { self.render_button(&ButtonValue::Multiplication,4)}
                        <div class="pure-u-1-4 calc__buttons-unit calc__buttons-unit--normal">
                            <button class="calc__button calc__button--normal" type="button" >{" "}</button>
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
            <div class={format!("{}{} calc__buttons-unit calc__buttons-unit--normal","pure-u-1-", len)}>
                <button class="calc__button calc__button--normal" type="button"  onclick=self.link.callback(move |_|Msg::PushButton(onclick_value.clone()))>{v}</button>
            </div>
        }
    }
    fn calculate(&mut self) {
        self.result = evaluate(&self.display);
        self.display = String::default();
    }
}
