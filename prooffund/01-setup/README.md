# 01 · Set up your machine

Slides 6 and 7. Four steps, taken from the [official Stellar setup guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup). Every code box has a copy button at its top right.

Do this at home on good internet if you can. You need about 5 GB of free disk space.

| Step | Mac or Linux | Windows |
|---|---|---|
| [1. Install Rust](#1-install-rust) | One command | Run `rustup-init.exe` |
| [2. Add the Wasm target](#2-add-the-wasm-target) | Same on both | Same on both |
| [3. Install the Stellar CLI](#3-install-the-stellar-cli) | `brew` or a script | `winget` |
| [4. Check it worked](#4-check-it-worked) | Same on both | Same on both |

Nothing to install? [Use Codespaces](#no-install-use-codespaces) instead.

## 1. Install Rust

**Mac or Linux.** Run this, then close and reopen your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows.** Download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe), then press Enter to accept the defaults. Use [Windows Terminal](https://learn.microsoft.com/en-us/windows/terminal/install) from here on, otherwise the CLI's emojis show as question marks.

You need Rust 1.84 or newer. If yours is older, update it:

```bash
rustup update stable
```

## 2. Add the Wasm target

Contracts compile to WebAssembly. This adds the target they need:

```bash
rustup target add wasm32v1-none
```

If you update Rust later, run this again.

## 3. Install the Stellar CLI

**Mac:**

```bash
brew install stellar-cli
```

**Linux:**

```bash
curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh
```

If Linux says `libdbus-1.so.3` is missing, run this and try again:

```bash
sudo apt-get install -y libdbus-1-3 libudev1
```

**Windows:**

```bash
winget install --id Stellar.StellarCLI
```

No winget? Download the installer from the [latest release](https://github.com/stellar/stellar-cli/releases/latest) and double-click it.

Restart your terminal afterwards.

## 4. Check it worked

```bash
rustc --version
```

```bash
stellar --version
```

Both should print a version number. You're ready.

## Your editor and wallet

- **Editor:** [VS Code](https://code.visualstudio.com) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.
- **Wallet:** add [Varial](https://varialwallet.com) to Chrome, Firefox or Brave. It used to be called Freighter, and older tutorials still use that name.

## Optional: save time on the day

The first build downloads a lot. Do it at home, so it's quick in the room:

```bash
git clone https://github.com/boundlessfi/build-day-uyo.git
```

```bash
cd build-day-uyo
```

```bash
stellar contract build
```

It's done when you see `Build Complete`. That can take several minutes on an older laptop.

## Optional: tab completion

On a Mac (zsh), this lets you press Tab to finish `stellar` commands:

```bash
echo "source <(stellar completion --shell zsh)" >> ~/.zshrc
```

For Bash, PowerShell and others, see [Autocompletion](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#autocompletion).

## No install? Use Codespaces

You need a free GitHub account and a steady connection.

1. Open [this repo](https://github.com/boundlessfi/build-day-uyo).
2. Click **Code**, then the **Codespaces** tab, then **Create codespace on main**.
3. Wait a few minutes. When the terminal is ready, steps 1 to 4 are already done for you.

## Stuck?

See [common problems](../CHEATSHEET.md#common-problems), or put your red note up. If nothing works, pair with someone and we'll fix it at the break.

---

Next: [02 · Make a testnet account](../02-account/)
