import { createLazyFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";


/**
 * The properties for {@link Invite}
 */
export type InviteProps = {};

/**
 * Invite form to create an account
 */
export default function Invite(props: InviteProps) {
    const [t] = useTranslation();

    return <div></div>;
}


export const Route = createLazyFileRoute("/invites/$inviteId")({
    component: Invite
});