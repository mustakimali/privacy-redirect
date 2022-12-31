const privacyRedirect = {
    SERVER: "https://privacydir.com",
    SERVER_PREFIX: "https://privacydir.com/?",
    PROCESSED_URLS: new Set(),

    init: function () {
        browser.webRequest.onBeforeRequest.addListener(handleRedirect, {
            urls: ["<all_urls>"]
        },
            ["blocking"]
        );

        console.log(`[Privacy Redirect] ${this.isExtension() ? "Extension " : ""}Loaded and protecting your privacy (${this.SERVER})`);

        document.querySelector("body").addEventListener("click", function (event) {
            var node = event.target;
            while (node != null) {
                if (node == null) return;
                if (node.tagName === "A") break;
                node = node.parentNode;
            }
            if (node === null || node.href === null) return;

            node.href = processUrl(node.href, window.location.origin);

            // event.preventDefault();
            // console.log(node.href);
        });

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
    isExtension: function () {
        return typeof browser === 'object';
    },
};

try {
    privacyRedirect.init();
} catch (e) {
    console.warn(`[Privacy Redirect] Error Loading: ${e}`);
}

function handleRedirect(requestDetails) {
    try {
        return handleRedirectInner(requestDetails);
    } catch (e) {
        console.warn(`[Privacy Redirect] Handle error: ${e} (Url: ${requestDetails.url})`);
    }
}
function handleRedirectInner(requestDetails) {
    var url = requestDetails.url;
    if (requestDetails.documentUrl != undefined
        || (requestDetails.originUrl != undefined && requestDetails.originUrl.startsWith(privacyRedirect.SERVER))) {
        return {};
    }

    var redirected = privacyRedirect.processUrl(url, requestDetails.originUrl == undefined ? null : new URL(requestDetails.originUrl).origin);

    if (url != redirected) {
        //console.log(requestDetails);
        console.log(`Processing: ${url}`);
        return { redirectUrl: redirected };
    } else {
        return {};
    }
}
