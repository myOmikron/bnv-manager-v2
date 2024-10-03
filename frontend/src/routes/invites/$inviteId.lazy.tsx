import { createLazyFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { FullUserInvite } from "src/api/generated";
import { Heading } from "src/components/base/heading";
import Form from "src/components/base/form";
import { ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Button } from "src/components/base/button";
import { useForm } from "@tanstack/react-form";
import { Input } from "src/components/base/input";
import { Code, Text } from "src/components/base/text";

/**
 * The properties for {@link Invite}
 */
export type InviteProps = {};

/**
 * Invite
 */
export default function Invite(props: InviteProps) {
    const [t] = useTranslation();
    const [tI] = useTranslation("invite");

    const { inviteId } = Route.useParams();
    const navigate = Route.useNavigate();

    const [invite, setInvite] = React.useState<FullUserInvite>();

    const acceptForm = useForm({
        defaultValues: {
            password: "",
            password2: "",
        },
        onSubmit: async ({ value }) => {
            const res = await Api.common.userInvites.acceptWithPw(inviteId, value.password);

            res.match(
                () => {
                    navigate({ to: "/" });
                },
                (err) => toast.error(err.message),
            );
        },
    });

    /**
     * Refresh the invite
     */
    const refreshInvite = async () => {
        const res = await Api.common.userInvites.get(inviteId);

        res.match(
            (x) => {
                setInvite(x);
            },
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        refreshInvite().then();
    }, [inviteId]);

    if (!invite) {
        return undefined;
    }

    return (
        <div className={"flex h-screen w-full items-center justify-center bg-zinc-50 p-3 dark:bg-neutral-950"}>
            <div className="w-full max-w-md flex-col rounded-xl border bg-white dark:border-zinc-800 dark:bg-zinc-900 dark:before:pointer-events-none forced-colors:outline">
                <div className={"flex h-full w-full flex-col gap-6 overflow-hidden p-6 py-8 sm:p-8 lg:p-12"}>
                    <h1 className={"text-center text-3xl font-normal text-black dark:text-white"}>BNV Manager</h1>

                    <Heading level={2}>{tI("heading.welcome", { user: invite.display_name })}</Heading>

                    <Form onSubmit={acceptForm.handleSubmit}>
                        <Fieldset>
                            <FieldGroup>
                                <div className={"flex flex-col gap-2"}>
                                    <Text>{tI("description.invite")}</Text>

                                    <Text>
                                        {tI("description.username")}&nbsp;
                                        <Code>{invite.username}</Code>
                                    </Text>
                                </div>

                                <acceptForm.Field name={"password"}>
                                    {(fieldApi) => (
                                        <Field>
                                            <RequiredLabel>{t("label.password")}</RequiredLabel>
                                            <Input
                                                type={"password"}
                                                value={fieldApi.state.value}
                                                onChange={(e) => fieldApi.handleChange(e.target.value)}
                                                onBlur={fieldApi.handleBlur}
                                                required={true}
                                                invalid={fieldApi.state.meta.errors.length > 0}
                                            />
                                            {fieldApi.state.meta.errors.map((err) => (
                                                <ErrorMessage>{err}</ErrorMessage>
                                            ))}
                                        </Field>
                                    )}
                                </acceptForm.Field>

                                <acceptForm.Field
                                    name={"password2"}
                                    validators={{
                                        onChangeListenTo: ["password"],
                                        onChange: ({ value, fieldApi }) => {
                                            if (value !== fieldApi.form.getFieldValue("password")) {
                                                return tI("errors.password-mismatch");
                                            }
                                            return undefined;
                                        },
                                    }}
                                >
                                    {(fieldApi) => (
                                        <Field>
                                            <RequiredLabel>{tI("label.repeat-password")}</RequiredLabel>
                                            <Input
                                                type={"password"}
                                                value={fieldApi.state.value}
                                                required={true}
                                                invalid={fieldApi.state.meta.errors.length > 0}
                                                onBlur={fieldApi.handleBlur}
                                                onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            />
                                            {fieldApi.state.meta.errors.map((err) => (
                                                <ErrorMessage>{err}</ErrorMessage>
                                            ))}
                                        </Field>
                                    )}
                                </acceptForm.Field>

                                <Button type={"submit"} color={"orange"}>
                                    {tI("button.set-password")}
                                </Button>
                            </FieldGroup>
                        </Fieldset>
                    </Form>
                </div>
            </div>
        </div>
    );
}

export const Route = createLazyFileRoute("/invites/$inviteId")({
    component: Invite,
});
