loadPrivacyRedirect();

function loadPrivacyRedirect() {
    console.log("[PR] Loaded");
    document.querySelector("body").addEventListener("click", function (event) {
        const SERVER_PREFIX = "https://privacy-redirect.fly.dev/?";
        var node;
        if (event.target.tagName === "A") {
            node = event.target;
        } else if (event.target.parentNode.tagName === "A") {
            node = event.target.parentNode;
        } else {
            return;
        }

        if (
            node.href.startsWith("http")
            && (!node.href.startsWith(window.location.origin) || node.href.indexOf("?") >= 0) // different site or has query string
            && !node.href.startsWith(SERVER_PREFIX) // not already updated
        ) {
            node.href = SERVER_PREFIX + node.href;
        }
    });
}