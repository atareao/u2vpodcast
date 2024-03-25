var ready = (callback) => {
    if (document.readyState != "loading") callback();
    else document.addEventListener("DOMContentLoaded", callback);
};

ready(() => {
    const saveButtons = document.getElementsByClassName("save-button");
    if (saveButtons != null) {
        Array.from(saveButtons).forEach((saveButton) => {
        saveButton.addEventListener("click", (event) => {
        console.log(event);
        const article = saveButton.parentNode;
        const data = [];
        Array.from(article.children).forEach((element) => {
            if (element.tagName === "INPUT" || element.tagName === "TEXTAREA") {
                data.push({
                    key: element.name,
                    value:
                        element.type === "checkbox"
                            ? element.checked
                                ? "TRUE"
                                : "FALSE"
                            : element.value,
                });
            }
        });
        if (data.length > 0) {
            console.log(data);
            fetch("/config", {
                method: "POST",
                cache: "no-cache",
                credentials: "same-origin",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
        })
            .then((response) => response.json())
            .then((json) => {
                if (json.result == "ok") {
                    console.log(json);
                    updateInputs(element.children, json.content);
                }
            })
            .catch((err) => console.log("Error", err));
    }
});
});
}
});

function updateInputs(children, data) {
    Array.from(children).forEach((element) => {
        if (element.tagName === "INPUT" || element.tagName === "TEXTAREA") {
            const newValue = getValue(data, element.name);
            if (newValue != null) {
                if (element.type === "checkbox") {
                    element.checked = newValue === "TRUE";
                } else {
                    element.value = newValue;
                }
            }
        }
    });
}

function getValue(data, key) {
    Array.from(data).forEach((item) => {
        if (key == item.key) {
            return item.value;
        }
    });
    return null;
}
