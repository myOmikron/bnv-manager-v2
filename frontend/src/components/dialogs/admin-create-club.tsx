import React from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import { Button, PrimaryButton } from "src/components/base/button";
import { Api } from "src/api/api";

/**
 * The properties for {@link AdminCreateClubDialog}
 */
export type AdminCreateClubDialogProps = {
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

    const form = useForm({
        defaultValues: {
            name: "",
        },
        validators: {
            // eslint-disable-next-line
            onSubmitAsync: async ({ formApi, value }) => {
                const res = await Api.admin.clubs.create({ name: value.name });
                if (res.result === "Err") {
                    return {
                        fields: {
                            name: [res.error.name_already_occupied ? t("error.name-already-occupied") : undefined],
                        },
                    };
                }
            },
        },
        // eslint-disable-next-line
        onSubmit: () => {
            props.onCreate();
        },
    });

    return (
        <Dialog open={true} onClose={props.onClose}>
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

                            <DialogActions>
                                <Button plain={true} onClick={props.onClose}>
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
