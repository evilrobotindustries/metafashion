use eth_wasm::{Address, Chain, Provider};
use gloo_console::{debug, error};
use std::sync::Arc;
use yew::{html, Callback, Component, Context, Html, Properties};

// Simple wrapper around Ethereum provider
pub struct Wallet {
    provider: Option<Arc<Provider>>,
}

// Callbacks to notify connected wallet
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_connected: Callback<Address>,
    pub on_connecting: Callback<()>,
    pub on_error: Callback<String>,
    pub on_disconnected: Callback<()>,
}

pub enum Msg {
    // Requests
    RequestAccounts,
    RequestingAccounts,
    // Events
    Connected(Chain),
    AccountsChanged(Vec<Address>),
    Failed(String),
    Disconnected,
}

impl Component for Wallet {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let provider = eth_wasm::Provider::new().map(|provider| {
            // Subscribe to provider events
            let connected = ctx.link().callback(|msg| msg);
            provider.on_connect(move |chain| {
                debug!(format!("wallet: connected {}", chain));
                connected.emit(Msg::Connected(chain))
            });

            provider.on_chain_changed(move |chain| {
                debug!(format!("wallet: chain changed to {}", chain))
            });

            let accounts_changed = ctx.link().callback(|msg| msg);
            let p = ctx.props().clone();
            provider.on_accounts_changed(move |accounts| {
                debug!(format!("wallet: accounts changed {:?}", accounts));

                if !accounts.is_empty() {
                    p.on_connected.emit(accounts.get(0).unwrap().clone())
                } else {
                    p.on_disconnected.emit(());
                }

                accounts_changed.emit(Msg::AccountsChanged(accounts))
            });

            let disconnected = ctx.link().callback(|msg| msg);
            provider.on_disconnect(move |_error| disconnected.emit(Msg::Disconnected));

            Arc::new(provider)
        });

        if provider.is_none() {
            ctx.props()
                .on_error
                .emit("No wallet provider available".to_string());
        }

        Self { provider }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let scope = ctx.link();
        match msg {
            // Requests
            Msg::RequestAccounts => {
                debug!("wallet: connect");
                // Request accounts, prompting user for permission to access accounts and balances
                let provider = self.provider.as_ref().unwrap().clone();
                ctx.link().send_future(async move {
                    match provider.request_accounts().await {
                        Ok(accounts) => Msg::AccountsChanged(accounts),
                        Err(e) => Msg::Failed(format!("{}", e)),
                    }
                });

                scope.send_message(Msg::RequestingAccounts);
                false
            }
            Msg::RequestingAccounts => {
                debug!("wallet: connecting...");
                ctx.props().on_connecting.emit(());
                false
            }
            // Events
            Msg::Connected(_) => true,
            Msg::AccountsChanged(accounts) => {
                debug!("wallet: accounts changed");
                if accounts.is_empty() {
                    ctx.props().on_disconnected.emit(());
                } else {
                    for account in accounts {
                        // Invoke callback to notify address changed
                        ctx.props().on_connected.emit(account);
                        return false;
                    }
                }
                false
            }
            Msg::Disconnected => {
                debug!("wallet: disconnected");
                ctx.props().on_disconnected.emit(());
                false
            }
            Msg::Failed(error) => {
                debug!("wallet: error");
                error!(error.clone());
                ctx.props().on_error.emit(error);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::RequestAccounts);
        html! {
            <>
                if self.provider.is_some() {
                    <button {onclick} class="button vip">{"Connect"}</button>
                }
                else {
                    <p class="has-text-danger">{"No wallet provider available"}</p>
                }
            </>
        }
    }
}
