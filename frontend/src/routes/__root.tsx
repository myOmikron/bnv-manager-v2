import { createRootRoute, ErrorComponentProps, Outlet, useRouter } from "@tanstack/react-router";
import { Suspense } from "react";
import { Dialog, DialogActions, DialogDescription, DialogTitle } from "src/components/base/dialog.tsx";
import { Button } from "src/components/base/button.tsx";
import { SidebarLayout } from "src/components/base/sidebar-layout.tsx";
import { Sidebar } from "src/components/base/sidebar.tsx";
import { Navbar } from "src/components/base/navbar.tsx";

function RootComponent() {
    return (
        <SidebarLayout sidebar={<Sidebar></Sidebar>} navbar={<Navbar></Navbar>}>
            <Outlet />
        </SidebarLayout>
    );
}

function ErrorComponent(props: ErrorComponentProps) {
    const router = useRouter();

    return (
        <Suspense>
            <Dialog open={true} onClose={() => {}}>
                <DialogTitle>{"Schade!"}</DialogTitle>
                <DialogDescription>{props.error.stack}</DialogDescription>
                <DialogActions>
                    <Button onClick={() => router.invalidate({ sync: true })}>Okay</Button>
                </DialogActions>
            </Dialog>
        </Suspense>
    );
}

export const Route = createRootRoute({
    component: RootComponent,
    errorComponent: (props) => <ErrorComponent {...props} />,
});
