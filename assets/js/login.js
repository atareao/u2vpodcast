class Login {
    constructor(form, fields) {
        this.form = form;
        this.fields = fields;
        this.validateonSubmit();
    }

    validateonSubmit() {
        let self = this; // setup calls to the "this" values of the class described in the constructor

        // add a "submit" event listener to the form
        this.form.addEventListener("submit", (e) => {
            // remove default functionality 
            e.preventDefault();
            var error = 0;
            // loop through the fields and check them against a function for validation
            self.fields.forEach((field) => {
                const input = document.querySelector(`#${field}`);
                if (self.validateFields(input) == false) {
                    // if a field does not validate, auto-increment our error integer
                    error++;
                }
            });
            // if everything validates, error will be 0 and can continue
            if (error == 0) {
                //do login api here or in this case, just submit the form and set a localStorage item
                localStorage.setItem("jwt_token", 1);
                this.form.submit();
            }
        });
    }

    validateFields(field) {
        // remove any whitespace and check to see if the field is blank, if so return false
        if (field.value.trim() === "") {
            // set the status based on the field, the field label, and if it is an error message
            this.setStatus(
                field,
                `${field.previousElementSibling.innerText} cannot be blank`,
                "error"
            );
            return false;
        } else {
            // if the field is not blank, check to see if it is password
            if (field.type == "password") {
                // if it is a password, check to see if it meets our minimum character requirement
                if (field.value.length < 8) {
                    // set the status based on the field, the field label, and if it is an error message
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} must be at least 8 characters`,
                        "error"
                    );
                    return false;
                } else {
                    // set the status based on the field without text and return a success message
                    this.setStatus(field, null, "success");
                    return true;
                }
            }else if (field.type == "email"){
                if (/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(field.value)) {
                    this.setStatus(field, null, "success");
                    return true;
                }else{
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} is not a valid address`,
                        "error"
                    );
                    return false;

                }
            } else {
                // set the status based on the field without text and return a success message
                this.setStatus(field, null, "success");
                return true;
            }
        }
    }

    setStatus(field, message, status) {
        // create variable to hold message
        const errorMessage = field.parentElement.querySelector(".alert");
        if(errorMessage) {
            if (status == "success") {
                errorMessage.innerText = "";
                errorMessage.classList.remove("alert-danger");
                errorMessage.classList.remove("alert-warning");
                errorMessage.classList.add("alert-success");
                errorMessage.hidden = true;
            } else if (status == "error") {
                errorMessage.innerText = message;
                errorMessage.classList.remove("alert-warning");
                errorMessage.classList.remove("alert-success");
                errorMessage.classList.add("alert-danger");
                errorMessage.hidden = false;
            }
        }
    }
}

document.addEventListener("readystatechange", (event) => {
    if (document.readyState === "complete") {
        const form = document.getElementById("loginForm");
        if (form) {
            const fields = ["email", "password"];
            const validator = new Login(form, fields);
        }
    }
})

