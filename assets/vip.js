const wsUri = "ws://127.0.0.1:8000/ws"
const api_key = "hnbh9WEllWzFzVviUClOGQ=="

const vipList = document.getElementById('vip-list');
const check = document.getElementById('vip-check');
const peers = document.getElementById('vip-peers');
const progress = document.getElementById('vip-progress');
const signup = document.getElementById('vip-signup');
const add = document.getElementById('vip-add');
const signupStatus = document.getElementById('vip-signup-status');
const status = document.getElementById('vip-status');
const total = document.getElementById('vip-total');
const userAddress = randomAddress(20);

let conn = null;
let connected = false;

vipList.addEventListener('click', async () => {
    await connect();
});

const connect = async () => {
    if (conn == null) {
        console.debug("api: connecting...");
        status.classList.remove('has-text-danger', 'has-text-success');
        status.classList.add('blink');
        status.innerText = "> Connecting to MetaFashion HQ";
        await delay(2000);

        conn = new WebSocket(wsUri);

        conn.onopen = async () => {
            connected = true;
            console.debug("api: connected");
            conn.send(api_key);
            console.debug("api: ready");

            status.classList.remove('blink', 'has-text-danger');
            status.classList.add('has-text-success');
            status.innerText = "Connected"

            // Show signup
            add.disabled = false;
            check.disabled = false;
            signup.classList.remove('is-hidden');
        };
        conn.onerror = async e => {
            status.classList.remove('blink');
            status.classList.add('has-text-danger');
            status.innerText = `Connection failed - please try again`;
        }
        conn.onmessage = e => {
            const message = JSON.parse(e.data);
            console.debug('api:', message.type)
            switch (message.type) {
                case 'peer-joined':
                case 'peer-left': {
                    peers.innerText = message.total
                    return
                }
                case 'registered': {
                    total.innerText = message.total
                    console.log(message.address)

                    if (message.address !== undefined && message.address != null) {
                        signupStatus.classList.remove('blink', 'has-text-success', 'has-text-danger');
                        if (message.address) {
                            signupStatus.classList.add('has-text-success');
                            signupStatus.innerText = '> Your address was successfully added to the VIP list';
                            add.disabled = true;
                            check.disabled = true;
                        } else {
                            signupStatus.classList.add('has-text-danger');
                            signupStatus.innerText = '> Your address was not found on the VIP list';
                        }
                    }

                    return
                }
            }
        };
        conn.onclose = async () => {
            if (!connected) return;

            console.log("api: disconnected");
            conn = null;

            add.disabled = true;
            check.disabled = true;

            status.classList.add('blink', 'has-text-danger');
            status.innerText = "> Disconnected - retrying"
            await delay(30_000)
            connect()
        };
    }
}

add.addEventListener('click', async () => {
    add.disabled = true;
    try {
        signupStatus.classList.remove('has-text-success', 'has-text-danger');
        signupStatus.classList.add('blink');
        signupStatus.innerText = "> Adding your address to the VIP list";
        await delay(2000);
        conn.send(JSON.stringify({type: "register", address: userAddress}));
    }
    catch (e) {
        add.disabled = false;
    }
});
check.addEventListener('click', async () => {
    check.disabled = true;
    try {
        signupStatus.classList.remove('has-text-success', 'has-text-danger');
        signupStatus.classList.add('blink');
        signupStatus.innerText = "> Checking address signup status";
        await delay(2000);
        conn.send(JSON.stringify({type: "check", address: userAddress}));
    }
    catch (e){
        check.disabled = false;
    }
});

connect();

function delay(ms){
    return new Promise(resolve => {
        setTimeout(resolve,ms);
    })
}

function randomAddress(length) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    for ( let i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;
}