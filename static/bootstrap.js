import init from "../pkg/r_os.js";

async function run() {
  await init();
  const ta = document.getElementById("console");
  ta.focus();
  // simple UX: append newline when Enter pressed so shell sees it.
  ta.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      // allow newline but also keep focus
      // nothing special: kernel inspects textarea content to detect newline
    }

    ta.selectionStart = ta.selectionEnd = ta.value.length;
    ta.scrollTop = ta.scrollHeight;
  });

  ta.addEventListener("mousedown", (e) => {
    e.preventDefault();
    ta.selectionStart = ta.selectionEnd = ta.value.length;
  });
}
run();