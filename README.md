# 🤖 larabot

This repo is a non-published Rust library that has the common code used by my Discord bots.

Although the code itself is very opinionated, feel free to fork the project and adjust it however you wish.

## ✨ Features

### 🔃 Interaction Handling Flow

#### 🫡 Command Creation

Has a method to create a commands for the given guild or globally based on the build configuration.

#### 🏃 Interaction Handling

Provides a trait to handle the interaction, which is meant to be implemented on each interaction's struct.

The trait's provided `handle` method sets up automatic deferring, runs the interaction, and handles the error by
reporting it to the user.

### 🌐 Localization

Has a struct to hold text in a localized manner, with methods to get the text in the desired language or return
Discord-compatible arrays of localized texts.

All user-facing texts used in this crate are also localized.

### 🙈 Generic Option to Result Conversion

Provides a trait to convert an `Option` to a `Result` concisely. This is here because the Discord API returns many
optional values.

### 📝 Tracing

Has a function to set up tracing with a formatter, environment filtering, conditional journald support, and log crate
compatibility.

### 📡 Event Handling

Provides methods to handle the ready event, errors that occur while handling events or while receiving an event.

### 🎨 Colors

Allows you to use [Discord-defined colors](https://discord.com/branding) with an enum the crate provides.
