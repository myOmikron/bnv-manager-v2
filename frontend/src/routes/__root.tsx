import { createRootRoute, Outlet } from "@tanstack/react-router";
import { AccountProvider } from "src/context/account";

export const Route = createRootRoute({
    component: () => (
        <AccountProvider>
            <Outlet />
        </AccountProvider>
    ),
});
