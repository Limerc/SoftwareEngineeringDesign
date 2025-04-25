document.getElementById("run").addEventListener("click", async () => {
    const code = document.getElementById("code").value;
    const response = await fetch("/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code }),
    });
    const result = await response.json();
    document.getElementById("output").textContent =
        `STDOUT:\n${result.stdout}\n\nSTDERR:\n${result.stderr}`;
});
