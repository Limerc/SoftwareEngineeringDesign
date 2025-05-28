// 设置 API 地址
const API_BASE = "http://139.59.103.151:8000"; // 后端 URL

let editor;

// 初始化编辑器
window.addEventListener("DOMContentLoaded", () => {
    try {
        // 暂时禁用 CodeMirror 编辑器
        // editor = CodeMirror.fromTextArea(document.getElementById("code-editor"), {
        //     lineNumbers: true,
        //     mode: "rust",
        //     theme: "default",
        //     matchBrackets: true,
        //     indentUnit: 4,
        //     tabSize: 4
        // });
        console.log("CodeMirror initialization skipped.");
    } catch (error) {
        console.error("Error initializing CodeMirror:", error);
        alert("Failed to initialize editor.");
    }

    // 加载题目列表
    fetchProblems();
});

// 获取题目列表
async function fetchProblems() {
    try {
        const response = await fetch(`${API_BASE}/problems`);
        if (!response.ok) {
            throw new Error(`Failed to fetch problems: ${response.statusText}`);
        }

        const problems = await response.json();

        const problemsList = document.getElementById("problems-list");
        problemsList.innerHTML = "";  // 清空问题列表
        problems.forEach((problem) => {
            const li = document.createElement("li");
            li.textContent = problem.title;
            li.addEventListener("click", () => fetchProblemDetails(problem.id));
            problemsList.appendChild(li);
        });
    } catch (error) {
        console.error("Error fetching problems:", error);
        alert("Failed to load problems. Please try again later.");
    }
}

// 获取题目详情
async function fetchProblemDetails(problemId) {
    try {
        const response = await fetch(`${API_BASE}/problems/${problemId}`);
        if (!response.ok) {
            throw new Error(`Failed to fetch problem details: ${response.statusText}`);
        }
        const problem = await response.json();

        document.getElementById("problem-title").textContent = problem.title;
        document.getElementById("problem-description").textContent = problem.description;
        document.getElementById("problem-examples").textContent = problem.examples;

        // 显示题目详情和代码编辑器
        document.getElementById("problem-details-container").classList.add("fade-in", "show");
        document.getElementById("code-editor-container").style.display = "block";
        document.getElementById("result-container").style.display = "block";

        // 清空编辑器和结果区域
        document.getElementById("code-editor").value = ""; // 使用普通文本框
        document.getElementById("result").textContent = "";

        document.getElementById("submit-code").dataset.problemId = problemId;
    } catch (error) {
        console.error("Error fetching problem details:", error);
        alert("Failed to load problem details. Please try again later.");
    }
}

// 提交代码
document.getElementById("submit-code").addEventListener("click", async () => {
    const code = document.getElementById("code-editor").value;
    const problemId = document.getElementById("submit-code").dataset.problemId;
    const resultBox = document.getElementById("result");
    const resultIcon = document.getElementById("result-icon");
    const submitBtn = document.getElementById("submit-code");

    if (!problemId) {
        alert("Please select a problem first!");
        return;
    }

    // 禁用提交按钮，显示加载状态
    submitBtn.disabled = true;
    submitBtn.textContent = "Judging...";
    resultBox.textContent = "Running...";
    resultIcon.style.backgroundColor = "gray"; // 重置图标颜色

    try {
        const response = await fetch(`${API_BASE}/judge`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ problem_id: parseInt(problemId), code }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error: ${response.status}`);
        }

        const result = await response.json();

        // 根据后端返回值设置状态
        if (result.success && result.message.includes("All test cases passed")){
            resultBox.style.color = "green"; // 绿色
            resultBox.textContent = "ALL PASS";
            resultIcon.style.backgroundColor = "green"; // 图标绿色
        } else if (!result.success && result.message.includes("Test case failed")) {
            resultBox.style.color = "yellow"; // 黄色
            resultBox.textContent = "Part PASS";
            resultIcon.style.backgroundColor = "yellow"; // 图标黄色
        } else if (!result.success && result.message.includes("Runtime error")) {
            resultBox.style.color = "red"; // 红色
            resultBox.textContent = "Failure: Runtime Error";
            resultIcon.style.backgroundColor = "red"; // 图标红色
        } else {
            resultBox.style.color = "red"; // 红色
            resultBox.textContent = `Failure: ${result.message}`;
            resultIcon.style.backgroundColor = "red"; // 图标红色
        }
    } catch (error) {
        console.error("Error submitting code:", error);
        resultBox.style.color = "red"; // 红色
        resultBox.textContent = "Error occurred: " + (error.message || "Unknown error");
        resultIcon.style.backgroundColor = "red"; // 图标红色
    } finally {
        submitBtn.disabled = false;
        submitBtn.textContent = "Submit Code";
        document.getElementById("result-container").scrollIntoView({ behavior: "smooth" });
    }
});