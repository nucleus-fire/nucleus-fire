<n:view title="TypeScript Test">
    <div class="container">
        <h1>TypeScript Integration Test</h1>
        <p>Check the console for a message from Lodash.</p>
        <div id="ts-output">Waiting for TS...</div>
    </div>

    <script lang="ts">
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
    </script>

    <style>
        .container { padding: 2rem; font-family: sans-serif; color: white; background: #222; }
    </style>
</n:view>
