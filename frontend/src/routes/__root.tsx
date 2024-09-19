import { createRootRoute, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
    /**
     * The root route that will be rendered when accessing every other route
     *
     * @returns JSX
     */
    component: () => <Outlet />,
});
