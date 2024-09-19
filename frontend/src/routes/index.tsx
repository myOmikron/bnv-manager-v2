import { createFileRoute, Navigate } from "@tanstack/react-router";

import React from "react";
import USER_CONTEXT, { UserProvider } from "src/context/user";
import i18n from "src/i18n";

/**
 * The properties for {@link RoleGuard}
 */
export type RoleGuardProps = {};

/**
 * A guard for distributing on sub-routes based on the user role
 */
export default function RoleGuard(props: RoleGuardProps) {
    const userContext = React.useContext(USER_CONTEXT);

    // TODO: Permissions

    return <Navigate to={"/u/mail"} />;
}

export const Route = createFileRoute("/")({
    /**
     * Wrapper around the index route to forward to the correct sub-path
     *
     * @returns JSX
     */
    component: () => (
        <UserProvider t={i18n.t}>
            <RoleGuard />
        </UserProvider>
    ),
});
