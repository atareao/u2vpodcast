document.addEventListener("readystatechange", () => {
    if (document.readyState === "complete") {
        const form = document.getElementById("loginForm");
        if (form) {
            console.log("form")
            const fields = ["email", "password"];
            const validator = new Login(form, fields);
        }
        const auth = new Auth();
        const logout = document.querySelector(".logout");
        if (logout){
            logout.addEventListener("click", () => {
                auth.logOut();
            });
        }
    }
})

