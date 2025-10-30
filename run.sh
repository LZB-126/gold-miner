#!/bin/bash

# 检查Rust是否安装
if ! command -v cargo &> /dev/null; then
    echo "Rust未安装，正在安装..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 检查是否安装了必要的系统依赖
if [ "$(uname)" = "Linux" ]; then
    echo "正在安装系统依赖..."
    sudo apt-get update
    sudo apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev
fi

# 构建并运行游戏
echo "构建并运行黄金矿工游戏..."
cargo run
