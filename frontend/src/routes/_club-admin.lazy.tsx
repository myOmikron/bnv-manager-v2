import { createLazyFileRoute, Outlet } from "@tanstack/react-router";

import React from "react";
import { Api } from "src/api/api";
import { StackedLayout } from "src/components/base/stacked-layout";
import { Navbar, NavbarDivider, NavbarItem, NavbarSection, NavbarSpacer } from "src/components/base/navbar";
import { useTranslation } from "react-i18next";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { Sidebar, SidebarBody, SidebarFooter, SidebarHeader, SidebarItem } from "src/components/base/sidebar";
import { Avatar } from "src/components/base/avatar";
import USER_CONTEXT, { UserProvider } from "src/context/user";

/**
 * The properties for {@link ClubAdminMenu}
 */
export type ClubAdminMenuProps = {};

/**
 * The menu for the user
 */
export default function ClubAdminMenu(props: ClubAdminMenuProps) {
    const [t] = useTranslation();
    const [tM] = useTranslation("menu");

    const userContext = React.useContext(USER_CONTEXT);

    return (
        <StackedLayout
            navbar={
                <Navbar className={"max-lg:hidden"}>
                    <NavbarSection>
                        <NavbarItem className={"w-max min-w-max"}>
                            <span className={"text-nowrap"}>BNV Manager</span>
                        </NavbarItem>
                    </NavbarSection>
                    <NavbarDivider />
                    <NavbarSection>
                        <NavbarItem href={"/ca/dashboard"}>{tM("button.dashboard")}</NavbarItem>
                    </NavbarSection>
                    <NavbarSpacer />
                    <NavbarSection>
                        <Dropdown>
                            <DropdownButton as={NavbarItem}>
                                <div className={"flex min-w-0 items-center gap-3 text-right"}>
                                    <span className="min-w-0">
                                        <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {userContext.user.display_name}
                                        </span>
                                    </span>
                                    <Avatar
                                        className={"size-8"}
                                        square={true}
                                        initials={userContext.user.display_name
                                            .split(" ")
                                            .map((a) => {
                                                return a[0];
                                            })
                                            .join("")}
                                    />
                                </div>
                            </DropdownButton>
                            <DropdownMenu className="min-w-52">
                                <DropdownItem href={"/ca/profile/general"}>
                                    <DropdownLabel>{tM("button.profile")}</DropdownLabel>
                                </DropdownItem>
                                <DropdownItem
                                    onClick={() => {
                                        Api.auth.logout().then(() => {
                                            userContext.reset();
                                        });
                                    }}
                                >
                                    <DropdownLabel>{tM("button.sign-out")}</DropdownLabel>
                                </DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    </NavbarSection>
                </Navbar>
            }
            sidebar={
                <Sidebar>
                    <SidebarHeader></SidebarHeader>
                    <SidebarBody>
                        <SidebarItem href={"/ca/dashboard"}>{tM("button.dashboard")}</SidebarItem>
                    </SidebarBody>
                    <SidebarFooter>
                        <Dropdown>
                            <DropdownButton as={SidebarItem}>
                                <div className={"flex min-w-0 items-center gap-3 text-right"}>
                                    <Avatar
                                        className={"size-8"}
                                        square={true}
                                        initials={userContext.user.display_name
                                            .split(" ")
                                            .map((a) => {
                                                return a[0];
                                            })
                                            .join("")}
                                    />
                                    <span className="min-w-0">
                                        <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {userContext.user.display_name}
                                        </span>
                                    </span>
                                </div>
                            </DropdownButton>
                            <DropdownMenu className="min-w-52" anchor={"top start"}>
                                <DropdownItem href={"/ca/profile/general"}>
                                    <DropdownLabel>{tM("button.profile")}</DropdownLabel>
                                </DropdownItem>
                                <DropdownItem
                                    onClick={() => {
                                        Api.auth.logout().then(() => {
                                            userContext.reset();
                                        });
                                    }}
                                >
                                    <DropdownLabel>{tM("button.sign-out")}</DropdownLabel>
                                </DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    </SidebarFooter>
                </Sidebar>
            }
        >
            <Outlet />
        </StackedLayout>
    );
}

export const Route = createLazyFileRoute("/_club-admin")({
    component: () => (
        <UserProvider>
            <ClubAdminMenu />
        </UserProvider>
    ),
});
