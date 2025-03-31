import { createFileRoute } from "@tanstack/react-router";
import { UserProvider } from "src/context/user";

function RouteComponent() {
    return (
        <UserProvider>
            <div>Hello "/"!</div>
        </UserProvider>
    );
}

export const Route = createFileRoute("/")({
    component: RouteComponent,
});
