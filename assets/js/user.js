// Config
const isOpenClass = "modal-is-open";
const openingClass = "modal-is-opening";
const closingClass = "modal-is-closing";
const animationDuration = 400; // ms
let visibleModal = null;

var ready = (callback) => {
  if (document.readyState != "loading") {
    callback();
  } else {
    document.addEventListener("DOMContentLoaded", callback);
  }
};

ready(() => {
  setAddElements();
  setEditElements();
  setDeleteElements();
});
function setAddElements() {
  const addElements = document.getElementsByClassName("add");
  let dialog;
  const modalAdd = document.getElementById("dialog");
  if (modalAdd != null) {
    dialog = new Dialog(modalAdd);
    if (addElements != null){
      Array.from(addElements).forEach((element) => {
        element.addEventListener("click", (event) =>{
          event.preventDefault();
          console.log("Add");
          dialog.open();

        });
      });
    }
  }
}

function setDeleteElements() {
  const deleteElements = document.getElementsByClassName("delete");
  if (deleteElements != null) {
    Array.from(deleteElements).forEach((element) => {
      const id = element.getAttribute("data-id");
      console.log(id);
      element.addEventListener("click", (event) => {
        event.preventDefault();
        console.log("Delete");
        console.log(document.cookie);
        fetch(`/api/1.0/users/?user_id=${id}`, {
          method: "DELETE",
          cache: "no-cache",
          credentials: "same-origin",
        })
          .then((response) => response.json())
          .then((json) => {
            console.log(json);
            if (json.status) {
              const tr = element.parentNode.parentNode;
              tr.parentNode.removeChild(tr);
            }
          })
          .catch((err) => console.log("Error", err));
      });
    });
  }
}
function setEditElements() {
  let dialog;
  const modalEdit = document.getElementById("dialog");
  if (modalEdit != null) {
    dialog = new Dialog(modalEdit);
  }
  const editElements = document.getElementsByClassName("edit");
  if (editElements != null) {
    Array.from(editElements).forEach((element) => {
      element.addEventListener("click", (event) => {
        event.preventDefault();
        console.log("Edit");
        Array.from(element.getAttributeNames()).forEach((key) => {
          const value = element.getAttribute(key);
          console.log(`${key} => ${value}`);
        });
        dialog.open(element);
      });
    });
  }
}

function set_selected(select, value){
  for (const option of select.options) {
    option.selected = (option.value == value);
  }
}

class Dialog {
  constructor(dialog) {
    this.dialog = dialog;
    this.init();
  }
  init() {
    document
      .getElementById("dialog-close")
      .addEventListener("click", () => {
        this.close("cancel");
      });
    document
      .getElementById("dialog-cancel")
      .addEventListener("click", () => {
        this.close("cancel");
      });
    document
      .getElementById("dialog-confirm")
      .addEventListener("click", () => {
        this.close("confirm");
      });
    this.name = document.getElementById("dialog-name");
    this.role = document.getElementById("dialog-role");
    this.active = document.getElementById("dialog-active");
  }
  isOpen() {
    return this.dialog.hasAttribute("open") &&
      this.dialog.getAttribute("open") != "false"
      ? true
      : false;
  }
  open(element) {
    if(element != null){
      this.dialog_role = "edit";
      document.getElementById("dialog-type").innerText = "Edit";
      this.element = element;
      this.name.value = element.getAttribute("data-name");
      set_selected(this.role, element.getAttribute("data-role"));
      this.active.checked = element.getAttribute("data-active") === "true";
      this.name.readOnly = (this.dialog_role == "edit");
      if (this.isScrollbarVisible()) {
        document.documentElement.style.setProperty(
          "--scrollbar-width",
          `${this.getScrollbarWidth()}px`,
        );
      }
    }else{
      this.dialog_role = "add";
      document.getElementById("dialog-type").innerText = "Add";
      this.name.value = "";
      set_selected(this.role, "User");
      this.active.checked = true;
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
      if (this.dialog_role == "edit") {
        fetch("/users", {
          method: "POST",
          cache: "no-cache",
          credentials: "same-origin",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            active: this.active.checked,
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
            const checked = ((this.active.checked) ? "fa-square-check" : "fa-square");
            items[0].innerHTML = `<i class="fa-regular ${checked}"></i>`;
            items[1].innerHTML = this.name.value;
            items[2].innerHTML = this.role.value;
            console.log(items);
          })
          .catch((err) => console.log("Error", err));
      } else if (this.dialog_role == "add") {
        const data = JSON.stringify({
            name: this.name.value,
            role: this.role.value,
            active: this.active.checked,
          });
        console.log(data);
        fetch("/api/1.0/channels/", {
          method: "POST",
          cache: "no-cache",
          credentials: "same-origin",
          headers: {
            "Content-Type": "application/json",
          },
          body: data,
        })
          .then((response) => response.json())
          .then((json) => {
            console.log(json);
            if (!json.status) {
              return;
            }
            const data = json.data;
            const tbody = document.getElementById("table-body");
            console.log(data);
            let active = `<i class="fa-regular fa-square${data.active?'-check':''}"></i>`;
            tbody.innerHTML += `
                        <tr id="${data.id}">
                            <td>${active}</td>
                            <td>${data.name}</td>
                            <td>${data.role}</td>
                            <td>
                                <button role="button"
                                        class="edit"
                                        data-role="edit"
                                        data-tooltip="Edit"
                                        data-id="${data.id}"
                                        data-name="${data.name}"
                                        data-active="${data.active}"
                                        data-role="${data.role}">
                                  <i class="fa-solid fa-pen"></i>
                                </button>
                            </td>
                            <td>
                                <button role="button"
                                        class="delete"
                                        data-role="delete"
                                        data-tooltip="Delete"
                                        data-id="${data.id}">
                                    <i class="fa-solid fa-trash"></i>
                                </button>
                            </td>
                        </tr>
                        `;
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
