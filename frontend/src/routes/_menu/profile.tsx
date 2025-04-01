import { createFileRoute } from "@tanstack/react-router";

function RouteComponent() {
    return <div>Hello "/_menu/profile"!</div>;
}

export const Route = createFileRoute("/_menu/profile")({
    component: RouteComponent,
});
