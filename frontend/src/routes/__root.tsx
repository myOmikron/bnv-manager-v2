import { createRootRoute, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
    // eslint-disable-next-line
    component: () => <Outlet />,
});
