import React from "react";
import { Api } from "../api/api";
import CONSOLE from "../utils/console";
import Login from "../components/login";
import { parseError, StatusCode } from "../api/error";
import { Navigate } from "@tanstack/react-router";
import { Me, RequiredError, ResponseError } from "src/api/generated";
import { toast } from "react-toastify";

/** The global {@link UserProvider} instance */
let USER_PROVIDER: UserProvider | null = null;

/** Data provided by the {@link USER_CONTEXT} */
export type UserContext = {
    /** The currently logged-in user */
    user: Me;

    /** Reload the user's information */
    reset: () => void;
};

/** {@link React.Context} to access {@link FullUser user information} */
const USER_CONTEXT = React.createContext<UserContext>({
    user: {
        uuid: "",
        username: "",
        display_name: "",
        permissions: {
            admin: false,
            club_user: [],
            club_admin: [],
        },
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
    user: Me | "unauthenticated" | "loading";
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
    fetchUser = async () => {
        // Guard against a lot of calls
        if (this.fetching) return;
        this.fetching = true;

        this.setState({ user: "loading" });

        try {
            const me = await Api.me.get();
            this.setState({ user: me });
        } catch (e) {
            let msg;
            if (e instanceof ResponseError) {
                const err = await parseError(e.response);
                if (err.status_code === StatusCode.Unauthenticated) {
                    this.setState({ user: "unauthenticated" });
                }
            } else if (e instanceof RequiredError) {
                CONSOLE.error(e);
                msg = "The server's response didn't match the spec";
            } else {
                CONSOLE.error("Unknown error occurred:", e);
                msg = "Unknown error occurred";
            }
            toast.error(msg);
            throw e;
        } finally {
            // Clear guard against a lot of calls
            this.fetching = false;
        }
    };

    /**
     * Hook when the component mounts
     */
    componentDidMount() {
        this.fetchUser().then();

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
                        <Login onLogin={() => this.fetchUser()} />
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
