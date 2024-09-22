import React from "react";
import { toast } from "react-toastify";
import { Api } from "src/api/api";
import { FullUser } from "src/api/generated/";
import CONSOLE from "src/utils/console";
import Login from "src/components/login";
import { ApiError, StatusCode } from "src/api/error";
import { Navigate } from "@tanstack/react-router";
import WS from "src/api/ws";

/** The global {@link UserProvider} instance */
let USER_PROVIDER: UserProvider | null = null;

/** Data provided by the {@link USER_CONTEXT} */
export type UserContext = {
    /** The currently logged-in user */
    user: FullUser;

    /** Reload the user's information */
    reset: () => void;
};

/** {@link React.Context} to access {@link FullUser user information} */
const USER_CONTEXT = React.createContext<UserContext>({
    user: {
        uuid: "",
        username: "",
        display_name: "",
        created_at: "",
        last_login: "",
        preferred_lang: "",
        role: "User",
    },

    /**
     * Reset the user's information
     */
    reset: () => {},
});
USER_CONTEXT.displayName = "UserContext";
export default USER_CONTEXT;

/**
 * The properties of the user provider
 */
type UserProviderProps = {
    /** The children of the properties */
    children: React.ReactNode | Array<React.ReactNode>;
};

/**
 * The state of the user provider
 */
type UserProviderState = {
    /** The user */
    user: FullUser | "unauthenticated" | "loading";
};

/**
 * Component for managing and providing the {@link UserContext}
 *
 * This is a **singleton** only use at most **one** instance in your application.
 */
export class UserProvider extends React.Component<UserProviderProps, UserProviderState> {
    state: UserProviderState = { user: "loading" };

    fetching: boolean = false;

    /**
     * Fetch the user
     */
    fetchUser = () => {
        // Guard against a lot of calls
        if (this.fetching) return;
        this.fetching = true;

        this.setState({ user: "loading" });

        Api.users.getMe().then((result) => {
            result.match(
                (user) => {
                    WS.connect(`${window.location.origin.replace("http", "ws")}/api/frontend/v1/common/ws/ws`);
                    window.localStorage.setItem("username", user.username);
                    this.setState({ user });
                },
                (error) => {
                    switch (error.status_code) {
                        case StatusCode.Unauthenticated:
                            this.setState({ user: "unauthenticated" });
                            break;
                        default:
                            toast.error(error.message);
                            break;
                    }
                },
            );
            // Clear guard against a lot of calls
            this.fetching = false;
        });
    };

    /**
     * Hook when the component mounts
     */
    componentDidMount() {
        this.fetchUser();

        // Register as global singleton
        // eslint-disable-next-line @typescript-eslint/no-this-alias
        if (USER_PROVIDER === null) USER_PROVIDER = this;
        else if (USER_PROVIDER === this) CONSOLE.error("UserProvider did mount twice");
        else CONSOLE.error("Two instances of UserProvider are used");
    }

    /**
     * Hook when the component will unmount
     */
    componentWillUnmount() {
        // Deregister as global singleton
        if (USER_PROVIDER === this) USER_PROVIDER = null;
        else if (USER_PROVIDER === null) CONSOLE.error("UserProvider instance did unmount twice");
        else CONSOLE.error("Two instances of UserProvider are used");
    }

    /**
     * The render function
     *
     * @returns The JSX component
     */
    render() {
        switch (this.state.user) {
            case "loading":
                return <div></div>;
            case "unauthenticated":
                return (
                    <>
                        <Navigate to="/" />
                        <Login
                            onLogin={() => {
                                this.fetchUser();
                            }}
                        />
                    </>
                );
            default:
                return (
                    <USER_CONTEXT.Provider
                        value={{
                            user: this.state.user,
                            reset: this.fetchUser,
                        }}
                    >
                        {this.props.children}
                    </USER_CONTEXT.Provider>
                );
        }
    }
}

/**
 * Inspect an error and handle the {@link StatusCode.Unauthenticated} status code by requiring the user to log in again.
 *
 * @param error {@link ApiError} to inspect for {@link StatusCode.Unauthenticated}
 */
export function inspectError(error: ApiError) {
    switch (error.status_code) {
        case StatusCode.Unauthenticated:
            if (USER_PROVIDER !== null) USER_PROVIDER.setState({ user: "unauthenticated" });
            else CONSOLE.warn("inspectError has been called without a UserProvider");
            break;
        default:
            break;
    }
}
