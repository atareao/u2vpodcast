// Config
const isOpenClass = "modal-is-open";
const openingClass = "modal-is-opening";
const closingClass = "modal-is-closing";
const animationDuration = 400; // ms
let visibleModal = null;

var ready = (callback) => {
    if (document.readyState != "loading"){
        callback();
    }else{
        document.addEventListener("DOMContentLoaded", callback);
    }
};

ready(() => {
    /* Do things after DOM has fully loaded */
    setEditElements();
    setDeleteElements();
});
function setDeleteElements(){
    const deleteElements = document.getElementsByClassName("delete-podcast");
    if (deleteElements != null) {
        Array.from(deleteElements).forEach((element) => {
            const id = element.getAttribute("data-id");
            element.addEventListener("click", (event) => {
                event.preventDefault();
                console.log("Delete podcast");
                console.log(document.cookie);
                fetch(`/podcasts?id=${id}`, {
                    method: "DELETE",
                    cache: "no-cache",
                    credentials: "same-origin",
                })
                    .then((response) => response.json())
                    .then((json) => {
                        console.log(json);
                        if (json.result == "ok") {
                            const tr = element.parentNode.parentNode;
                            tr.parentNode.removeChild(tr);
                        }
                    })
                    .catch((err) => console.log("Error", err));
            });
        });
    }
}
function setEditElements(){
    let podcastDialog;
    const modalEditPodcast = document.getElementById("dialog-podcast");
    if (modalEditPodcast != null) {
        podcastDialog = new PodcastDialog(modalEditPodcast);
    }
    const editElements = document.getElementsByClassName("open-dialog-podcast");
    if (editElements != null) {
        Array.from(editElements).forEach((element) => {
            element.addEventListener("click", (event) => {
                event.preventDefault();
                console.log("Edit podcast");
                Array.from(element.getAttributeNames()).forEach((key) => {
                    const value = element.getAttribute(key);
                    console.log(`${key} => ${value}`);
                });
                podcastDialog.open(element);
            });
        });
    }
}

class PodcastDialog {
    constructor(dialog) {
        this.dialog = dialog;
        this.init();
    }
    init() {
        document
            .getElementById("dialog-podcast-close")
            .addEventListener("click", (event) => {
                this.close("cancel");
            });
        document
            .getElementById("dialog-podcast-cancel")
            .addEventListener("click", (event) => {
                this.close("cancel");
            });
        document
            .getElementById("dialog-podcast-confirm")
            .addEventListener("click", (event) => {
                this.close("confirm");
            });
        this.active = document.getElementById("dialog-podcast-active");
        this.name = document.getElementById("dialog-podcast-name");
        this.url = document.getElementById("dialog-podcast-url");
    }
    isOpen() {
        return this.dialog.hasAttribute("open") &&
            this.dialog.getAttribute("open") != "false"
            ? true
            : false;
    }
    open(element) {
        this.element = element;
        this.active.checked = ((element.getAttribute("data-active")==="true")?true:false);
        this.name.value = element.getAttribute("data-name");
        this.url.value = element.getAttribute("data-url");
        this.role = element.getAttribute("data-role");
        this.name.readOnly = (this.role == "edit");
        if (this.isScrollbarVisible()) {
            document.documentElement.style.setProperty(
                "--scrollbar-width",
                `${this.getScrollbarWidth()}px`,
            );
        }
        document.documentElement.classList.add(isOpenClass, openingClass);
        setTimeout(() => {
            visibleModal = this.dialog;
            document.documentElement.classList.remove(openingClass);
        }, animationDuration);
        this.dialog.setAttribute("open", true);
    }

    close(status) {
        if (status == "confirm") {
            if (this.role == "edit") {
                fetch("/podcasts", {
                    method: "POST",
                    cache: "no-cache",
                    credentials: "same-origin",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        active:this.active.checked, 
                        name: this.name.value,
                        url: this.url.value,
                    }),
                })
                    .then((response) => response.json())
                    .then((json) => {
                        if (json.result == "ko") {
                            return;
                        }
                        const items = this.element.parentNode.parentNode.children;
                        const checked = ((this.active.checked)?"fa-square-check":"fa-square");
                        items[0].innerHTML = `<i class="fa-regular ${checked}"></i>`;
                        items[1].innerHTML = this.name.value;
                        items[2].innerHTML = this.url.value;
                        console.log(items);
                    })
                    .catch((err) => console.log("Error", err));
            } else if (this.role == "add") {
                fetch("/podcasts", {
                    method: "POST",
                    cache: "no-cache",
                    credentials: "same-origin",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        active:this.active.checked, 
                        name: this.name.value,
                        url: this.url.value,
                    }),
                })
                    .then((response) => response.json())
                    .then((json) => {
                        if (json.result == "ko") {
                            return;
                        }
                        const data = json.content;
                        const table = document.getElementById("podcasts");
                        const tbody = table.children[0];
                        const checked = ((data.active)?"fa-square-check":"fa-square");
                        console.log(data);
                        const inner = tbody.innerHTML;
                        tbody.innerHTML = `
                        <tr id="${data.id}">
                            <td><i class="fa-regular fa-square ${checked}"></i></td>
                            <td>${data.name}</td>
                            <td>${data.url}</td>
                            <td>${data.last_pub_date}</td>
                            <td>
                                <button role="button" data-active="${data.active} "data-name="${data.name}" data-url="${data.url}" data-target="dialog-podcast" data-role="edit" class="open-dialog-podcast" data-tooltip="Edit podcast">
                                    <i class="fa-solid fa-pen"></i>
                                </button>
                            </td>
                            <td>
                                <button role="button" data-id="${data.id}" class="delete-podcast" data-tooltip="Delete podcast">
                                    <i class="fa-solid fa-trash"></i>
                                </button>
                            </td>
                        </tr>
                        ` + inner;
                        console.log(table);
                        setDeleteElements();
                        setEditElements();
                    })
                    .catch((err) => console.log("Error", err));
            }
        }
        document.documentElement.classList.add(closingClass);
        setTimeout(() => {
            document.documentElement.classList.remove(closingClass, isOpenClass);
            document.documentElement.style.removeProperty("--scrollbar-width");
            this.dialog.removeAttribute("open");
        }, animationDuration);
    }

    getScrollbarWidth() {
        // Creating invisible container
        const outer = document.createElement("div");
        outer.style.visibility = "hidden";
        outer.style.overflow = "scroll"; // forcing scrollbar to appear
        outer.style.msOverflowStyle = "scrollbar"; // needed for WinJS apps
        document.body.appendChild(outer);

        // Creating inner element and placing it in the container
        const inner = document.createElement("div");
        outer.appendChild(inner);

        // Calculating difference between container's full width and the child width
        const scrollbarWidth = outer.offsetWidth - inner.offsetWidth;

        // Removing temporary elements from the DOM
        outer.parentNode.removeChild(outer);

        return scrollbarWidth;
    }

    // Is scrollbar visible
    isScrollbarVisible() {
        return document.body.scrollHeight > screen.height;
    }
}

function handleLogoutClick() {
    fetch("/logout")
        .then((data) => {
            console.log(data);
        })
        .catch((error) => {
            console.log(error);
        });
    window.location.ref = "/";
    location.reaload();
}

