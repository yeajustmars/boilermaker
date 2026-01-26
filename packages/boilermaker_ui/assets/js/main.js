(function () {
  const hideElement = (el) => {
    el.classList.add("hidden");
  };

  const showElement = (el) => {
    el.classList.remove("hidden");
  }

  const toggleElement = (el) => {
    if (el.classList.contains("hidden")) {
      showElement(el);
    } else {
      hideElement(el);
    }
  }

  document.addEventListener("DOMContentLoaded", function() {
    // ________________________________________ Dropdown menus
    const dropdownContainers = document.querySelectorAll(".dropdown-container");
    dropdownContainers.forEach(container => {
      let button = container.querySelector(".dropdown-button");
      let menu = container.querySelector(".dropdown-menu");

      button.addEventListener("click", (event) => {
        let menu = event.target.closest(".dropdown-container").querySelector(".dropdown-menu");
        toggleElement(menu);
      });

      menu.addEventListener("mouseleave", () => {
        menu.classList.add("hidden");
      });
    });

    // ________________________________________ Collapsibles
    const collapsibleButtons = document.querySelectorAll(".collapsible-button");
    collapsibleButtons.forEach(button => {
      button.addEventListener("click", (event) => {
        let content = event.target.closest(".collapsible").querySelector(".collapsible-content");
        toggleElement(content);
      })
    });

    // ________________________________________ Code
    hljs.highlightAll();
  });

})();
