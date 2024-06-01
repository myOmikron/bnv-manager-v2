import React from "react";
import { toast } from "react-toastify";
import { Api } from "../../api/api";

/**
 * The properties of the login view
 */
type LoginProps = {
    /** The function that should be called on a successful sign-in */
    onLogin(): void;
};

/**
 * The login view
 */
export default function Login(props: LoginProps) {
    const { onLogin } = props;
    const [username, setUsername] = React.useState<string>("");
    const [password, setPassword] = React.useState<string>("");

    const performLogin = () => {
        Api.auth.login(username, password).then((res) =>
            res.match(
                () => {
                    toast.success("Signed in");
                    onLogin();
                },
                (err) => toast.error(err.message),
            ),
        );
    };

    return (
        <form
            method="post"
            onSubmit={(e) => {
                e.preventDefault();
                performLogin();
            }}
        ></form>
    );
}
