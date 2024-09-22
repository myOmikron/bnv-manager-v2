import { createFileRoute } from "@tanstack/react-router";
import SecurityProfile from "src/components/profile";

export const Route = createFileRoute("/_admin/a/_profile/profile/security")({
    component: SecurityProfile,
});
