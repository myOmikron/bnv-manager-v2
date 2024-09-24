import React from "react";
import { useForm } from "@tanstack/react-form";
import { Field, FieldGroup, Fieldset, Label } from "src/components/base/fieldset";
import { useTranslation } from "react-i18next";
import { Input } from "src/components/base/input";
import Form from "src/components/base/form";
import { Button } from "src/components/base/button";
import { Api } from "src/api/api";
import { toast } from "react-toastify";

/**
 * The properties for {@link Login}
 */
export type LoginProps = {
    /** Callback to call when the user has successfully logged-in */
    onLogin: () => void;
};

/**
 * Login view
 */
export default function Login(props: LoginProps) {
    const [t] = useTranslation();
    const [tL] = useTranslation("login");

    const form = useForm({
        defaultValues: {
            username: "",
            password: "",
        },
        onSubmit: async ({ value }) => {
            const res = await Api.common.auth.login(value.username, value.password);

            res.match(
                () => {
                    toast.success(tL("toast.login-successful"));
                    props.onLogin();
                },
                (err) => toast.error(err.message),
            );
        },
    });

    return (
        <div className={"flex h-screen w-full items-center justify-center bg-zinc-50 p-3 dark:bg-neutral-950"}>
            <div className="w-full max-w-md flex-col rounded-xl border bg-white dark:border-zinc-800 dark:bg-zinc-900 dark:before:pointer-events-none forced-colors:outline">
                <div
                    className={
                        "grid h-full w-full flex-col justify-items-center overflow-hidden p-6 py-8 sm:p-8 lg:p-12"
                    }
                >
                    <span className={"text-3xl font-normal text-black dark:text-white"}>BNV Manager</span>
                    <Form
                        className={"mt-12 flex w-full max-w-sm flex-col overflow-hidden p-1"}
                        onSubmit={form.handleSubmit}
                    >
                        <Fieldset>
                            <FieldGroup>
                                <form.Field name={"username"}>
                                    {(fieldApi) => (
                                        <Field>
                                            <Label>{t("label.username")}</Label>
                                            <Input
                                                autoComplete={"username"}
                                                value={fieldApi.state.value}
                                                onChange={(e) => fieldApi.setValue(e.target.value)}
                                            />
                                        </Field>
                                    )}
                                </form.Field>

                                <form.Field name={"password"}>
                                    {(fieldApi) => (
                                        <Field>
                                            <Label>{t("label.password")}</Label>
                                            <Input
                                                autoComplete={"current-password"}
                                                value={fieldApi.state.value}
                                                onChange={(e) => fieldApi.setValue(e.target.value)}
                                                type={"password"}
                                            />
                                        </Field>
                                    )}
                                </form.Field>

                                <Button type={"submit"} color={"dark/white"} className={"w-full"}>
                                    {tL("button.sign-in")}
                                </Button>
                            </FieldGroup>
                        </Fieldset>
                    </Form>
                </div>
            </div>
        </div>
    );
}
