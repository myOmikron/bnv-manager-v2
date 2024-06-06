import React from "react";
import { toast } from "react-toastify";
import { Api } from "../../api/api";
import {
    Field,
    FieldGroup,
    Fieldset,
    Label,
    Legend,
} from "../../components/fieldset";
import { Input } from "../../components/input";
import { Button } from "../../components/button";
import Monkey from "./monkey-2-min.webp";
import { useTranslation } from "react-i18next";

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
    const [t] = useTranslation("login");
    const [username, setUsername] = React.useState<string>("");
    const [password, setPassword] = React.useState<string>("");

    const performLogin = () => {
        Api.ldap.login(username, password).then((res) =>
            res.match(
                () => {
                    toast.success(t("Signed in"));
                    onLogin();
                },
                (err) => toast.error(err.message),
            ),
        );
    };

    return (
        <div
            className={
                "flex h-screen w-full items-center justify-center bg-zinc-50 p-3 dark:bg-neutral-950"
            }
        >
            <div className="w-full max-w-2xl rounded-xl border bg-white dark:border-zinc-800 dark:bg-zinc-900 dark:before:pointer-events-none forced-colors:outline">
                <div
                    className={
                        "grid h-full w-full place-items-start justify-items-center overflow-hidden p-6 py-8 sm:p-8 lg:p-12"
                    }
                >
                    <form
                        method="post"
                        className={
                            "grid w-full max-w-2xl grid-cols-1 items-center justify-items-center gap-12 space-y-8 md:grid-cols-2"
                        }
                        onSubmit={(e) => {
                            e.preventDefault();
                            performLogin();
                        }}
                    >
                        <img
                            src={Monkey}
                            alt={"Symbol"}
                            className={"max-h-96"}
                        />
                        <Fieldset className={"w-full"}>
                            <Legend>{t("Sign in")}</Legend>
                            <FieldGroup>
                                <Field>
                                    <Label>{t("Username")}</Label>
                                    <Input
                                        required={true}
                                        type={"username"}
                                        value={username}
                                        onChange={(e) =>
                                            setUsername(e.target.value)
                                        }
                                    />
                                </Field>
                                <Field>
                                    <Label>{t("Password")}</Label>
                                    <Input
                                        required={true}
                                        type={"password"}
                                        value={password}
                                        onChange={(e) =>
                                            setPassword(e.target.value)
                                        }
                                    />
                                </Field>
                                <Button
                                    className={"w-full"}
                                    type={"submit"}
                                    color={"orange"}
                                >
                                    {t("Sign in")}
                                </Button>
                            </FieldGroup>
                        </Fieldset>
                    </form>
                </div>
            </div>
        </div>
    );
}
