import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import HeadingLayout from "src/components/base/heading-layout";
import { useTranslation } from "react-i18next";

/**
 * The properties for {@link UserOverview}
 */
export type UserOverviewProps = {};

/**
 * The overview over all users
 */
export default function UserOverview(props: UserOverviewProps) {
    const [t] = useTranslation();
    const [tU] = useTranslation("admin-user-overview");

    return <HeadingLayout heading={tU("heading.user-overview")}></HeadingLayout>;
}

export const Route = createFileRoute("/_admin/a/users/")({
    component: UserOverview,
});
