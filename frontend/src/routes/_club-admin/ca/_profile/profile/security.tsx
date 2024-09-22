import { createFileRoute } from "@tanstack/react-router";
import SecurityProfile from "src/components/profile";

export const Route = createFileRoute("/_club-admin/ca/_profile/profile/security")({
    component: SecurityProfile,
});
