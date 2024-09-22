import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_admin/a/dashboard")({
    component: () => <div>Hello /_admin/a/!</div>,
});
