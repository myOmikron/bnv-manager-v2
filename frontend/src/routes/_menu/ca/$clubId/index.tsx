import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_menu/ca/$clubId/")({
    component: RouteComponent,
});

function RouteComponent() {
    return <div>Hello "/_menu/ca/$clubId/"!</div>;
}
