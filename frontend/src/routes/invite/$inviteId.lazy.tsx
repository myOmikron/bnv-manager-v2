import { createLazyFileRoute } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";

/**
 * The properties for {@link Invite}
 */
export type InviteProps = {};

/**
 * Invite
 */
export default function Invite(props: InviteProps) {
    const [t] = useTranslation();
    const [tI] = useTranslation("invite");

    const { inviteId } = Route.useParams();

    return <div></div>;
}

export const Route = createLazyFileRoute("/invite/$inviteId")({
    component: Invite,
});
