import { createFileRoute } from "@tanstack/react-router";
import { ProfileLayout } from "src/components/profile";

export const Route = createFileRoute("/_admin/a/_profile")({
    component: ProfileLayout,
});
