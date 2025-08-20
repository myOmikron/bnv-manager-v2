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
import { AuthLayout } from "src/components/base/auth-layout";
import { toast } from "react-toastify";

/**
 * The properties for {@link Login}
 */
type LoginProps = {
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
                const id = toast.loading(t("toast.signing-in"));
                try {
                    await Api.common.auth.login(value.username, value.password);
                } catch (e) {
                    toast.update(id, {
                        isLoading: false,
                        render: t("toast.sign-in-failed"),
                        type: "error",
                        autoClose: 3500,
                    });
                    return {
                        form: t("error.invalid-username-or-password"),
                    };
                }

                toast.update(id, {
                    isLoading: false,
                    render: t("toast.signed-in"),
                    type: "success",
                    autoClose: 3500,
                });

                props.onLogin();
            },
        },
    });

    return (
        <AuthLayout>
            <Form onSubmit={form.handleSubmit} className={"grid w-full max-w-sm grid-cols-1 gap-8"}>
                <Fieldset className={"w-full"}>
                    <FieldGroup>
                        <Heading>BNV Manager</Heading>

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
                                        invalid={fieldApi.form.state.errors.length > 0}
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
                                        invalid={fieldApi.form.state.errors.length > 0}
                                    />
                                </Field>
                            )}
                        </form.Field>

                        <PrimaryButton type={"submit"} className={"w-full"}>
                            {t("button.sign-in")}
                        </PrimaryButton>
                    </FieldGroup>
                </Fieldset>
            </Form>
        </AuthLayout>
    );
}
