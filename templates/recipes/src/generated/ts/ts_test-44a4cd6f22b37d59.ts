
        import _ from "lodash";

        const message: string = "Hello from TypeScript!";
        const kebab = _.kebabCase("Nucleus TypeScript Integration Works");
        
        console.log(message);
        console.log("Lodash Output:", kebab);

        const el = document.getElementById("ts-output");
        if (el) {
            el.innerText = `${message} | ${kebab}`;
            el.style.color = "#4facfe";
        }
    