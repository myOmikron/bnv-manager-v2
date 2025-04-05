import { createLazyFileRoute, Outlet } from "@tanstack/react-router";

import React, { useContext } from "react";
import { useTranslation } from "react-i18next";
import USER_CONTEXT, { UserProvider } from "src/context/user";
import {
    Sidebar,
    SidebarBody,
    SidebarDivider,
    SidebarFooter,
    SidebarHeader,
    SidebarHeading,
    SidebarItem,
    SidebarLabel,
    SidebarSection
} from "src/components/base/sidebar";
import { SidebarLayout } from "src/components/base/sidebar-layout";
import { Navbar, NavbarItem, NavbarLabel, NavbarSpacer } from "src/components/base/navbar";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { ArrowRightStartOnRectangleIcon, ChevronUpIcon, UserGroupIcon, UserIcon } from "@heroicons/react/20/solid";
import { Api } from "src/api/api";

/**
 * The properties for {@link Menu}
 */
export type MenuProps = {};

/**
 * The menu of the application
 */
function Menu(props: MenuProps) {
    const [t] = useTranslation("menu");

    const ctx = useContext(USER_CONTEXT);

    return (
        <SidebarLayout
            sidebar={
                <Sidebar>
                    <SidebarHeader>
                        <span className={"text-xl font-bold text-black dark:text-white"}>BNV Manager</span>
                    </SidebarHeader>

                    <SidebarBody>
                        <SidebarSection>
                            <SidebarHeading>{t("heading.user-settings")}</SidebarHeading>
                        </SidebarSection>

                        {ctx.user.admin && (
                            <>
                                <SidebarDivider />
                                <SidebarSection>
                                    <SidebarHeading>{t("heading.admin-settings")}</SidebarHeading>

                                    <SidebarItem href={"/a/clubs"}>
                                        <UserGroupIcon />
                                        <SidebarLabel>{t("button.club-overview")}</SidebarLabel>
                                    </SidebarItem>
                                </SidebarSection>
                            </>
                        )}
                    </SidebarBody>
                    <SidebarFooter className={"max-lg:hidden"}>
                        <Dropdown>
                            <DropdownButton as={SidebarItem}>
                                <span className="grid h-10 w-full min-w-0 grid-cols-[30px_1fr_20px] items-center gap-3">
                                    <UserIcon />
                                    <span className="min-w-0">
                                        <span
                                            className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {ctx.user.display_name}
                                        </span>
                                        <span
                                            className="block truncate text-xs/5 font-normal text-zinc-500 dark:text-zinc-400">
                                            {ctx.user.username}
                                        </span>
                                    </span>
                                    <ChevronUpIcon />
                                </span>
                            </DropdownButton>
                            <DropdownMenu anchor={"top end"}>
                                <DropdownItem href={"/profile"}>
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
                            <DropdownItem href={"/profile"}>
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
            <Outlet />
        </SidebarLayout>
    );
}

export const Route = createLazyFileRoute("/_menu")({
    // eslint-disable-next-line
    component: () => (
        <UserProvider>
            <Menu />
        </UserProvider>
    )
});
