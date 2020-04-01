window.addEventListener("load", () => {
    const form = document.getElementById("form");

    const username = form.querySelector<HTMLInputElement>("input[name='username']");

    const password = form.querySelector<HTMLInputElement>("input[name='password']");

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
            window.location.href = getParam("redirect") ?? "/";
        } else {
            alert("Invalid cred");
        }
    });
});

function getParams() {
    return window.location.search.substr(1).split("&").map(x => x.split("=")).map(([key, value]) => ({ key, value: decodeURIComponent(value) }));
}

function getParam(key: string) {
    return getParams().find(i => i.key === key)?.value;
}
