#!/usr/bin/env bash

# https://stackoverflow.com/a/2924755
norm=$(tput sgr0)
bold=$(tput bold)
# https://askubuntu.com/a/1167054
ul=$(tput smul)
# https://stackoverflow.com/a/5947802
blue="\033[0;34m"

echo "1. Install Xcode command-line tools."
echo -e "   ${blue}${bold}xcode-select --install${norm}"
echo

echo "2. Install Homebrew from ${ul}https://brew.sh${norm}."
echo -e "   ${blue}${bold}/bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"${norm}"
echo

echo "3. Update Homebrew."
echo -e "   ${blue}${bold}brew analytics off && brew update && brew upgrade${norm}"
echo

echo "4. Install \`fish\`."
echo -e "   ${blue}${bold}brew install fish${norm}"
echo

echo "5. Set \`fish\` as default shell (adjust path to it below as necessary)."
echo -e "   ${blue}${bold}sudo bash -c \"echo /opt/homebrew/bin/fish >> /etc/shells\"${norm}"
echo -e "   ${blue}${bold}chsh -s /opt/homebrew/bin/fish${norm}"
echo

echo "6. Continue steps below in \`fish\` shell."
echo

echo "7. Enable \`fish\` vi keybindings."
echo -e "   ${blue}${bold}fish_vi_key_bindings${norm}"
echo

echo "8. Install \`fisher\` and \`fish-pure\`."
echo -e "   ${blue}${bold}curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher install jorgebucaran/fisher${norm}"
echo -e "   ${blue}${bold}fisher install pure-fish/pure${norm}"
echo

echo "9. Add Homebrew dir to \$PATH."
echo -e "   ${blue}${bold}fish_add_path /opt/homebrew/bin${norm}"
echo

echo "10. Install Rust toolchain from ${ul}https://www.rust-lang.org/tools/install${norm}."
echo -e "   ${blue}${bold}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${norm}"
echo

echo "11. Add \`cargo\` dir to \$PATH."
echo -e "   ${blue}${bold}fish_add_path \$HOME/.cargo/bin${norm}"
echo

echo "12. Install and run the cli app to install the bundle."
echo -e "   ${blue}${bold}cargo install --path drip${norm}"
echo -e "   ${blue}${bold}drip bundle -d recipe/${norm}"
echo
