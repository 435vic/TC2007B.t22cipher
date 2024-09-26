import { invoke } from "@tauri-apps/api/tauri";

// let greetInputEl: HTMLInputElement | null;
// let greetMsgEl: HTMLElement | null;

// async function greet() {
//   if (greetMsgEl && greetInputEl) {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsgEl.textContent = await invoke("greet", {
//       name: greetInputEl.value,
//     });
//   }
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
//     e.preventDefault();
//     greet();
//   });
// });

// let inputFormatButtons = document.querySelectorAll(".input-format");
// let inputFormat = "b64";

// for (let btn of inputFormatButtons) {
//     btn.addEventListener("click", () => {
//         inputFormat = btn.getAttribute("data-format") || "b64";
//         inputFormatButtons.forEach((btn) => {
//             btn.classList.remove("selected");
//         });
//         btn.classList.add("selected");
//     });
// }

let inputArea: HTMLTextAreaElement = document.querySelector("#input")!;
let outputArea: HTMLTextAreaElement = document.querySelector("#output")!;
let keyInput: HTMLInputElement = document.querySelector("#encryption-key")!;
let errorArea: HTMLDivElement = document.querySelector("#error-msg")!;
let encryptButton = document.querySelector("#encrypt-btn")!;
let decryptButton = document.querySelector("#decrypt-btn")!;
let randomKeyButton = document.querySelector("#random-key")!;

encryptButton.addEventListener("click", async () => {
    let input = inputArea.value;
    outputArea.value = "Encrypting...";
    try {
        let output = await invoke("encrypt", {
            plaintext: input,
            key: keyInput.value,
        });

        // Convert the array of numbers to base64
        const uint8Array = new Uint8Array(output as number[]);
        const binaryString = String.fromCharCode.apply(null, uint8Array as any);
        const base64Output = btoa(binaryString);

        outputArea.value = base64Output;
    } catch (e) {
        errorArea.innerHTML = e as string;
    }
    // outputArea.value = output;
});

randomKeyButton.addEventListener("click", async () => {
    // Random 16 character all lowercase key
    // no numeric characters
    keyInput.value = Array.from({length: 16}, () => String.fromCharCode(Math.floor(Math.random() * 26) + 97)).join('')
});

decryptButton.addEventListener("click", async () => {
    let input = inputArea.value;
    errorArea.innerHTML = "";
    try {
        let output = await invoke("decrypt", {
            ciphertext: input,
            key: keyInput.value,
        });

        const uint8Array = new Uint8Array(output as number[]);
        const decodedString = new TextDecoder().decode(uint8Array);
        outputArea.value = decodedString;
    } catch (e) {
        errorArea.innerHTML = e as string;
    }
    // outputArea.value = output;
});
