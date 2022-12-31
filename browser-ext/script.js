const privacyRedirect = {
    SERVER: "https://privacydir.com",
    SERVER_PREFIX: "https://privacydir.com/?",
    PROCESSED_URLS: new Set(),

    init: function () {
        var isExtension = false;
        if (typeof chrome === "object" || typeof browser === "object") {
            // Browser Extension
            let inst = typeof chrome === "object" ? chrome : browser;
            isExtension = true;

            inst.webRequest.onBeforeRequest.addListener(privacyRedirect.handleRedirect, {
                urls: ["<all_urls>"]
            }, ["blocking"]);
        } else {
            // Page Script
            document.querySelector("body").addEventListener("click", function (event) {
                var node = event.target;
                while (node != null) {
                    if (node == null) return;
                    if (node.tagName === "A") break;
                    node = node.parentNode;
                }
                if (node === null || node.href === null) return;

                node.href = processUrl(node.href, window.location.origin);
            });
        }

        console.log(`[Privacy Redirect] ${isExtension ? "Extension " : ""}Loaded and protecting your privacy (${this.SERVER})`);

    },
    processUrl: function (url, origin) {
        if (url.startsWith(this.SERVER_PREFIX)) return url; // already updated

        if (
            url.startsWith("http")
            && (!url.startsWith(origin) || url.indexOf("?") >= 0) // different site or has query string
        ) {
            url = this.SERVER_PREFIX + url;
        } else if (url.startsWith("/") && url.indexOf("?") >= 0) { // relative link with query string
            var absHref = origin + url;
            url = absHref;
        }
        return url;
    },
    handleRedirect: function (requestDetails) {
        try {
            return privacyRedirect.handleRedirectInner(requestDetails);
        } catch (e) {
            console.warn(`[Privacy Redirect] Handle error: ${e} (Url: ${requestDetails.url})`);
        };
    },
    handleRedirectInner: function (requestDetails) {
        var url = requestDetails.url;
        var origin = requestDetails.originUrl;

        if (requestDetails.documentUrl != undefined
            || (origin != undefined && origin.startsWith(privacyRedirect.SERVER))) {
            return {};
        }

        var redirected = privacyRedirect.processUrl(url, origin == undefined ? null : new URL(origin).origin);

        if (url != redirected) {
            console.log(`Processing: ${url}`);
            return { redirectUrl: redirected };
        } else {
            return {};
        }
    }

};

try {
    privacyRedirect.init();
} catch (e) {
    console.warn(`[Privacy Redirect] Error Loading: ${e}`);
}
