import { createFileRoute, Navigate } from "@tanstack/react-router";

import React from "react";
import USER_CONTEXT, { UserProvider } from "src/context/user";

/**
 * The properties for {@link RoleGuard}
 */
export type RoleGuardProps = {};

/**
 * A guard for distributing on sub-routes based on the user role
 */
export default function RoleGuard(props: RoleGuardProps) {
    const { user } = React.useContext(USER_CONTEXT);

    switch (user.role) {
        case "Administrator":
            return <Navigate to={"/a/dashboard"} />;
        case "User":
            return <Navigate to={"/u/mail"} />;
        case "ClubAdmin":
            return <Navigate to={"/ca/dashboard"} />;
    }
}

export const Route = createFileRoute("/")({
    /**
     * Wrapper around the index route to forward to the correct sub-path
     *
     * @returns JSX
     */
    component: () => (
        <UserProvider>
            <RoleGuard />
        </UserProvider>
    ),
});
