import init, { parse } from "./pkg/mdbook_discord_components_wasm.js";

const wasm = await init();

const main = document.getElementsByTagName("main")[0];
const input = document.getElementById("input");
const button = document.getElementById("run");

button.addEventListener("click", () => {
    if (!wasm || !main || !input)
        return;
    let inputText = input.value;
    try {
        let out = parse("yaml", inputText);
        console.info("Parser produced", `\n${out}`);
        main.innerHTML = out;
    } catch (err) {
        main.innerHTML = `<p class="error">${err}</p>`;
    }
});

input.addEventListener("change", () => {
    if (!localStorage)
        return;
    localStorage.setItem("textarea", input.value);
})

if (localStorage)
    input.value = localStorage.getItem("textarea") || "";
