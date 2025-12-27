(function () {
  const showElement = (el) => el.style.removeProperty("display");

  const hideElement = (el) => el.style.display = "none";

  const toggleElement = (el) => {
    if (el.style.display === "none") {
      showElement(el);
    } else {
      hideElement(el);
    }
  };

  const highlightCodeSnippets = () => hljs.highlightAll();

  document.addEventListener("DOMContentLoaded", function() {
    const mainMenu = document.querySelector("#main-nav-menu");
    const mainDropdown = document.querySelector("#main-nav-dropdown");

    mainMenu.addEventListener("click", () => {
      toggleElement(mainDropdown);
    });

    mainDropdown.addEventListener("mouseleave", () => {
      hideElement(mainDropdown);
    });

    highlightCodeSnippets();
  });

})();
