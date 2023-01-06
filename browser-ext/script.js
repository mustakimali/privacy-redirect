const privacyRedirect = {
    SERVER: "https://privacydir.com",
    SERVER_PREFIX: "https://privacydir.com/?",

    ALLOWED_LIST_GLOBAL: [],
    RECENT_PROCESSES: {},
    ALLOW_INTERNAL_REDIRECTS: [],

    init: function () {
        var isExtension = false;
        if (typeof chrome === "object") {
            // Browser Extension
            chrome.webRequest.onBeforeRequest.addListener(privacyRedirect.handleRedirect, {
                urls: ["<all_urls>"]
            }, ["blocking"]);
        } else {
            // Page Script
            privacyRedirect.initPageScript();
        }

        setInterval(() => {
            privacyRedirect.RECENT_PROCESSES = {};
        }, 2000);

        console.log(`[Privacy Redirect] ${isExtension ? "Extension " : ""}Loaded and protecting your privacy (${this.SERVER})`);

    },
    initPageScript: function () {
        document.querySelector("body").addEventListener("click", function (event) {
            var node = event.target;
            while (node != null) {
                if (node == null) return;
                if (node.tagName === "A") break;
                node = node.parentNode;
            }
            if (node === null || node.href === null) return;

            const newUrl = privacyRedirect.processUrl(node.href, window.location.origin);
            if (node.href != newUrl) {
                node.href = newUrl;
            }
        });
    },
    processUrl: function (url, origin) {
        if (url.startsWith(this.SERVER_PREFIX)) return url; // already updated

        if (
            url.startsWith("http")
            && (origin == null || !url.startsWith(origin)) // different site and has query string
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
        if (requestDetails.method !== "GET") {
            return {};
        }

        if (requestDetails.type !== "main_frame" && requestDetails.type !== "sub_frame") {
            return {};
        }
        var url = requestDetails.url;
        var origin = requestDetails.originUrl;

        // if (requestDetails.documentUrl != undefined
        //     || (origin != undefined && origin.startsWith(privacyRedirect.SERVER))) {
        //     return {};
        // }

        // Skip processing if the domain is in the allow list
        const allowed = privacyRedirect.getAllowedList();
        const urlParam = new URL(url);
        for (var i = 0; i < allowed.length; i++) {
            if (urlParam.hostname.indexOf(allowed[i]) >= 0) {
                // Skip: Found on allow list
                return {};
            }
        }

        // Skip processing repeated requests, it usually means redirect loop
        if (privacyRedirect.RECENT_PROCESSES[url] != undefined) {
            return {};
        }

        var redirected = privacyRedirect.processUrl(url, origin == undefined ? null : new URL(origin).origin);

        if (url !== redirected) {
            // Redirect
            privacyRedirect.RECENT_PROCESSES[url] = true;
            return { redirectUrl: redirected };
        } else if (!redirected.startsWith(privacyRedirect.SERVER_PREFIX) && urlParam.search !== "") {
            // internal redirect but there's a query string
            // only process if this is allowed

            if (privacyRedirect.ALLOW_INTERNAL_REDIRECTS.find(reg => url.match(reg)) !== undefined) {
                redirected = privacyRedirect.processUrl(url, null);
                return {
                    redirectUrl: redirected
                };
            }
        }

        return {};
    },
    getAllowedList: function () {
        return privacyRedirect.ALLOWED_LIST_GLOBAL;
    },
    updateAllowedList: function () {
        fetch(`${privacyRedirect.SERVER}/api/v1/allowed-list`)
            .then(r => r.json())
            .then(r => {
                const list = r.result;
                privacyRedirect.ALLOWED_LIST_GLOBAL = list;
                privacyRedirect.ALLOW_INTERNAL_REDIRECTS = r.internal_redirect.map(rg => new RegExp(rg));

                console.log("The following domains will be skipped as they are known to break due to missing referrer.: " + JSON.stringify(list));
            }).catch(r => console.warn("[Privacy Redirect] Error updating allow list"));
    }

};

try {
    privacyRedirect.init();

    // Update list of domains to skip processing
    // They are known to break due to missing referrer.
    privacyRedirect.updateAllowedList();

    // Update the list every 30 mins
    setInterval(function () {
        privacyRedirect.updateAllowedList();
    }, 30 * 60 * 1000);
} catch (e) {
    console.warn(`[Privacy Redirect] Error Loading: ${e}`);
}
