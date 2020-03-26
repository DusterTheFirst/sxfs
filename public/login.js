window.addEventListener("load", () => {
    /** @type {HTMLFormElement} */
    const form = document.getElementById("form");

    /** @type {HTMLInputElement} */
    const username = form.querySelector("input[name='username']");

    /** @type {HTMLInputElement} */
    const password = form.querySelector("input[name='password']");

    form.addEventListener("submit", async (e) => {
        e.preventDefault();

        const response = await fetch("/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: `username=${encodeURIComponent(username.value)}&password=${encodeURIComponent(password.value)}`
        });
        if (response.status === 202) {
            window.location = getParam("redirect") ?? "/";
        } else {
            alert("Invalid cred");
        }
    });
});

function getParams() {
    return window.location.search.substr(1).split("&").map(x => x.split("=")).map(([key, value]) => ({ key, value: decodeURIComponent(value) }));
}

/**
 * @param {string} key
 */
function getParam(key) {
    return getParams().find(i => i.key === key)?.value;
}
