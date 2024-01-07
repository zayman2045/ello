# Ello

Ello is a full-stack web application that uses the OpenAI API to allow users to customize and interact with their own AI assistants through a chat interface.

## Overview

The frontend of Ello is developed in Python, leveraging the Django framework for handling HTTP requests, manipulating and rendering HTML templates, and interfacing with the custom-built API layer.

Ello's backend is constructed in Rust, utilizing the async-openai crate to formulate and dispatch requests to the OpenAI API using Rust data types. User-provided messages and other incoming JSON data are deserialized into structs, which are then employed to generate assistants, threads, and runs.

## Usage