import { createFileRoute, Navigate } from "@tanstack/react-router";

import React from "react";
import { UserProvider } from "src/context/user";

/**
 * The properties for {@link RoleGuard}
 */
export type RoleGuardProps = {};

/**
 * A guard for distributing on sub-routes based on the user role
 */
export default function RoleGuard(props: RoleGuardProps) {
    return <Navigate to={"/u/mail"} />;
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
