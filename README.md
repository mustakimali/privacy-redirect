# privacy-redirect

A web service to remove known tracking urls before redirecting.

## How to Use?
### For your browser

[<img src="frontend/src/get-the-addon-fx-apr-2020.svg" height="60" />](https://addons.mozilla.org/en-US/firefox/addon/privacydir/)


### For your website

Install the script in your website.

```html
<script src="https://privacydir.com/app/script.js"></script>
```


## How to Contribute?

Please send PR to:

* Add a known tracking query string or fragements in the [tracker rules](./tracking-params/src/rules.rs).
* Improve the performance of the [web service](./web/).

## Submit Bug Report

Please create a new issue or submit your feedback using [this form](https://forms.gle/xd5XFT6JHRTvvwqY6).

[<img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" height="60" />](https://www.buymeacoffee.com/mustak.im)

## To Do
- [x] Browser Extension (in progress) (Firefox: Submitted for review, Chrome: WIP)
- [ ] Emit `/metrics` from the app
- [ ] Tracing
