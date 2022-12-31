const privacyRedirect = {
    init: function () {
        console.log("[Privacy Redirect] Loaded and protecting your privacy (https://privacydir.com)");

        document.querySelector("body").addEventListener("click", function (event) {
            const SERVER_PREFIX = "https://privacydir.com/?";
            var node = event.target;
            while (node != null) {
                if (node == null) return;
                if (node.tagName === "A") break;
                node = node.parentNode;
            }

            if (node.href.startsWith(SERVER_PREFIX)) return; // not already updated

            if (
                node.href.startsWith("http")
                && (!node.href.startsWith(window.location.origin) || node.href.indexOf("?") >= 0) // different site or has query string
            ) {
                node.href = SERVER_PREFIX + node.href;
            } else if (node.href.startsWith("/") && node.href.indexOf("?") >= 0) { // relative link with query string
                var absHref = window.location.origin + node.href;
                node.href = absHref;
            }

            // event.preventDefault();
            // console.log(node.href);
        });

    }
};

privacyRedirect.init();
