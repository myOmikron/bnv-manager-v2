import * as Headless from "@headlessui/react";
import React from "react";
import { LinkProps as RouterLinkProps, Link as RouterLink } from "@tanstack/react-router";

/**
 * The properties of the Link
 */
export type LinkProps = (
    | {
          /** Render props of tanstacks link */
          render: (state: { isActive: boolean }) => React.ReactNode;
          children?: never;
      }
    | {
          /** The children to render */
          children: React.ReactNode;
          render?: never;
      }
) & {
    /** Custom href */
    href: RouterLinkProps["to"];
    /** The classname to set */
    className?: string;
} & Omit<RouterLinkProps, "to" | "children">;

export const Link = React.forwardRef(function Link(props: LinkProps, ref: React.ForwardedRef<HTMLAnchorElement>) {
    const { href, params, children, render, ...other } = props;

    return (
        <Headless.DataInteractive>
            <RouterLink to={href} params={params} {...other} ref={ref} children={children ? children : render} />
        </Headless.DataInteractive>
    );
});