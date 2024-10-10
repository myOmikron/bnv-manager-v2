import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import HeadingLayout from "src/components/base/heading-layout";
import { useTranslation } from "react-i18next";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { Description, ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Button } from "src/components/base/button";
import { Input } from "src/components/base/input";

/**
 * The properties for {@link ClubCreate}
 */
export type ClubCreateProps = {};

/**
 * The creation of a club
 */
export default function ClubCreate(props: ClubCreateProps) {
    const [t] = useTranslation();
    const [tC] = useTranslation("admin-club-create");

    const navigate = Route.useNavigate();

    const createForm = useForm({
        defaultValues: {
            name: "",
            domain: "",
        },
        onSubmit: async ({ value, formApi }) => {
            const res = await Api.admin.clubs.create({ name: value.name, domain: value.domain });

            res.match(
                (res) => {
                    if (res.result === "Ok") {
                        navigate({ to: "/a/clubs/$clubId/general", params: { clubId: res.value.uuid } });
                    } else {
                        if (res.error.name_in_use) {
                            formApi.setFieldMeta("name", (meta) => {
                                meta.errors.push(tC("errors.name-in-use"));
                                return { ...meta };
                            });
                        }
                    }
                },
                (err) => toast.error(err.message),
            );
        },
    });

    return (
        <>
            <BackButton href={"/a/dashboard"}>
                <Text>{t("button.back")}</Text>
            </BackButton>
            <HeadingLayout className={"mt-6"} heading={tC("heading.create-club")}>
                <Form onSubmit={createForm.handleSubmit} className={"max-w-sm"}>
                    <Fieldset>
                        <FieldGroup>
                            <createForm.Field name={"name"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{tC("label.club-name")}</RequiredLabel>
                                        <Description>{tC("description.club-name")}</Description>
                                        <Input
                                            autoFocus={true}
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.setValue(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                        {fieldApi.state.meta.errors.map((x) => (
                                            <ErrorMessage>{x}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </createForm.Field>

                            <createForm.Field name={"domain"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{tC("label.domain")}</RequiredLabel>
                                        <Description>{tC("description.domain")}</Description>
                                        <Input
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.setValue(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                        {fieldApi.state.meta.errors.map((x) => (
                                            <ErrorMessage>{x}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </createForm.Field>

                            <Button type={"submit"}>{t("button.create")}</Button>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </HeadingLayout>
        </>
    );
}

export const Route = createFileRoute("/_admin/a/clubs/create")({
    component: ClubCreate,
});
