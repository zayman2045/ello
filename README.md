# Ello

Ello is a full-stack web application that allows users to build and interact with their own AI assistants using the OpenAI API.

## Overview

Ello's frontend is written in python by way of the Django framework. Django uses...

The backend of the app uses...

Ello utilizes the async-openai crate for constructing requests and processing responses from OpenAI's Assistants API. User supplied "messages" are added to conversation "threads" that can be ran by assistants to generate an appropriate response.

## Usage