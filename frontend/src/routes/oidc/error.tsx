import { createFileRoute } from "@tanstack/react-router";

import { useTranslation } from "react-i18next";

/**
 * Props for {@link OidcError}
 */
export type OidcErrorProps = {};

/**
 * Displayable errors while using oidc
 */
export default function OidcError(props: OidcErrorProps) {
    const [t] = useTranslation("oidc-error");
    const [tg] = useTranslation();

    const search = Route.useSearch();

    return <div></div>;
}

/**
 * Search parameters for the oidc error route
 */
type ErrorParams = {
    /** The errors to display */
    error: string;
};

export const Route = createFileRoute("/oidc/error")({
    component: OidcError,
    validateSearch: (search: Record<string, unknown>): ErrorParams => {
        return {
            error: search?.error as string | "/",
        };
    },
    loaderDeps: ({ search: { error } }) => ({ error }),
});
