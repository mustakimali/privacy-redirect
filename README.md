# privacy-redirect

A web service to protect your privacy online by removing known trackers from outgoing links before redirecting.

## How to Use?
### For your browser

Protect your privacy on any website.

[<img src="frontend/src/get-the-addon-fx-apr-2020.svg" height="60" />](https://addons.mozilla.org/en-US/firefox/addon/privacydir/)


### For your website

Install the script in your website to protect the privacy of your visitor.

```html
<script src="https://privacydir.com/app/script.js"></script>
```


## How to Contribute?

Please send PR to:

* Add a known tracking query string or fragments in the [tracker rules](./tracking-params/src/rules.rs). Any change will be published in [crates.io](https://crates.io/crates/tracking-params) as `tracking-params`.
* Improve the performance of the [web service](./web/). This application is the backend and uses the `tracking-params` crate.
* Contribute to the [landing page](./frontend/) design.

## Submit Bug Report

If a site is broken then it's likely because it depends on the referrer to function properly (what a nightmare!). In this case you can add this domain to the global allow list [here](web/src/main.rs#L10). In the future this list can be managed from the extension itself.

Please create a new issue in Github or submit your feedback using [this form](https://forms.gle/xd5XFT6JHRTvvwqY6).

[<img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" height="60" />](https://www.buymeacoffee.com/mustak.im)


## Notes on Privacy
This service is meant to protect- not invade your privacy. It doesn't track your browsing history. The source code is public as a proof.

## To Do
- [x] Browser Extension (Firefox: Released and maintained, Chrome: WIP)
- [ ] Tracing using honeycomb to monitor performance (broken)
