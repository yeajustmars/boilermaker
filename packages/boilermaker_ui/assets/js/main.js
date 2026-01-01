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
  });

})();
