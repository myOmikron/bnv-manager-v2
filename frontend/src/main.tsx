import React from "react";
import ReactDOM from "react-dom/client";
import "src/index.css";
import "react-toastify/dist/ReactToastify.css";
import { ToastContainer } from "react-toastify";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { LoggingSwitch } from "src/utils/console";

// import i18n so it can be initialized
import "src/i18n";

// Import the generated route tree
import { routeTree } from "src/routeTree.gen";

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

ReactDOM.createRoot(document.getElementById("root")!).render(
    <>
        <LoggingSwitch />
        <ToastContainer toastClassName={"toast-message"} closeOnClick={true} />
        <RouterProvider router={router} />
    </>,
);
