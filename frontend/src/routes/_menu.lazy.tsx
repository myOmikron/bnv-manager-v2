import { createLazyFileRoute, Outlet } from "@tanstack/react-router";

import React, { Suspense, useContext } from "react";
import { useTranslation } from "react-i18next";
import {
    Sidebar,
    SidebarBody,
    SidebarDivider,
    SidebarFooter,
    SidebarHeader,
    SidebarHeading,
    SidebarItem,
    SidebarLabel,
    SidebarSection,
} from "src/components/base/sidebar";
import { SidebarLayout } from "src/components/base/sidebar-layout";
import { Navbar, NavbarItem, NavbarLabel, NavbarSpacer } from "src/components/base/navbar";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import {
    ArrowRightStartOnRectangleIcon,
    ChevronUpIcon,
    FingerPrintIcon,
    PresentationChartBarIcon,
    UserGroupIcon,
    UserIcon,
    UserPlusIcon,
} from "@heroicons/react/20/solid";
import { Api } from "src/api/api";
import Logo from "src/assets/bnv.svg?react";
import ACCOUNT_CONTEXT, { AccountProvider } from "src/context/account";

/**
 * The properties for {@link Menu}
 */
export type MenuProps = {};

/**
 * The menu of the application
 */
function Menu(props: MenuProps) {
    const [t] = useTranslation("menu");

    const ctx = useContext(ACCOUNT_CONTEXT);

    return (
        <SidebarLayout
            sidebar={
                <Sidebar>
                    <SidebarHeader>
                        <Logo className={"w-full p-5 dark:text-white"} />
                    </SidebarHeader>

                    <SidebarBody>
                        {ctx.user.roles.admins.map((admin) => (
                            <React.Fragment key={`ca-${admin.club_uuid}`}>
                                <SidebarSection>
                                    <SidebarHeading className={"whitespace-pre-line"}>
                                        {t("heading.club-admin", { club: admin.club_name })}
                                    </SidebarHeading>
                                    <SidebarItem href={"/ca/$clubId"} params={{ clubId: admin.club_uuid }}>
                                        <PresentationChartBarIcon />
                                        <SidebarLabel>{t("button.club-dashboard")}</SidebarLabel>
                                    </SidebarItem>
                                </SidebarSection>
                                <SidebarDivider />
                            </React.Fragment>
                        ))}

                        {ctx.user.roles.super_admin && (
                            <>
                                <SidebarSection>
                                    <SidebarHeading>{t("heading.admin-settings")}</SidebarHeading>

                                    <SidebarItem href={"/a/clubs"}>
                                        <UserGroupIcon />
                                        <SidebarLabel>{t("button.club-overview")}</SidebarLabel>
                                    </SidebarItem>

                                    <SidebarItem href={"/a/admins"}>
                                        <UserPlusIcon />
                                        <SidebarLabel>{t("button.admin-overview")}</SidebarLabel>
                                    </SidebarItem>

                                    <SidebarItem href={"/a/oidc"}>
                                        <FingerPrintIcon />
                                        <SidebarLabel>{t("button.oidc-provider")}</SidebarLabel>
                                    </SidebarItem>
                                </SidebarSection>
                            </>
                        )}
                    </SidebarBody>
                    <SidebarFooter className={"max-lg:hidden"}>
                        <Dropdown>
                            <DropdownButton as={SidebarItem}>
                                <span className="grid h-10 w-full min-w-0 grid-cols-[1fr_20px] items-center gap-3">
                                    <span className="min-w-0">
                                        <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {ctx.user.display_name}
                                        </span>
                                        <span className="block truncate text-xs/5 font-normal text-zinc-500 dark:text-zinc-400">
                                            {ctx.user.username}
                                        </span>
                                    </span>
                                    <ChevronUpIcon />
                                </span>
                            </DropdownButton>
                            <DropdownMenu anchor={"top end"}>
                                <DropdownItem href={"/profile/general"}>
                                    <UserIcon />
                                    <DropdownLabel>{t("button.profile")}</DropdownLabel>
                                </DropdownItem>
                                <DropdownItem
                                    onClick={async () => {
                                        await Api.auth.logout();
                                        ctx.reset();
                                    }}
                                >
                                    <ArrowRightStartOnRectangleIcon />
                                    <DropdownLabel>{t("button.sign-out")}</DropdownLabel>
                                </DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    </SidebarFooter>
                </Sidebar>
            }
            navbar={
                <Navbar>
                    <NavbarSpacer />
                    <Dropdown>
                        <DropdownButton as={NavbarItem}>
                            <UserIcon />
                            <NavbarLabel className={"grow"}>{ctx.user.display_name}</NavbarLabel>
                            <ChevronUpIcon />
                        </DropdownButton>
                        <DropdownMenu anchor={"top end"}>
                            <DropdownItem href={"/profile/general"}>
                                <UserIcon />
                                <DropdownLabel>{t("button.profile")}</DropdownLabel>
                            </DropdownItem>
                            <DropdownItem
                                onClick={async () => {
                                    await Api.auth.logout();
                                    ctx.reset();
                                }}
                            >
                                <ArrowRightStartOnRectangleIcon />
                                <DropdownLabel>{t("button.sign-out")}</DropdownLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                </Navbar>
            }
        >
            <Suspense>
                <Outlet />
            </Suspense>
        </SidebarLayout>
    );
}

export const Route = createLazyFileRoute("/_menu")({
    component: () => (
        <AccountProvider>
            <Menu />
        </AccountProvider>
    ),
});
