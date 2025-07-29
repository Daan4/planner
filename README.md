### Env
```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install cargo-binstall
cargo binstall dioxus-cli
sudo apt-update
sudo apt-upgrade
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  zenity \
  sqlite3
echo "export DISPLAY=:0" > ~/.zshrc
cargo binstall diesel_cli
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt-get install -y nsolid
nsolid -v
npm install tailwindcss @tailwindcss/cli
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css
cargo add dioxus-primitives --git https://github.com/Dioxusabs/components
```

### DB
```bash
echo DATABASE_URL=/path/to/your/sqlite/database.db > .env
diesel setup
diesel migration generate --diff-schema initial
diesel migration run
diesel migration redo
```

### App
```bash
dx build --platform web
dx serve --platform web
```

### CSS
```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css
```