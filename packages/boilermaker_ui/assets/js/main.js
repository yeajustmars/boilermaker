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

  document.addEventListener("DOMContentLoaded", function() {
    // ________________________________________ Main dropdown navigation
    const mainMenu = document.querySelector("#nav-main-dropdown-button");
    const mainDropdown = document.querySelector("#nav-main-dropdown-menu");

    hideElement(mainDropdown);

    mainMenu.addEventListener("click", () => {
      console.log("Toggling main dropdown");
      toggleElement(mainDropdown);
    });

    mainDropdown.addEventListener("mouseleave", () => {
      hideElement(mainDropdown);
    });

    // ________________________________________ Collapsibles
    const collapsibleButtons = document.querySelectorAll(".collapsible-link");
    collapsibleButtons.forEach(button => {
      button.addEventListener("click", (event) => {
        let content = event.target.closest(".collapsible").querySelector(".collapsible-content");
        if (content.classList.contains("hidden")) {
          content.classList.remove("hidden");
        } else {
          content.classList.add("hidden");
        }
      })
    });

    // ________________________________________ Code
    hljs.highlightAll();
  });

})();
