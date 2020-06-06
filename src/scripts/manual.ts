window.addEventListener("load", () => {
    let input = document.querySelector<HTMLInputElement>(".manualupload.file")!;

    document.querySelector(".visit.manualupload")!.addEventListener("click", (e) => {
        e.preventDefault();

        input.click();
    });

    input.addEventListener("change", async () => {
        if (input.files === null) {
            return;
        }

        let file = input.files[0];

        let filename = file.name;
        let filecontents = await file.arrayBuffer();

        await fetch(`${location.protocol}//${await (await fetch("/upload_url")).text()}/u?filename=${encodeURIComponent(filename)}`, {
            body: filecontents,
            method: "POST",
            credentials: "include",
            headers: {
                "Cookie": document.cookie
            }
        });

        location.assign("/u");
    });
});