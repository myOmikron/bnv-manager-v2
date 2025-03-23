import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api.tsx";
import React from "react";

/**
 * The properties for {@link Invite}
 */
export type InviteProps = {};

/**
 * Invite form to create an account
 */
export default function Invite(props: InviteProps) {
    const [t] = useTranslation("invite");

    const data = Route.useLoaderData();

    return <div></div>;
}

export const Route = createFileRoute("/invites/$inviteId")({
    component: Invite,
    // eslint-disable-next-line
    loader: async ({ params: { inviteId } }) => await Api.invites.get(inviteId),
});
