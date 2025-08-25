import { createFileRoute, Navigate } from "@tanstack/react-router";

/**
 * Props for {@link LinkAuthentication}
 */
export type LinkAuthenticationProps = {};

/**
 * Link to oidc authentication
 */
export default function LinkAuthentication(props: LinkAuthenticationProps) {
    const search = Route.useSearch();

    return <Navigate to={"/oidc/auth"} search={search} />;
}

/**
 * Search parameters for the oidc authentication route
 */
type SearchParams = {
    /** Uri to redirect to after successful authentication */
    redirect_url: string;
};

export const Route = createFileRoute("/links/oidc/auth")({
    component: LinkAuthentication,
    // eslint-disable-next-line
    validateSearch: (search: Record<string, unknown>): SearchParams => {
        return {
            redirect_url: search?.redirect_url as string | "/",
        };
    },
    // eslint-disable-next-line
    loaderDeps: ({ search: { redirect_url } }) => ({ redirect_url }),
});
