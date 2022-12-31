const privacyRedirect = {
    SERVER: "https://privacydir.com",
    SERVER_PREFIX: "https://privacydir.com/?",
    EXTENSION_LOADED: false,

    init: function () {
        var isExtension = false;
        if (navigator.userAgent.indexOf("Firefox") >= 0 && (typeof chrome === "object" || typeof browser === "object")) {
            // Browser Extension
            let inst = typeof chrome === "object" ? chrome : browser;
            isExtension = true;

            inst.webRequest.onBeforeRequest.addListener(privacyRedirect.handleRedirect, {
                urls: ["<all_urls>"]
            }, ["blocking"]);
            privacyRedirect.EXTENSION_LOADED = true;
        } else {
            // Page Script
            privacyRedirect.initPageScript();
        }

        console.log(`[Privacy Redirect] ${isExtension ? "Extension " : ""}Loaded and protecting your privacy (${this.SERVER})`);

    },
    initPageScript: function () {
        document.querySelector("body").addEventListener("click", function (event) {
            console.log(event);
            var node = event.target;
            while (node != null) {
                if (node == null) return;
                if (node.tagName === "A") break;
                node = node.parentNode;
            }
            if (node === null || node.href === null) return;

            const newUrl = privacyRedirect.processUrl(node.href, window.location.origin);
            if (node.href != newUrl) {
                console.log(`Processing: ${node.href}`);
                node.href = newUrl;
            }
        });
    },
    processUrl: function (url, origin) {
        if (url.startsWith(this.SERVER_PREFIX)) return url; // already updated

        if (
            url.startsWith("http")
            && (!url.startsWith(origin)) // different site and has query string
        ) {
            url = this.SERVER_PREFIX + url;
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
