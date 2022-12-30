(function () {
    console.log("[PR] Loaded");
    document.querySelector("body").addEventListener("click", function (event) {
        var node;
        if (event.target.tagName === "A") {
            node = event.target;
        } else if (event.target.parentNode.tagName === "A") { 
            node = event.target.parentNode;
        } else {
            return;
        }

        node.href = "https://privacy-redirect.fly.dev/?" + node.href;
        //event.preventDefault();
    });
})()
