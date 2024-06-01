import React from "react";
import { Outlet } from "react-router";
import { Navbar, NavbarItem, NavbarSection } from "../../components/navbar";
import { Sidebar } from "../../components/sidebar";
import { StackedLayout } from "../../components/stacked-layout";

/**
 * The properties for the {@link Menu}
 */
export type MenuProps = {};

/**
 * The main menu component
 */
export default function Menu(props: MenuProps) {
    return (
        <StackedLayout
            navbar={
                <Navbar>
                    <NavbarSection>
                        <NavbarItem>Foobar</NavbarItem>
                    </NavbarSection>
                </Navbar>
            }
            sidebar={<Sidebar></Sidebar>}
        >
            <Outlet />
        </StackedLayout>
    );
}
