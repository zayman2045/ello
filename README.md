# Ello

Ello is a full-stack web application that leverages the OpenAI API, and provides users with an instant messenger-like interface, enabling them to customize and engage in chat conversations with their own AI friends.

## Overview

The frontend is developed in Python, leveraging the Django framework for handling HTTP requests, manipulating and rendering HTML templates, and interfacing with the custom-built API layer. Ello leverages server-side conditional rendering through view function context and template tags. This approach provides greater control over the rendered elements and reduces the number of templates required to display all pages, improving both performance and developer experience.

Ello's backend is constructed in Rust, utilizing the Actix-web framework to establish the web service and the async-openai crate to formulate and dispatch requests to the OpenAI API using Rust data types. An OpenAI API key is used to generate a client for handling API calls, and is shared amongst all HTTP services within the specified scope. User-provided messages and other incoming JSON data are deserialized into structs, which are then employed to generate, update and delete individual assistants, threads, and runs. 

## Getting Started

### To run this project locally:

Before running the application, you need to set up your OpenAI API key. Create a `.env` file in the root of the project and add the following line:

```zsh
OPENAI_API_KEY=your_openai_api_key
```

Replace your_openai_api_key with your actual OpenAI API key. This key is used by the backend service to authenticate with the OpenAI API. If you do not have an OpenAI API key, you can create one [here] (https://platform.openai.com/api-keys).

Make sure you have Docker and Docker Compose installed. If not, you can download and install Docker from [here](https://docs.docker.com/get-docker/) and Docker Compose from [here](https://docs.docker.com/compose/install/).

Once Docker and Docker Compose are installed, you can build and run the application using the following command from the project root directory:

```zsh
docker-compose up --build
``` 

The application will be available at http://0.0.0.0:8000.

