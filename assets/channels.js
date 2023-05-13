"use strict";

const isOpenClass = "modal-is-open";
const openingClass = "modal-is-opening";
const closingClass = "modal-is-closing";
const animationDuration = 400;
let visibleModal = null;

const toggleModal = (e) => {
        e.preventDefault();
        e = document.getElementById(e.currentTarget.getAttribute("data-target"));
        (void 0 !== e && null != e && isModalOpen(e) ? closeModal : openModal)(e);
};
const createNewChannel = (e) => {
    console.log("Create new channel");
    const form = new FormData(document.getElementById(e.currentTarget.getAttribute("data-form")));
    console.log(form);
    const data = {};
    form.forEach((value, key) => data[key] = value);
    console.log(data);
    const payload = JSON.stringify(data);
    console.log(payload);
    fetch("/api/v1/channels", {
        method: "POST",
        mode: "cors",
        cache: "no-cache",
        credentials: "same-origin",
        headers: {
            "Content-Type": "application/json"
        },
        body: payload,
    }).then(function (response) {
        if (response.ok) {
            return response.json();
        }
        return Promise.reject(response);
    }).then(function (data) {
        console.log(data);
    }).catch(function (error) {
        console.warn(error);
    });
    toggleModal(e);
};
const isModalOpen = (e) => !(!e.hasAttribute("open") || "false" == e.getAttribute("open"));
const openModal = (e) => {
    isScrollbarVisible() && document.documentElement.style.setProperty("--scrollbar-width", getScrollbarWidth() + "px");
    document.documentElement.classList.add(isOpenClass, openingClass);
    setTimeout(() => {
        visibleModal = e;
        document.documentElement.classList.remove(openingClass);
    }, animationDuration),
    e.setAttribute("open", !0);
};
const closeModal = (e) => {
    visibleModal = null;
    document.documentElement.classList.add(closingClass),
    setTimeout(() => {
        document.documentElement.classList.remove(closingClass, isOpenClass);
        document.documentElement.style.removeProperty("--scrollbar-width");
        e.removeAttribute("open");
    }, animationDuration);
};
const getScrollbarWidth =
        (document.addEventListener("click", (e) => {
            null == visibleModal || visibleModal.querySelector("article").contains(e.target) || closeModal(visibleModal);
        }),
        document.addEventListener("keydown", (e) => {
            "Escape" === e.key && null != visibleModal && closeModal(visibleModal);
        }),
        () => {
            var e = document.createElement("div"),
                t = ((e.style.visibility = "hidden"), (e.style.overflow = "scroll"), (e.style.msOverflowStyle = "scrollbar"), document.body.appendChild(e), document.createElement("div")),
                t = (e.appendChild(t), e.offsetWidth - t.offsetWidth);
            return e.parentNode.removeChild(e), t;
        });
const isScrollbarVisible = () => document.body.scrollHeight > screen.height;

