import {
    createBrowserRouter,
    createRoutesFromElements,
    Route,
} from "react-router-dom";
import React, { Suspense } from "react";
import Error from "./views/error";
import { UserProviderWrapper } from "./context/user";
import Mail from "./views/mail/mail";
import Websites from "./views/websites/websites";
import Profile from "./views/profile/profile";
import WebsiteConfiguration from "./views/websites/website-configuration/website-configuration";

/**
 * An element in the router
 */
class PathElement {
    readonly path;
    readonly element;

    /**
     * The constructor for a path element
     *
     * @param path The path for the route
     * @param element The element that should be rendered
     */
    constructor(path: string, element: React.ReactElement) {
        this.path = path;
        this.element = element;
    }

    getRoute() {
        return (
            <Route
                key={this.path}
                path={this.path}
                element={this.element}
            ></Route>
        );
    }
}

export const ROUTER = {
    HOME: new PathElement("/", <div></div>),
    MAIL: new PathElement("/mail", <Mail />),
    WEBSITES: new PathElement("/websites", <Websites />),
    WEBSITE_CONFIGURATION: new PathElement(
        "/websites/:websiteUuid",
        <WebsiteConfiguration />,
    ),
    PROFILE: new PathElement("/profiles", <Profile />),
};

export const router = createBrowserRouter(
    createRoutesFromElements(
        <Route
            key={"outer"}
            element={
                <Suspense fallback={<Error />}>
                    <UserProviderWrapper />
                </Suspense>
            }
            errorElement={<Error />}
        >
            {Object.values(ROUTER).map((x) => x.getRoute())}
        </Route>,
    ),
);
