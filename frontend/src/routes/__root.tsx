import * as React from "react";
import { createRootRoute, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
    component: RootComponent
});

function RootComponent() {
    return (
        <React.Fragment>
            <div>Hello "__root"!</div>
            <Outlet />
        </React.Fragment>
    );
}
