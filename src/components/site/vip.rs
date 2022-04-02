use chrono::{DateTime, Utc};
use gloo_console::{debug, error};
use gloo_timers::future::sleep;
use nanoid::nanoid;
use obfstr::obfstr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use yew::{html, Component, Context, Html};

const API_ADDRESS: &str = "wss://metafashion-hq-api.onrender.com/ws";

impl Component for VIP {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Connect);
        Self {
            address: nanoid!(42),
            signup: false,
            check: false,
            signup_status: Status::None,
            entries: 0,
            peers: 0,
            status: Status::None,
            ws: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                debug!("vip: connect");
                // Ignore if already connected
                if self.ws.is_some() {
                    return false;
                }

                // Connect to api via websockets
                if let Err(e) = self.connect(
                    ctx,
                    API_ADDRESS,
                    obfstr!("9d5035b611b1dc600a762bf264a6ff19").to_owned(),
                ) {
                    error!(e);
                    ctx.link().send_message(Msg::Failed);
                }

                ctx.link().send_message(Msg::Connecting);
                false
            }
            Msg::Connecting => {
                debug!("vip: connecting...");
                self.status = Status::Working("> Connecting to MetaFashion HQ");
                //await delay(2000);
                true
            }
            Msg::Connected => {
                debug!("vip: connected");
                self.status = Status::Success("Connected");
                self.signup = true;
                self.check = true;
                self.signup_status = Status::None;
                true
            }
            Msg::Failed => {
                debug!("vip: failed");
                self.status = Status::Fail("Connection failed - please try again");
                self.signup = false;
                self.check = false;
                true
            }
            Msg::Disconnected => {
                debug!("vip: disconnected");
                self.ws = None;

                self.status = Status::Retrying("> Disconnected - retrying");
                self.signup = false;
                self.check = false;

                ctx.link().send_future(async move {
                    sleep(Duration::from_secs(30)).await;
                    Msg::Connect
                });
                true
            }
            Msg::MessageReceived(message) => match message {
                Message::SignedUp { address, total, .. } => {
                    self.entries = total;
                    if let Some(exists) = address {
                        if exists {
                            debug!("vip: signed-up");
                            self.signup_status = Status::Success(
                                "> Your address was successfully added to the VIP list",
                            );
                            self.signup = false;
                            self.check = false;
                        } else {
                            debug!("vip: not signed-up");
                            self.signup_status =
                                Status::Fail("> Your address was not found on the VIP list");
                            self.signup = true;
                            self.check = false;
                        }
                    }
                    true
                }
                Message::PeerJoined { total, .. } => {
                    self.peers = total;
                    true
                }
                Message::PeerLeft { total, .. } => {
                    self.peers = total;
                    true
                }
            },
            Msg::Signup => {
                debug!("vip: sign-up requested");
                self.signup = false;
                self.signup_status = Status::Working("> Signing up to the VIP list");

                let address = self.address.clone();
                let ws = self.ws.as_ref().unwrap().clone();
                ctx.link().send_future(async move {
                    // Pause for effect
                    sleep(Duration::from_secs(2)).await;

                    let request = serde_json::to_string(&Request::SignUp { address })
                        .expect("could not serialise the request");
                    if let Err(e) = ws.send_with_str(&request) {
                        error!(e);
                        return Msg::Failed;
                    }

                    Msg::SigningUp
                });
                true
            }
            Msg::SigningUp => false,
            Msg::Check => {
                debug!("vip: sign-up requested");
                self.check = false;
                self.signup_status = Status::Working("> Checking address signup status");

                let address = self.address.clone();
                let ws = self.ws.as_ref().unwrap().clone();
                ctx.link().send_future(async move {
                    // Pause for effect
                    sleep(Duration::from_secs(2)).await;

                    let request = serde_json::to_string(&Request::Check { address })
                        .expect("could not serialise the request");
                    if let Err(e) = ws.send_with_str(&request) {
                        error!(e);
                        return Msg::Failed;
                    }

                    Msg::Checking
                });

                true
            }
            Msg::Checking => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Statuses
        let connected = matches!(self.status, Status::Success(_));
        let (status, status_classes) = self.status.destructure();
        let (signup_status, signup_status_classes) = self.signup_status.destructure();

        // Button callbacks
        let signup = ctx.link().callback(|_| Msg::Signup);
        let check = ctx.link().callback(|_| Msg::Check);

        html! {
        <>
            <div class="modal-background"></div>
            <div class="modal-content">
                <div class="container">
                    <section class="section" >
                        <h1 class="title">{"VIP Signup"}</h1>
                        if connected {
                            <div id="vip-signup" class="container block has-text-centered">
                                <div class="block">
                                    <p>{"Welcome to the MetaFashion VIP signup"}</p>
                                </div>
                                <div class="block">
                                    <p>{"> Connect your wallet to add your address to the MetaFashion VIP list"}</p>
                                </div>
                                <div class="columns is-mobile">
                                    <div class="column has-text-centered">
                                        <button id="vip-add" onclick={signup} class="button vip" disabled={ !self.signup }>{"Signup"}</button>
                                    </div>
                                    <div class="column has-text-centered">
                                        <button id="vip-check" onclick={check} class="button vip" disabled={ !self.check }>{"Check"}</button>
                                    </div>
                                </div>
                                <div class="block">
                                    <span id="vip-signup-status" class={ signup_status_classes }>{ signup_status }</span>
                                </div>
                            </div>
                        }
                        <div class="block has-text-centered">
                            {"Status: "}<span id="vip-status" class={ status_classes }>{ status }</span>
                        </div>
                        <div class="columns is-mobile">
                            <div class="column has-text-centered">
                                <p>{"Entries: "}<span id="vip-total">{ self.entries }</span></p>
                            </div>
                            <div class="column has-text-centered">
                                <p>{"Peers: "}<span id="vip-peers">{ self.peers }</span></p>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
            <button class="modal-close is-large" aria-label="close"></button>
        </>
        }
    }
}

pub struct VIP {
    address: String,
    signup: bool,
    check: bool,
    signup_status: Status,
    entries: u64,
    peers: u64,
    status: Status,
    ws: Option<Arc<WebSocket>>,
}

pub enum Msg {
    Connect,
    Connecting,
    Connected,
    Failed,
    Disconnected,
    MessageReceived(Message),
    Signup,
    SigningUp,
    Check,
    Checking,
}

impl VIP {
    fn connect(
        &mut self,
        ctx: &Context<Self>,
        address: &str,
        api_key: String,
    ) -> Result<(), JsValue> {
        let ws = WebSocket::new(address)?;

        // Create message handler
        let handler = ctx.link().callback(|msg| msg);
        let on_message =
            Closure::wrap(Box::new(
                move |e: MessageEvent| match e.data().into_serde::<String>() {
                    Ok(data) => match serde_json::from_str::<Message>(&data) {
                        Ok(message) => {
                            handler.emit(Msg::MessageReceived(message));
                        }
                        Err(e) => {
                            error!(format!("unable to deserialise message: {:?} {:?}", e, data));
                            handler.emit(Msg::Failed);
                        }
                    },
                    Err(e) => {
                        error!(format!("unable to deserialise message: {:?}", e));
                        handler.emit(Msg::Failed);
                    }
                },
            ) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();

        // Create error handler
        let handler = ctx.link().callback(|msg| msg);
        let on_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            error!("error event: {:?}", e);
            handler.emit(Msg::Failed)
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();

        // Create open handler
        let handler = ctx.link().callback(|msg| msg);
        let ws_clone = ws.clone();
        let on_open = Closure::wrap(Box::new(move |_| {
            // Auth
            match ws_clone.send_with_str(&api_key) {
                Ok(_) => handler.emit(Msg::Connected),
                Err(err) => error!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();

        // Create close handler
        let handler = ctx.link().callback(|msg| msg);
        let on_close = Closure::wrap(Box::new(move |e| {
            error!("close event: {:?}", e);
            handler.emit(Msg::Disconnected)
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();

        self.ws = Some(Arc::new(ws));

        Ok(())
    }
}

enum Status {
    None,
    Working(&'static str),
    Success(&'static str),
    Fail(&'static str),
    Retrying(&'static str),
}

impl Status {
    fn destructure(&self) -> (&'static str, &'static str) {
        match self {
            Status::None => ("", ""),
            Status::Working(message) => (message, "blink"),
            Status::Success(message) => (message, "has-text-success"),
            Status::Fail(message) => (message, "has-text-danger"),
            Status::Retrying(message) => (message, "blink has-text-danger"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Request {
    #[serde(rename = "sign-up")]
    SignUp { address: String },
    #[serde(rename = "check")]
    Check { address: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "signed-up")]
    SignedUp {
        total: u64,
        address: Option<bool>,
        last_signed_up: Option<DateTime<Utc>>,
    },
    #[serde(rename = "peer-joined")]
    PeerJoined {
        total: u64,
        last_joined: Option<DateTime<Utc>>,
    },
    #[serde(rename = "peer-left")]
    PeerLeft {
        total: u64,
        last_left: Option<DateTime<Utc>>,
    },
}
