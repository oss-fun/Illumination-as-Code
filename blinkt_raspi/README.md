# ラズパイ 3/4 向けクロスコンパイル方法

- パッケージをインストール
  `sudo apt install gcc-arm-linux-gnueabihf`

- Cargo.toml に色々記述

- ビルド
  `cargo build --target armv7-unknown-linux-gnueabihf --release`

`scp -i ~/.ssh/tfdemo/tfdemo_ed25519 ./target/armv7-unknown-linux-gnueabihf/release/blinkt_raspi pi@10.6.18.124`
