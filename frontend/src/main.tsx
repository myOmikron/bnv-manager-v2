import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { RouterProvider } from "react-router";
import { router } from "./router";
import { LoggingSwitch } from "./utils/console";
import "react-toastify/dist/ReactToastify.css";
import { ToastContainer } from "react-toastify";

import "./i18n";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <LoggingSwitch />
        <ToastContainer toastClassName={"toast-message"} closeOnClick={true} />
        <RouterProvider router={router} />
    </React.StrictMode>,
);
