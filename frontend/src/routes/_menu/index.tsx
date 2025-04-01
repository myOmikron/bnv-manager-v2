import { createFileRoute } from "@tanstack/react-router";

function RouteComponent() {
    return <div>Hello "/"!</div>;
}

export const Route = createFileRoute("/_menu/")({
    component: RouteComponent,
});
