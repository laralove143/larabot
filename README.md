# 🤖 larabot

This repo is a non-published Rust library that has the common code used by my Discord bots.

Although the code itself is very opinionated, feel free to fork the project and adjust it however you wish.

## ✨ Features

### 🌐 Localization

Provides a struct to hold text in a localized manner, with methods to get the text in the desired language or return
Discord-compatible arrays of localized text.

### 🙈 Generic Option to Result Conversion

Provides a trait to convert an `Option` to a `Result` concisely. This is here because the Discord API returns many
optional values.
