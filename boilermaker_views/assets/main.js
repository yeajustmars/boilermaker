(function () {
  const showElement = (el) => {
      el.style.removeProperty("display");
  };

  const hideElement = (el) => {
      el.style.display = "none";
  };

  const toggleDisplay = (el) => {
    if (el.style.display === "none") {
      showElement(el);
    } else {
      hideElement(el);
    }
  };

  document.addEventListener("DOMContentLoaded", function() {
    const mainMenu = document.querySelector("#main-nav-menu");
    const mainDropdown = document.querySelector("#main-nav-dropdown");

    mainMenu.addEventListener("click", () => {
      toggleDisplay(mainDropdown);
    });

    mainDropdown.addEventListener("mouseleave", () => {
      hideElement(mainDropdown);
    });

  });

})();
