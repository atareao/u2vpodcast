class Auth {
     // setup the class and hide the body by default
    constructor() {
        document.querySelector("body").style.display = "none";
        const auth = localStorage.getItem("jwt_token");
        this.validateAuth(auth);
    }
    // check to see if the localStorage item passed to the function is valid and set
    validateAuth(auth) {
        if (auth != 1) {
            window.location.replace("/login");
        } else {
            document.querySelector("body").style.display = "block";
        }
    }
    // will remove the localStorage item and redirect to login  screen
    logOut() {
        localStorage.removeItem("jwt_token");
        window.location.replace("/login");
    }
}

