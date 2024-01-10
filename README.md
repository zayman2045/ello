# Ello

Ello is a full-stack web application that uses the OpenAI API to allow users to customize and interact with their own AI assistants through a chat interface.

## Overview

The frontend is developed in Python, leveraging the Django framework for handling HTTP requests, manipulating and rendering HTML templates, and interfacing with the custom-built API layer. Ello leverages server-side conditional rendering through view function context and template tags. This approach provides greater control over the rendered elements and reduces the number of templates required to display all pages, improving both performance and developer experience.

Ello's backend is constructed in Rust, utilizing the Actix-web framework to establish the web service and the async-openai crate to formulate and dispatch requests to the OpenAI API using Rust data types. An OpenAI API key is used to generate a client for handling API calls, and is shared amongst all HTTP services within the specified scope. User-provided messages and other incoming JSON data are deserialized into structs, which are then employed to generate, update and delete individual assistants, threads, and runs. 

## Getting Started

To run the project locally: