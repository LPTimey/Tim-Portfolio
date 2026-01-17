"use strict";

/**
 * Unendliches Carousel mit Clones links/rechts und Teleport
 */

const carousels = /** @type {NodeListOf<HTMLDivElement>} */ (
    document.querySelectorAll(".carousel")
);

for (const carousel of carousels) {

    const content = /** @type {HTMLUListElement} */ (
        carousel.querySelector(".carousel-content")
    );

    const children = /** @type {HTMLElement[]} */ ([...content.children]);
    const clonesLeft = children.filter(el => el.dataset.cloneIndexLeft !== undefined);
    const clonesRight = children.filter(el => el.dataset.cloneIndexRight !== undefined);
    const imgs = children.filter(el => el.dataset.index !== undefined);

    const back = /** @type {HTMLButtonElement} */ (
        carousel.querySelector(".carousel-button-left")
    );
    const next = /** @type {HTMLButtonElement} */ (
        carousel.querySelector(".carousel-button-right")
    );
    const dots = /** @type {NodeListOf<HTMLLIElement>} */ (
        carousel.querySelectorAll(".carousel-dot")
    );

    const items = imgs.length;

    // Master-Index inklusive Clones
    let fakeCurrent = clonesLeft.length;
    // Index der echten Bilder
    let current = fakeCurrent - clonesLeft.length;

    /**
     * Scrollt zu fakeCurrent
     * @param {boolean} smooth 
     */
    function scrollToCurrent(smooth) {
        const child = content.children[fakeCurrent];
        child.scrollIntoView({
            behavior: smooth ? "smooth" : "instant",
            inline: "start",
            block: "nearest",
        });
    }

    /**
     * Update Carousel Klassen & Dots
     */
    function updateCarousel(scroll = true) {
        // Ableiten current
        current = fakeCurrent - clonesLeft.length;

        // Klassen für echte Bilder setzen
        imgs.forEach((li) => {
            li.classList.toggle("active", Number(li.dataset.index) === current);
        });

        // Scrollen
        if (scroll) scrollToCurrent(true);

        // Dots
        dots.forEach((dot) => {
            dot.classList.toggle("active", Number(dot.dataset.for) === current);
        });

        // dataset aktualisieren
        carousel.dataset.current = String(current);
    }

    /**
     * Teleport, wenn fakeCurrent auf Clone landet
     */
    function teleportIfClone() {
        // Clone links
        if (fakeCurrent < clonesLeft.length) {
            fakeCurrent += items; // zum echten Item am Ende
            scrollToCurrent(false);
        }
        // Clone rechts
        else if (fakeCurrent >= clonesLeft.length + items) {
            fakeCurrent -= items; // zum echten Item am Anfang
            scrollToCurrent(false);
        }

        // immer current ableiten
        current = fakeCurrent - clonesLeft.length;
        updateCarousel(false)
    }

    // Buttons
    next.addEventListener("click", (e) => {
        e.preventDefault();
        fakeCurrent += 1;
        updateCarousel(true);
        setTimeout(teleportIfClone, 300); // Dauer des smooth scrolls
    });

    back.addEventListener("click", (e) => {
        e.preventDefault();
        fakeCurrent -= 1;
        updateCarousel(true);
        setTimeout(teleportIfClone, 300);
    });

    // Dots
    dots.forEach((dot) => {
        dot.addEventListener("click", () => {
            const index = Number(dot.dataset.for);
            if (!Number.isNaN(index)) {
                fakeCurrent = clonesLeft.length + index;
                updateCarousel(true);
                setTimeout(teleportIfClone, 300);
            }
        });
    });

    // Initiales Rendern
    updateCarousel();
}
