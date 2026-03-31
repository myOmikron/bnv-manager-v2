import React from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogProps, DialogTitle } from "src/components/base/dialog";
import { Button, PrimaryButton } from "src/components/base/button";
import { useForm } from "@tanstack/react-form";
import Form from "src/components/base/form";
import { ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import { Select } from "src/components/base/select";
import { DomainOptionSchema, MemberAliasApi } from "src/api/api_aliases";

/**
 * The properties for {@link ProposeAliasDialog}
 */
export type ProposeAliasDialogProps = DialogProps & {
    /** Available domains for the alias */
    domains: DomainOptionSchema[];
    /** Callback after successful proposal */
    onCreate: () => void;
};

/**
 * Dialog for proposing a new mail alias
 */
export default function ProposeAliasDialog(props: ProposeAliasDialogProps) {
    const [t] = useTranslation("dialog-propose-alias");
    const [tg] = useTranslation();

    const form = useForm({
        defaultValues: {
            localPart: "",
            domainUuid: props.domains[0]?.uuid ?? "",
        },
        validators: {
            onSubmitAsync: async ({ value }) => {
                const res = await MemberAliasApi.propose({
                    local_part: value.localPart,
                    domain_uuid: value.domainUuid,
                });

                if (res.result === "Err") {
                    return {
                        fields: {
                            localPart: res.error.invalid_local_part
                                ? t("error.invalid-local-part")
                                : res.error.alias_already_taken
                                  ? t("error.alias-already-taken")
                                  : res.error.domain_not_in_club
                                    ? t("error.domain-not-in-club")
                                    : null,
                        },
                    };
                }

                props.onCreate();
            },
        },
    });

    React.useEffect(() => {
        if (props.open) {
            form.reset();
        }
    }, [props.open]);

    const selectedDomain = props.domains.find((d) => d.uuid === form.getFieldValue("domainUuid"));

    return (
        <Dialog open={props.open} onClose={props.onClose}>
            <DialogTitle>{t("heading.propose-alias")}</DialogTitle>
            <DialogBody>
                <Form onSubmit={form.handleSubmit}>
                    <Fieldset>
                        <FieldGroup>
                            <form.Field name={"localPart"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.local-part")}</RequiredLabel>
                                        <div data-slot={"control"} className={"flex items-center gap-1"}>
                                            <Input
                                                autoFocus={true}
                                                required={true}
                                                value={fieldApi.state.value}
                                                onChange={(e) => fieldApi.handleChange(e.target.value)}
                                                invalid={fieldApi.state.meta.errors.length > 0}
                                            />
                                            <span className={"shrink-0 text-sm text-zinc-500 dark:text-zinc-400"}>
                                                @{selectedDomain?.domain ?? ""}
                                            </span>
                                        </div>
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage key={String(err)}>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </form.Field>

                            {props.domains.length > 1 && (
                                <form.Field name={"domainUuid"}>
                                    {(fieldApi) => (
                                        <Field>
                                            <RequiredLabel>{t("label.domain")}</RequiredLabel>
                                            <Select
                                                value={fieldApi.state.value}
                                                onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            >
                                                {props.domains.map((d) => (
                                                    <option key={d.uuid} value={d.uuid}>
                                                        {d.domain}
                                                    </option>
                                                ))}
                                            </Select>
                                        </Field>
                                    )}
                                </form.Field>
                            )}

                            <DialogActions>
                                <Button onClick={props.onClose} plain={true}>
                                    {tg("button.cancel")}
                                </Button>
                                <PrimaryButton type={"submit"}>{t("button.propose")}</PrimaryButton>
                            </DialogActions>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </DialogBody>
        </Dialog>
    );
}
