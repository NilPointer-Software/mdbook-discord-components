import init, { parse } from "./pkg/mdbook_discord_components_wasm.js";

const wasm = await init();

const main = document.getElementsByTagName("main")[0];
const input = document.getElementById("input");
const button = document.getElementById("run");

button.addEventListener("click", () => {
    if (!wasm || !main || !input)
        return;
    let inputText = input.value;
    main.innerHTML = parse("yaml", inputText);
});

input.addEventListener("change", () => {
    if (!localStorage)
        return;
    localStorage.setItem("textarea", input.value);
})

if (localStorage)
    input.value = localStorage.getItem("textarea") || "";
