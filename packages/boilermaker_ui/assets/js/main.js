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

    // ________________________________________ Dropdowns
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
      });
    });

    // ________________________________________ Tabs
    const tabButtons = document.querySelectorAll(".tab-button");
    tabButtons.forEach(button => {
      button.addEventListener("click", (event) => {
        let btn = event.target;
        let target = btn.dataset.target;
        let tabButtons = event.target.closest(".tab-buttons").querySelectorAll(".tab-button");
        let tabs = event
          .target
          .closest(".tabs")
          .querySelector(".tab-panes")
          .querySelectorAll(".tab");

        tabButtons.forEach(btn => {
          btn.classList.remove("active");
        });
        btn.classList.add("active");

        tabs.forEach(tab => {
          if (tab.dataset.tab === target) {
            tab.classList.add("active");
          } else {
            tab.classList.remove("active");
          }
        });
      });
    });

    // ________________________________________ Code
    hljs.highlightAll();
  });

})();
