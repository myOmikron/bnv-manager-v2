import React, { useContext } from "react";
import { Outlet } from "react-router";
import {
    Navbar,
    NavbarDivider,
    NavbarItem,
    NavbarSection,
    NavbarSpacer,
} from "../../components/navbar";
import {
    Sidebar,
    SidebarBody,
    SidebarFooter,
    SidebarHeader,
    SidebarItem,
} from "../../components/sidebar";
import { StackedLayout } from "../../components/stacked-layout";
import {
    Dropdown,
    DropdownButton,
    DropdownItem,
    DropdownMenu,
} from "../../components/dropdown";
import { Avatar } from "../../components/avatar";
import USER_CONTEXT from "../../context/user";
import { Api } from "../../api/api";
import MonkeyLogo from "./monkey-logo.svg?react";
import { ROUTER } from "../../router";
import { useTranslation } from "react-i18next";

/**
 * The properties for the {@link Menu}
 */
export type MenuProps = {};

/**
 * The main menu component
 */
export default function Menu(props: MenuProps) {
    const [t] = useTranslation("menu");
    const userContext = useContext(USER_CONTEXT);

    return (
        <StackedLayout
            navbar={
                <Navbar className={"max-lg:hidden"}>
                    <NavbarSection>
                        <NavbarItem className={"w-max min-w-max"}>
                            <MonkeyLogo className={"size-10"} />
                            <span className={"text-nowrap"}>BNV Manager</span>
                        </NavbarItem>
                    </NavbarSection>
                    <NavbarDivider />
                    <NavbarSection>
                        <NavbarItem href={ROUTER.MAIL.path}>Mail</NavbarItem>
                        <NavbarItem href={ROUTER.WEBSITES.path}>
                            {t("Websites")}
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
                                <DropdownItem
                                    href={ROUTER.PROFILE.path}
                                    asElement={"Link"}
                                >
                                    {t("Profile")}
                                </DropdownItem>
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
                        <SidebarItem
                            href={ROUTER.MAIL.path}
                            asElement={"NavLink"}
                        >
                            {t("Mail")}
                        </SidebarItem>
                        <SidebarItem
                            href={ROUTER.WEBSITES.path}
                            asElement={"NavLink"}
                        >
                            {t("Websites")}
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
                                <DropdownItem
                                    href={ROUTER.PROFILE.path}
                                    asElement={"Link"}
                                >
                                    {t("Profile")}
                                </DropdownItem>
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
