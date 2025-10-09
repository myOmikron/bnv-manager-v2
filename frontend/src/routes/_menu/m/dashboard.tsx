import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import ACCOUNT_CONTEXT from "src/context/account";
import MailcowLogo from "src/assets/mailcow.svg?react";
import { Subheading } from "src/components/base/heading";

/**
 * The properties for {@link MemberDashboard}
 */
export type MemberDashboardProps = {};

/**
 * Dashboard for members
 */
export default function MemberDashboard(props: MemberDashboardProps) {
    const [t] = useTranslation("member-dashboard");

    const ctx = React.useContext(ACCOUNT_CONTEXT);

    return (
        <HeadingLayout heading={t("heading.hello", { name: ctx.account.display_name })}>
            <Subheading>{t("heading.quick-access")}</Subheading>
            <div className={"grid lg:grid-cols-3"}>
                <a
                    href={""}
                    className={
                        "flex items-center gap-12 rounded border bg-zinc-800 p-5 duration-75 hover:border-orange-500 dark:border-zinc-700"
                    }
                >
                    <MailcowLogo className={"size-10"} />
                    <Subheading>Mail - TODO</Subheading>
                </a>
            </div>
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_menu/m/dashboard")({
    component: MemberDashboard,
});
