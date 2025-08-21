"use strict";

const carousels = /** @type {NodeListOf<HTMLDivElement>} */ (
    document.querySelectorAll(".carousel")
);

for (const carousel of carousels) {
    const content = /** @type {HTMLUListElement} */ (
        carousel.querySelector(".carousel-content")
    );
    const buttons = /** @type {HTMLMenuElement} */ (
        carousel.querySelector(".carousel-buttons")
    );
    const back = /** @type {HTMLButtonElement} */ (
        buttons.children[0].querySelector(".carousel-button")
    );
    const next = /** @type {HTMLButtonElement} */ (
        buttons.children[1].querySelector(".carousel-button")
    );
    const dots = /** @type {NodeListOf<HTMLLIElement>} */ (
        carousel.querySelectorAll(".carousel-dot")
    );

    let current = 0;
    const items = content.children.length - 1;

    back.addEventListener("click", (e) => {
        e.preventDefault();
        current = (current - 1 + items) % items;
        updateCarousel();
    });

    next.addEventListener("click", (e) => {
        e.preventDefault();
        current = (current + 1) % items;
        updateCarousel();
    });

    dots.forEach((dot) => {
        dot.addEventListener("click", () => {
            const index = Number(dot.dataset.for);
            if (!Number.isNaN(index)) {
                current = index;
                updateCarousel();
            }
        });
    });

    updateCarousel(false);

    function updateCarousel(scroll = true) {
        const child = content.children[current];

        (/** @type {HTMLElement[]} */([...content.children])).forEach((li, i) => {
            li.classList.toggle("active", Number(li.dataset.index) === current);
        });

        if (scroll) {
            child.scrollIntoView({
                behavior: "smooth",
                inline: "start",
                block: "nearest"
            });
        }

        dots.forEach((dot, i) => {
            dot.classList.toggle("active", Number(dot.dataset.for) === current);
        });

        carousel.dataset.current = String(current);
    }
}
