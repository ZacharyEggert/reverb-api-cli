#!/usr/bin/env bash
set -euo pipefail

# Install Homebrew if on macOS and not already installed
if [[ "$OSTYPE" == "darwin"* ]] && ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Installing..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# Install python3 if not already installed
if ! command -v python3 &> /dev/null; then
    echo "Python3 not found. Installing..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt update
        sudo apt install -y python3 python3-pip
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install python3
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi
else
    echo "Python3 is already installed."
fi

# Install the cargo rust package manager if not already installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Cargo is already installed."
fi

# Install the reverb-api-cli package
if ! command -v reverb-api-cli &> /dev/null; then
    echo "Installing reverb-api-cli..."
    cargo install reverb-api-cli@0.1.0-alpha.2
else
    echo "reverb-api-cli is already installed."
fi

# Install nodejs if not already installed
if ! command -v node &> /dev/null; then
    echo "Node.js not found. Installing..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        curl -fsSL https://deb.nodesource.com/setup_18.x | sudo bash -
        sudo apt install -y nodejs
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install node
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi
else
    echo "Node.js is already installed."
fi

npm i -g skills -y

# use npx skills to install all the skills from this repo
echo "Installing skills from this repo..."
npx skills add ZacharyEggert/reverb-api-cli --global --agent claude-code --skill reverb-listings -y
npx skills add ZacharyEggert/reverb-api-cli --global --agent claude-code --skill recipe-pricecheck -y
npx skills add ZacharyEggert/reverb-api-cli --global --agent claude-code --skill recipe-checkdeals -y



# Check auth status and prompt for API key if not authenticated
if ! revcli auth status 2>&1 | grep -q "Authenticated"; then
    echo "Reverb CLI is not authenticated. Running auth setup..."
    revcli auth set-key
fi

echo "Installation complete. You can now use the reverb-api-cli and the installed skills."