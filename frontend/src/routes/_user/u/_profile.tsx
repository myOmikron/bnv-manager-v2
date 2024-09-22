import { createFileRoute } from "@tanstack/react-router";
import { ProfileLayout } from "src/components/profile";

export const Route = createFileRoute("/_user/u/_profile")({
    component: ProfileLayout,
});
