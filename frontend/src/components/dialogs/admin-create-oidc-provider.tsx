import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { Description, ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Button, PrimaryButton } from "src/components/base/button";
import { Api, UUID } from "src/api/api";
import { Input } from "src/components/base/input";
import { ClipboardDocumentListIcon } from "@heroicons/react/20/solid";
import { toast } from "react-toastify";

/**
 * The properties for {@link DialogCreateOidcProvider}
 */
export type DialogCreateOidcProviderProps = {
    /** Callback for close action */
    onClose: () => void;
    /** Callback for creation of the new oidc provider */
    onCreate: () => void;
};

/**
 * A dialog to create a new oidc provider
 */
export default function DialogCreateOidcProvider(props: DialogCreateOidcProviderProps) {
    const [t] = useTranslation("dialog-create-oidc-provider");
    const [tg] = useTranslation();

    const form = useForm({
        defaultValues: {
            name: "",
            redirectUrl: "",
        },
        validators: {
            // eslint-disable-next-line
            onSubmitAsync: async ({ value }) => {
                await Api.admin.oidcProvider.create({
                    name: value.name,
                    redirect_uri: value.redirectUrl,
                });

                props.onCreate();
            },
        },
    });

    return (
        <Dialog open={true} onClose={props.onClose}>
            <DialogTitle>{t("heading.create-oidc-provider")}</DialogTitle>
            <DialogBody>
                <Form onSubmit={form.handleSubmit}>
                    <Fieldset>
                        <FieldGroup>
                            <form.Field name={"name"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.name")}</RequiredLabel>
                                        <Input
                                            autoFocus={true}
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </form.Field>

                            <form.Field name={"redirectUrl"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.redirect-url")}</RequiredLabel>
                                        <Input
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                    </Field>
                                )}
                            </form.Field>

                            <DialogActions>
                                <Button onClick={props.onClose} plain={true}>
                                    {tg("button.cancel")}
                                </Button>
                                <PrimaryButton type={"submit"}>{t("button.create-oidc-provider")}</PrimaryButton>
                            </DialogActions>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </DialogBody>
        </Dialog>
    );
}
