# privacy-redirect App

This application serves [privacydir.com](https://privacydir.com) website.


## Backend
This takes any links as a query string, removes any known tracking query strings (see using the crate [tracking-params](../tracking-params)).

It then returns a piece of html that instructs the browser to redirect to the destination without making the referrer available to the website.

## Frontend
It also serves static contents to render the application landingg page ([privacydir.com/app](https://privacydir.com/app)). This is done by building the [frontend application](../frontend) at the time the Docker container is built.


## Technology
* Rust
* `actix-web` and various open source crates.