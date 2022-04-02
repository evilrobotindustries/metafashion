# MetaFashion

### Initial Setup
Install npm dependencies (bulma, bulma-collapsible and bulma-timeline):

    npm install

Run setup script, which processes the npm packages into the assets folder:

    npm run setup

### Run
Install trunk via cargo, ensuring that [~HOME/.cargo/bin is in your $PATH](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html):

    cargo install trunk

Run using `trunk serve`:

    trunk serve
