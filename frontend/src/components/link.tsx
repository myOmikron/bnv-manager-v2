import * as Headless from "@headlessui/react";
import React from "react";
import {
    NavLink,
    NavLinkProps,
    Link as ReactRouterLink,
    LinkProps as ReactRouterLinkProps,
} from "react-router-dom";

export const Link = React.forwardRef(function Link(
    props: {
        href: string;
        activeClassName?: (props: {
            isActive: boolean;
            isPending: boolean;
            isTransitioning: boolean;
        }) => string | undefined;
    } & (
        | (Omit<NavLinkProps, "to"> & { asElement: "NavLink" })
        | (Omit<ReactRouterLinkProps, "to"> & { asElement: "Link" })
        | (Omit<ReactRouterLinkProps, "to"> & { asElement: "a" })
    ),
    ref: React.ForwardedRef<HTMLAnchorElement>,
) {
    return (
        <Headless.DataInteractive>
            {props.asElement === "NavLink" ? (
                <NavLink
                    {...props}
                    to={props.href}
                    ref={ref}
                    className={
                        props.activeClassName !== undefined
                            ? props.activeClassName
                            : props.className
                    }
                />
            ) : props.asElement === "Link" ? (
                <ReactRouterLink
                    {...props}
                    to={props.href}
                    ref={ref}
                    className={props.className}
                />
            ) : props.asElement === "a" ? (
                <a
                    {...props}
                    href={props.href}
                    ref={ref}
                    className={props.className}
                />
            ) : undefined}
        </Headless.DataInteractive>
    );
});
