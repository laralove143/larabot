# 🤖 larabot

This repository is a non-published Elixir library that has the common code used by my Discord bots.

Although the code itself is very opinionated, feel free to fork the project and adjust it however you wish.

## ✨ Features

### 📝 Logging

Sets up logging with [DiscoLog](https://github.com/mrdotb/disco-log).

### 📨 User Feedback

Includes a command that users can use to provide feedback, which uses logging to report the feedback.

### 🏃 Command Behavior

Provides a behavior to define and handle a command.

Has a method to create commands for the given guild and globally. Saves command IDs to be accessible from anywhere.

### 🚨 Error Handling

Provides methods to handle errors concisely.

### 📡 Event Handling

Provides a behavior to handle events.

### ⏰ Scheduling

Has a behavior to easily run periodic tasks in a `GenServer`.

### 🎨 Colors

Allows you to use [Discord-defined colors](https://my.corebook.io/1zObrQ89Q4wHhgFCfYIUhMUvmNf4XjxO/04-color-gradients/colors?m=6596168) and colors of button styles.
