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

    // ________________________________________ Docs
    const docLinks = document.querySelectorAll(".doc a");

    if (docLinks.length > 0) {
      docLinks.forEach(link => {
        let href = link.getAttribute("href");
        if (href && !href.startsWith("/docs/") && !href.startsWith("#")) {
          link.setAttribute('target', '_blank');
        }
      });
    }

    const docHeaders = document.querySelectorAll(".doc h1, .doc h2, .doc h3");

    const docSectionStyle = {
      'H1': "doc-link-h1",
      'H2': "doc-link-h2",
      'H3': "doc-link-h3"
    };

    if (docHeaders.length > 0) {
      const docSectionNav = document.getElementById("doc-sections");
      const template = document.getElementById("doc-section-template");

      docHeaders.forEach((header, index) => {
        header.id = `doc-section-${index}`;

        let clone = document.importNode(template.content, true);

        let link = clone.querySelector("li a");
        link.href = `#${header.id}`;
        link.classList.add(docSectionStyle[header.tagName]);
        link.textContent = header.textContent;

        docSectionNav.appendChild(clone);
      });

      const docSectionLinks = docSectionNav.querySelectorAll("a");

      window.addEventListener('scroll', () => {
        let currentSection = '';

        docHeaders.forEach(section => {
            const sectionTop = section.getBoundingClientRect().top;
            if (sectionTop <= 100) {
                currentSection = section.id;
            }
        });

        docSectionLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === `#${currentSection}`) {
                link.classList.add('active');
            }
        });
      });
    }

    // ________________________________________ Code
    hljs.highlightAll();

  });
})();
