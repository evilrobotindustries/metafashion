use crate::components::site::wallet::Wallet;
use chrono::{DateTime, Utc};
use eth_wasm::Address;
use gloo_console::{debug, error};
use gloo_timers::future::sleep;
use num_format::{Locale, ToFormattedString};
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
            // Wallet
            wallet_address: None,
            // API
            api_key: obfstr!("9d5035b611b1dc600a762bf264a6ff19").to_owned(),
            api: None,
            api_connected: false,
            api_peers: 0,
            api_status: Status::None,
            // Signup
            signup_enabled: false,
            check_enabled: false,
            signup_status: Status::None,
            entries: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                debug!("vip: connect");
                // Ignore if already connected
                if self.api.is_some() {
                    return false;
                }

                // Connect to api via websockets
                if let Err(e) = self.connect(ctx, API_ADDRESS, self.api_key.clone()) {
                    error!(e);
                    ctx.link().send_message(Msg::Failed);
                }

                ctx.link().send_message(Msg::Connecting);
                false
            }
            Msg::Connecting => {
                debug!("vip: connecting...");
                self.api_status = Status::Working("Connecting to MetaFashion HQ");
                //await delay(2000);
                true
            }
            Msg::Connected => {
                debug!("vip: connected");
                self.api_connected = true;
                self.api_status = Status::Normal("Connected to MetaFashion HQ");
                self.signup_enabled = true;
                self.check_enabled = true;
                self.signup_status = Status::None;
                true
            }
            Msg::Failed => {
                debug!("vip: failed");
                self.api_status = Status::Fail("Connection failed - please try again".to_string());
                self.signup_enabled = false;
                self.check_enabled = false;
                true
            }
            Msg::Disconnected => {
                debug!("vip: disconnected");
                self.api = None;
                self.api_connected = false;
                self.api_status = Status::Retrying("Disconnected from MetaFashion HQ - retrying");
                self.api_peers = 0;

                self.signup_enabled = false;
                self.check_enabled = false;

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
                                "Your address was successfully added to the VIP list",
                            );
                            self.signup_enabled = false;
                            self.check_enabled = false;
                        } else {
                            debug!("vip: not signed-up");
                            self.signup_status = Status::Fail(
                                "Your address was not found on the VIP list".to_string(),
                            );
                            self.signup_enabled = true;
                            self.check_enabled = false;
                        }
                    }
                    true
                }
                Message::PeerJoined { total, .. } => {
                    self.api_peers = total;
                    true
                }
                Message::PeerLeft { total, .. } => {
                    self.api_peers = total;
                    true
                }
            },
            Msg::Signup => {
                debug!("vip: sign-up requested");

                if self.wallet_address.is_none() {
                    self.signup_status = Status::Fail("Wallet is not connected".to_string());
                    return true;
                }

                self.signup_enabled = false;
                self.signup_status = Status::Working("Signing up to the VIP list");

                let address = self.wallet_address.as_ref().unwrap().clone();
                let ws = self.api.as_ref().unwrap().clone();
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

                if self.wallet_address.is_none() {
                    self.signup_status = Status::Fail("Wallet is not connected".to_string());
                    return true;
                }

                self.check_enabled = false;
                self.signup_status = Status::Working("Checking address signup status");

                let address = self.wallet_address.as_ref().unwrap().clone();
                let ws = self.api.as_ref().unwrap().clone();
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
            Msg::WalletConnected(address) => {
                self.wallet_address = Some(address);
                self.signup_status = Status::None;

                if self.api_connected {
                    self.signup_enabled = true;
                    self.check_enabled = true;
                }
                true
            }
            Msg::WalletConnecting => {
                self.signup_status = Status::Working("Connecting...");
                true
            }
            Msg::WalletError(error) => {
                self.signup_status = Status::Fail(error);
                self.wallet_address = None;
                self.signup_enabled = false;
                self.check_enabled = false;
                true
            }
            Msg::WalletDisconnected => {
                self.signup_enabled = false;
                self.check_enabled = false;
                self.wallet_address = None;
                self.signup_status = Status::None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Statuses
        let (status, status_classes) = self.api_status.destructure();
        let (signup_status, signup_status_classes) = self.signup_status.destructure();

        // Callbacks
        let signup = ctx.link().callback(|_| Msg::Signup);
        let check = ctx.link().callback(|_| Msg::Check);
        let wallet_connected = ctx.link().callback(|address| Msg::WalletConnected(address));
        let wallet_connecting = ctx.link().callback(|_| Msg::WalletConnecting);
        let wallet_error = ctx.link().callback(|error| Msg::WalletError(error));
        let wallet_disconnected = ctx.link().callback(|_| Msg::WalletDisconnected);

        let address = self
            .wallet_address
            .as_ref()
            .map_or("".to_string(), |a| a.to_string());

        html! {
        <>
            <div class="modal-background"></div>
            <div class="modal-content">
                <div class="container">
                    <section class="section" >
                        <h1 class="title">{"VIP Signup"}</h1>
                        <div id="vip-signup" class="container block has-text-centered">
                            <div class="block">
                                <p>{"Welcome to the MetaFashion VIP signup"}</p>
                            </div>
                            if self.wallet_address.is_none() {
                                // Connect
                                <div class="block">
                                    <p>{"> Connect your wallet to sign up to the MetaFashion VIP list"}</p>
                                </div>
                                <div class="block">
                                    <Wallet on_connected={wallet_connected}
                                            on_connecting={wallet_connecting}
                                            on_error={wallet_error}
                                            on_disconnected={wallet_disconnected} />
                                </div>
                            }
                            else {
                                // Signup
                                <div class="block">
                                    <p>{"Address: "}<span id="vip-address">{ address }</span></p>
                                </div>
                                <div class="columns is-mobile">
                                    <div class="column has-text-centered">
                                        <button id="vip-add" onclick={signup} class="button vip" disabled={ !self.signup_enabled }>{"Sign up"}</button>
                                    </div>
                                    <div class="column has-text-centered">
                                        <button id="vip-check" onclick={check} class="button vip" disabled={ !self.check_enabled }>{"Check"}</button>
                                    </div>
                                </div>
                            }
                            <div class="block">
                                <span id="vip-signup-status" class={ signup_status_classes }>{ signup_status }</span>
                            </div>
                        </div>
                        <div class="columns is-mobile">
                            <div class="column has-text-centered">
                                <p class="vip-metric">{ self.entries.to_formatted_string(&Locale::en) }</p>
                                <p>{"Addresses"}</p>
                            </div>
                            <div class="column has-text-centered">
                                <p class="vip-metric">{ self.api_peers.to_formatted_string(&Locale::en) }</p>
                                <p>{"Peers"}</p>
                            </div>
                        </div>
                        <div class="block has-text-centered">
                            <span id="vip-status" class={ status_classes }>{ status }</span>
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
    // Wallet
    wallet_address: Option<Address>,
    // API
    api_key: String,
    api: Option<Arc<WebSocket>>,
    api_connected: bool,
    api_peers: u64,
    api_status: Status,
    // Signup
    signup_enabled: bool,
    check_enabled: bool,
    signup_status: Status,
    entries: u64,
}

pub enum Msg {
    // VIP Signup
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
    // Wallet
    WalletConnected(Address),
    WalletConnecting,
    WalletError(String),
    WalletDisconnected,
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

        self.api = Some(Arc::new(ws));

        Ok(())
    }
}

pub enum Status {
    None,
    Normal(&'static str),
    Working(&'static str),
    Success(&'static str),
    Fail(String),
    Retrying(&'static str),
}

impl Status {
    fn destructure(&self) -> (&str, &'static str) {
        match self {
            Status::None => ("", ""),
            Status::Normal(message) => (message, ""),
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
    SignUp { address: Address },
    #[serde(rename = "check")]
    Check { address: Address },
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
