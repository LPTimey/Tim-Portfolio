"use strict";

/******************************\
 *                            *
 *          Dialogs           *
 *                            *
\******************************/
//#region dialogs

let dialogs = document.querySelectorAll("dialog");
let dialog_buttons = document.querySelectorAll("[dialog-target]");
let modal_buttons = document.querySelectorAll("[modal-target]");
let popover_buttons = document.querySelectorAll("[popover-target]");

/**
 * 
 * @param {HTMLElement} button 
 * @param {string} typeStr 
 * @param {(targetElement: HTMLDialogElement)=>void} openFn 
 * @returns {void}
 */
function activate_button(button, typeStr, openFn) {
    const target = button.getAttribute(typeStr);
    if (target == null) {
        console.error(`no ${typeStr}`)
        return;
    }
    const targetElement = document.querySelector(target);
    if (targetElement == null || !(targetElement instanceof HTMLDialogElement)) {
        console.error(`${target} is not a valid ${typeStr}`)
        return;
    }
    button.addEventListener("click", event => {
        if (!targetElement.open) {
            openFn(targetElement)
        }
        else {
            targetElement.close()
        }
    })
}
dialog_buttons.forEach(button => {
    activate_button(button, "dialog-target", (targetElement) => targetElement.show());
});
modal_buttons.forEach(button => {
    activate_button(button, "modal-target", (targetElement) => targetElement.showModal());
});
popover_buttons.forEach(button => {
    activate_button(button, "popover-target", (targetElement) => targetElement.showPopover({source:button}));
});

dialogs.forEach(dialog => {
    /* add close on click outside */
    dialog.addEventListener("click", event => {
        const dialogDimensions = dialog.getBoundingClientRect()
        if (
            event.clientX < dialogDimensions.left ||
            event.clientX > dialogDimensions.right ||
            event.clientY < dialogDimensions.top ||
            event.clientY > dialogDimensions.bottom
        ) {
            dialog.close()
        }
    });
    /* add close on esc */
    dialog.addEventListener("keyup", event => {
        if (event.key === "Escape") {
            dialog.close()
        }
    });
    /* stop the rest of the page from scrolling when open */
    dialog.addEventListener("toggle", event => {
        if (dialog.open) {
            // Erhalte die Höhe des Scrollbalkens (falls vorhanden)
            const scrollbarWidth = window.innerWidth - document.documentElement.clientWidth;

            // Blockiere das Scrollen und vermeide Layout-Sprünge
            document.body.style.overflow = 'hidden';
            // Hinzufügen des Platzes für den Scrollbalken
            document.body.style.paddingRight = `${scrollbarWidth}px`;
        } else {
            // Entferne das Blockieren des Scrollens und den zusätzlichen Padding
            document.body.style.overflow = '';
            // Entferne den Platz für den Scrollbalken
            document.body.style.paddingRight = '';
        }
    })
});

//#endregion dialogs

/******************************\
 *                            *
 *       LocalStorage         *
 *                            *
\******************************/
//#region LocalStorage

// window.localStorage.setItem("theme","dark");


//#endregion LocalStorage


/******************************\
 *                            *
 *           Utils            *
 *                            *
\******************************/
//#region Utils

/**
 * 
 * @param {string} hex 
 * @param {number} [alpha=100] 
 * @returns 
 */
export function hexToRGB_CSS(hex, alpha = 100) {
    return `rgb(from ${hex} r g b / ${alpha}%)`;
}

//#endregion Utils