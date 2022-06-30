# MetaFashion

NFT project site using [Yew](https://yew.rs). Uses websockets for VIP list wallet address registration.

### Initial Setup
Install npm dependencies (animate, bulma, bulma-collapsible and bulma-timeline):

    npm install

Run setup script, which processes the npm packages into the assets folder (via esbuild):

    npm run setup

### Run
Install [trunk](https://trunkrs.dev) via cargo, ensuring that [~HOME/.cargo/bin is in your $PATH](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html):

    cargo install trunk

Run using `trunk serve`:

    trunk serve
