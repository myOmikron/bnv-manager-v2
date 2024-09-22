import { createFileRoute } from "@tanstack/react-router";
import { GeneralProfile } from "src/components/profile";

export const Route = createFileRoute("/_user/u/_profile/profile/general")({
    component: GeneralProfile,
});
