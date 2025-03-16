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

dialogs.forEach(dialog => {
    /* add close on click outside */
    dialog.addEventListener("click", e => {
        const dialogDimensions = dialog.getBoundingClientRect()
        if (
            e.clientX < dialogDimensions.left ||
            e.clientX > dialogDimensions.right ||
            e.clientY < dialogDimensions.top ||
            e.clientY > dialogDimensions.bottom
        ) {
            dialog.close()
        }
    });
    /* add close on esc */
    dialog.addEventListener("keyup", e => {
        if (e.key === "Escape") {
            dialog.close()
        }
    });
});
dialog_buttons.forEach(button => {
    const target = button.getAttribute("dialog-target");
    if (target == null) {
        console.log("no dialog-target")
        return;
    }
    const targetElement = document.querySelector(target);
    if (targetElement == null || !(targetElement instanceof HTMLDialogElement)) {
        console.log(`${target} is not a valid dialog-target`)
        return;
    }
    button.addEventListener("click", e => {
        if (!targetElement.open) {
            targetElement.show()
        }
        else {
            targetElement.close()
        }
    })
});
modal_buttons.forEach(button => {
    console.log(button);
    const target = button.getAttribute("modal-target");
    if (target == null) {
        console.log("no modal-target")
        return;
    }
    const targetElement = document.querySelector(target);
    if (targetElement == null || !(targetElement instanceof HTMLDialogElement)) {
        console.log(`${target} is not a valid modal-target`)
        return;
    }
    button.addEventListener("click", e => {
        console.log(e)
        console.log(targetElement)
        if (!targetElement.open) {
            targetElement.showModal()
        }
        else {
            targetElement.close()
        }
    })
    console.log(button);
});
popover_buttons.forEach(button => {
    const target = button.getAttribute("popover-target");
    if (target == null) {
        console.log("no popover-target")
        return;
    }
    const targetElement = document.querySelector(target);
    if (targetElement == null || !(targetElement instanceof HTMLDialogElement)) {
        console.log(`${target} is not a valid popover-target`)
        return;
    }
    button.addEventListener("click", e => {
        if (!targetElement.open) {
            targetElement.showPopover()
        }
        else {
            targetElement.close()
        }
    })
});

//#endregion dialogs