// Add event listener for onload
window.addEventListener("load", () => {
    /** The uploads  */
    const uploads = document.querySelectorAll<HTMLDivElement>("div.upload")!;

    // Loop through the uploads
    for (let upload of uploads) {
        const { id, filename, domain, https } = upload.dataset;
        const delete_button = upload.querySelector<HTMLButtonElement>("button.delete")!;
        const copy_button = upload.querySelector<HTMLButtonElement>("button.copy")!;

        upload.addEventListener("click", (e) => {
            e.stopImmediatePropagation();

            location.href = `/u/${id}/${filename}`;
        });

        delete_button.addEventListener("click", (e) => {
            e.stopImmediatePropagation();

            if (confirm(`Delete ${filename}?`))
                location.href = `/u/d/${id}/${filename}`;
        });

        copy_button.addEventListener("click", async (e) => {
            e.stopImmediatePropagation();

            await navigator.clipboard.writeText(`http${https === "true" ? "s" : ""}://${domain}/u/${id}/${filename}`);
        });
    }

    // Get images to lazy load
    const lazyImages = document.querySelectorAll("img.lazy");

    const lazyImageObserver = new IntersectionObserver((entries, _observer) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                const lazyImage = entry.target as HTMLImageElement;
                lazyImage.src = lazyImage.dataset.src ?? "";
                lazyImage.onload = () => {
                    lazyImage.classList.remove("lazy");
                }
                lazyImageObserver.unobserve(lazyImage);
            }
        });
    });

    lazyImages.forEach((lazyImage) => {
        lazyImageObserver.observe(lazyImage);
    });
});