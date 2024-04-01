const go = new Go();
WebAssembly.instantiateStreaming(fetch("main.wasm"), go.importObject).then(result => {
    go.run(result.instance);
    
    document.getElementById("countButton").addEventListener("click", () => {
        const text = document.getElementById("input").value;
        const counts = globalThis.countChars(text);
        document.getElementById("countWithSpaces").textContent = counts.withSpaces;
        document.getElementById("countWithoutSpaces").textContent = counts.withoutSpaces;
        document.getElementById("lineCount").textContent = counts.lines;
        document.getElementById("paragraphCount").textContent = counts.paragraphs;
    });

    document.getElementById("resetButton").addEventListener("click", () => {
        document.getElementById("input").value = "";
        document.getElementById("countWithSpaces").textContent = "0";
        document.getElementById("countWithoutSpaces").textContent = "0";
        document.getElementById("lineCount").textContent = "0";
        document.getElementById("paragraphCount").textContent = "0";
    });
});
