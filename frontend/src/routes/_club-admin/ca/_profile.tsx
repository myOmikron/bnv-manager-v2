import { createFileRoute } from "@tanstack/react-router";
import { ProfileLayout } from "src/components/profile";

export const Route = createFileRoute("/_club-admin/ca/_profile")({
    component: ProfileLayout,
});
