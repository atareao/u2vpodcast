// create a variable for the login form
const form = document.querySelector(".loginForm");
// if the form exists, run the class
if (form) {
    // setup the fields we want to validate, we only have two but you can add others
    const fields = ["username", "password"];
    // run the class
    const validator = new Login(form, fields);
}
const auth = new Auth();

document.querySelector(".logout").addEventListener("click", (e) => {
    auth.logOut();
});

