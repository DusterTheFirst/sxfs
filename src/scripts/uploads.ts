window.addEventListener("load", () => {
    let table = document.getElementById("uploads_table");
    const info_div = document.querySelector("div.info");
    const preview = document.querySelector("div.info");
    const open_link = document.querySelector("div.info");
    const delete_link = document.querySelector("div.info");

    let rows = table.querySelectorAll<HTMLTableRowElement>("tr[data-link]");
    for (let row of rows) {
        let link: string = row.dataset.link;

        
    }
});