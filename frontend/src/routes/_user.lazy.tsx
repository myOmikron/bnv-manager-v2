import { createLazyFileRoute, Outlet } from "@tanstack/react-router";

import React from "react";
import { Api } from "src/api/api";
import { StackedLayout } from "src/components/base/stacked-layout";
import {
    Navbar,
    NavbarDivider,
    NavbarItem,
    NavbarSection,
    NavbarSpacer,
} from "src/components/base/navbar";
import { useTranslation } from "react-i18next";
import {
    Dropdown,
    DropdownButton,
    DropdownItem,
    DropdownMenu,
} from "src/components/base/dropdown";
import {
    Sidebar,
    SidebarBody,
    SidebarFooter,
    SidebarHeader,
    SidebarItem,
} from "src/components/base/sidebar";
import { Avatar } from "src/components/base/avatar";
import USER_CONTEXT, { UserProvider } from "src/context/user";
import i18n from "src/i18n";

/**
 * The properties for {@link UserMenu}
 */
export type UserMenuProps = {};

/**
 * The menu for the user
 */
export default function UserMenu(props: UserMenuProps) {
    const [t] = useTranslation();
    const [tMenu] = useTranslation("menu");

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
                        <NavbarItem href={"/u/mail"}>Mail</NavbarItem>
                        <NavbarItem href={"/u/websites"}>
                            {tMenu("button.websites")}
                        </NavbarItem>
                    </NavbarSection>
                    <NavbarSpacer />
                    <NavbarSection>
                        <Dropdown>
                            <DropdownButton as={NavbarItem}>
                                <div
                                    className={
                                        "flex min-w-0 items-center gap-3 text-right"
                                    }
                                >
                                    <span className="min-w-0">
                                        <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {userContext.user.displayName}
                                        </span>
                                    </span>
                                    <Avatar
                                        className={"size-8"}
                                        square={true}
                                        initials={userContext.user.displayName
                                            .split(" ")
                                            .map((a) => {
                                                return a[0];
                                            })
                                            .join("")}
                                    />
                                </div>
                            </DropdownButton>
                            <DropdownMenu className="min-w-52">
                                <DropdownItem>{t("Profile")}</DropdownItem>
                                <DropdownItem
                                    onClick={() => {
                                        Api.auth.logout().then(() => {
                                            userContext.reset();
                                        });
                                    }}
                                >
                                    {t("Sign out")}
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
                        <SidebarItem href={"/u/mail"}>{t("Mail")}</SidebarItem>
                        <SidebarItem href={"/u/websites"}>
                            {tMenu("button.websites")}
                        </SidebarItem>
                    </SidebarBody>
                    <SidebarFooter>
                        <Dropdown>
                            <DropdownButton as={SidebarItem}>
                                <div
                                    className={
                                        "flex min-w-0 items-center gap-3 text-right"
                                    }
                                >
                                    <Avatar
                                        className={"size-8"}
                                        square={true}
                                        initials={userContext.user.displayName
                                            .split(" ")
                                            .map((a) => {
                                                return a[0];
                                            })
                                            .join("")}
                                    />
                                    <span className="min-w-0">
                                        <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                            {userContext.user.displayName}
                                        </span>
                                    </span>
                                </div>
                            </DropdownButton>
                            <DropdownMenu
                                className="min-w-52"
                                anchor={"top start"}
                            >
                                <DropdownItem>{t("Profile")}</DropdownItem>
                                <DropdownItem
                                    onClick={() => {
                                        Api.auth.logout().then(() => {
                                            userContext.reset();
                                        });
                                    }}
                                >
                                    {t("Sign out")}
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

export const Route = createLazyFileRoute("/_user")({
    component: () => (
        <UserProvider t={i18n.t}>
            <UserMenu />
        </UserProvider>
    ),
});
