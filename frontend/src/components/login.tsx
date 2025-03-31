import { useTranslation } from "react-i18next";
import React from "react";
import { Field, FieldGroup, Fieldset, Label } from "src/components/base/fieldset";
import { useForm } from "@tanstack/react-form";
import { Input } from "src/components/base/input";
import { PrimaryButton } from "src/components/base/button";
import Form from "src/components/base/form";
import { Heading } from "src/components/base/heading";
import { Api } from "src/api/api";
import { Text } from "src/components/base/text";

/**
 * The properties for {@link Login}
 */
export type LoginProps = {
    /** The function that should be executed on a successful login */
    onLogin: () => void;
};

/**
 * The login.json component
 */
export default function Login(props: LoginProps) {
    const [t] = useTranslation("login");

    const form = useForm({
        defaultValues: {
            username: "",
            password: "",
        },
        validators: {
            // eslint-disable-next-line
            onSubmitAsync: async ({ formApi, value }) => {
                const res = await Api.auth.login(value.username, value.password);

                if (res.result === "Err") {
                    return {
                        form: t("error.invalid-username-or-password"),
                    };
                }

                props.onLogin();
            },
        },
    });

    return (
        <div className={"flex h-screen w-full items-center justify-center bg-zinc-50 p-3 dark:bg-neutral-950"}>
            <div className="w-full max-w-md rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900 dark:before:pointer-events-none forced-colors:outline">
                <Form className={"p-12"} onSubmit={form.handleSubmit}>
                    <Fieldset className={"w-full"}>
                        <FieldGroup>
                            <Heading>BNV Manager</Heading>

                            <form.Field name={"username"}>
                                {(fieldApi) => (
                                    <Field>
                                        <Label>{t("label.username")}</Label>
                                        <Input
                                            autoFocus={true}
                                            autoComplete={"username"}
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                        />
                                    </Field>
                                )}
                            </form.Field>

                            <form.Field name={"password"}>
                                {(fieldApi) => (
                                    <Field>
                                        <Label>{t("label.password")}</Label>
                                        <Input
                                            required={true}
                                            type={"password"}
                                            autoComplete={"current-password"}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                        />
                                    </Field>
                                )}
                            </form.Field>

                            <form.Subscribe selector={(state) => [state.errorMap]}>
                                {([errorMap]) =>
                                    errorMap.onSubmit ? (
                                        <Text
                                            className={
                                                "!data-disabled:opacity-50 !dark:text-red-500 !text-base/6 !text-red-600 sm:!text-sm/6"
                                            }
                                        >
                                            {errorMap.onSubmit.form}
                                        </Text>
                                    ) : null
                                }
                            </form.Subscribe>

                            <PrimaryButton type={"submit"} className={"w-full"}>
                                {t("button.sign-in")}
                            </PrimaryButton>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </div>
        </div>
    );
}
