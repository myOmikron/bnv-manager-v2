import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import { Button, PrimaryButton } from "src/components/base/button";
import { Api } from "src/api/api";
import { Domain } from "src/api/generated/admin";
import { Combobox, ComboboxLabel, ComboboxOption } from "src/components/base/combobox";

/**
 * The properties for {@link AdminCreateClubDialog}
 */
export type AdminCreateClubDialogProps = {
    /** Whether the dialog is open */
    open: boolean;
    /** What to do when the dialog is closed */
    onClose: () => void;
    /** What to do on creation of the new club */
    onCreate: () => void;
};

/**
 * Dialog for creating a new club
 */
export default function AdminCreateClubDialog(props: AdminCreateClubDialogProps) {
    const [t] = useTranslation("dialog-create-club");
    const [tg] = useTranslation();

    const [domains, setDomains] = React.useState<Array<Domain>>([]);

    const form = useForm({
        defaultValues: {
            name: "",
            primaryDomain: null as Domain | null | undefined,
        },
        validators: {
            onSubmitAsync: async ({ value }) => {
                if (!value.primaryDomain) {
                    return { fields: { primaryDomain: t("error.missing-value") } };
                }

                const res = await Api.admin.clubs.create({
                    name: value.name,
                    primary_domain: value.primaryDomain.uuid,
                });
                if (res.result === "Err") {
                    return {
                        fields: {
                            name: res.error.name_already_exists ? t("error.name-already-occupied") : undefined,
                            primaryDomain: res.error.domain_already_exists
                                ? t("error.domain-already-occupied")
                                : undefined,
                        },
                    };
                }
            },
        },
        onSubmit: () => {
            props.onCreate();
            form.reset();
        },
    });

    const retrieveUnassociatedDomains = async () => {
        const domains = await Api.admin.domains.unassociated();
        setDomains(domains);
    };

    useEffect(() => {
        retrieveUnassociatedDomains().then();
    }, [props.open]);

    return (
        <Dialog
            open={props.open}
            onClose={() => {
                props.onClose();
                form.reset();
            }}
        >
            <DialogTitle>{t("heading.create-club")}</DialogTitle>
            <DialogBody>
                <Form onSubmit={form.handleSubmit}>
                    <Fieldset>
                        <FieldGroup>
                            <form.Field name={"name"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.club-name")}</RequiredLabel>
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

                            <form.Field name={"primaryDomain"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.primary-domain")}</RequiredLabel>
                                        <Combobox
                                            options={domains}
                                            displayValue={(d) => (d ? d.domain : "")}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                            value={fieldApi.state.value}
                                            onChange={(d) => fieldApi.handleChange(d)}
                                        >
                                            {(domain) => (
                                                <ComboboxOption value={domain}>
                                                    <ComboboxLabel>{domain.domain}</ComboboxLabel>
                                                </ComboboxOption>
                                            )}
                                        </Combobox>
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </form.Field>

                            <DialogActions>
                                <Button
                                    plain={true}
                                    onClick={() => {
                                        props.onClose();
                                        form.reset();
                                    }}
                                >
                                    {tg("button.cancel")}
                                </Button>
                                <PrimaryButton type={"submit"}>{t("button.create-club")}</PrimaryButton>
                            </DialogActions>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </DialogBody>
        </Dialog>
    );
}
