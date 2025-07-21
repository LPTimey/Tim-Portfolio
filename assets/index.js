"use strict";

const timelines = /** @type {NodeListOf<HTMLElement>} */(document.querySelectorAll(".timeline"));

timelines.forEach(timeline => {
    timeline.addEventListener("mouseenter", ev => {
        timeline.setAttribute("show-slider", "")
    });
    timeline.addEventListener("mouseleave", ev => {
        const percentage =
            Number(timeline.style.getPropertyValue("--slider-top").replaceAll("px", ""));
        if (!percentage || isNaN(percentage) || percentage <= 50) {
            timeline.style.setProperty("--slider-top", `0px`);
        } else {
            const timelineRect = timeline.getBoundingClientRect();
            timeline.style.setProperty("--slider-top", `${timelineRect.height}px`);
        }
        timeline.removeAttribute("show-slider")
    });
    const items = timeline.querySelectorAll(".timeline-item");
    items.forEach(item => {
        item.addEventListener("mouseenter", ev => {
            const timelineRect = timeline.getBoundingClientRect();
            const itemRect = item.getBoundingClientRect();

            // Y-Position relativ zur Timeline
            const itemCenter = itemRect.top + itemRect.height / 2;
            const relativeY = itemCenter - timelineRect.top;

            // Prozentwert berechnen
            // const percentage = (relativeY / timelineRect.height) * 100;

            // Setzen der CSS-Variable
            timeline.style.setProperty("--slider-top", `${relativeY}px`);
        });
    })
})