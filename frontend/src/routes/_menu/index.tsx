import { createFileRoute } from "@tanstack/react-router";

/**
 * Props for {@link Dashboard}
 */
export type DashboardProps = {};

/**
 * Dashboard for all users
 */
function Dashboard(props: DashboardProps) {
    return <div>Hello "/"!</div>;
}

export const Route = createFileRoute("/_menu/")({
    component: Dashboard,
});
