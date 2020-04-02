// Add event listener for onload
window.addEventListener("load", () => {
    /** The form to read from  */
    const form = document.getElementById("form")!;

    /** The username input element */
    const username = form.querySelector<HTMLInputElement>("input[name='username']")!;
    /** The password input element */
    const password = form.querySelector<HTMLInputElement>("input[name='password']")!;

    /** The alert box to show on error */
    const alert = form.querySelector<HTMLDivElement>("div.alert")!;
    alert.hidden = true;

    // Listen for the form's submission
    form.addEventListener("submit", async (e) => {
        // Prevent actual submission of the form
        e.preventDefault();

        // Send the user info to the server async
        const response = await fetch("/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: `username=${encodeURIComponent(username.value)}&password=${encodeURIComponent(password.value)}`
        });

        // Check if the user is valid
        if (response.status === 202) {
            // Send the user to the preferred redirect or to the home if there is none
            window.location.href = getParam("redirect") ?? "/";
        } else {
            // Alert the user of invalid username/password
            alert.hidden = false;
        }
    });
});

/** Helper method to get the URL Get params */
function getParams() {
    return window.location.search.substr(1).split("&").map(x => x.split("=")).map(([key, value]) => ({ key, value: decodeURIComponent(value) }));
}

/** Helper method to get one get param from the url */
function getParam(key: string): string | undefined {
    return getParams().find(i => i.key === key)?.value;
}
