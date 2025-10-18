import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogProps, DialogTitle } from "src/components/base/dialog";
import { Button, PrimaryButton } from "src/components/base/button";
import React from "react";
import { CredentialResetSchema, SimpleAccountSchema } from "src/api/generated/admin";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { Field, FieldGroup, Fieldset, Label } from "src/components/base/fieldset";
import { Text } from "src/components/base/text";
import { Subheading } from "src/components/base/heading";

/**
 * Props for {@link AdminResetCredentialsDialog}
 */
export type AdminResetCredentialsDialogProps = DialogProps & {
    /** Account to reset */
    account: SimpleAccountSchema;
};

/**
 * Dialog to associate a domain
 */
export default function AdminResetCredentialsDialog(props: AdminResetCredentialsDialogProps) {
    const [t] = useTranslation("dialog-reset-credentials");
    const [tg] = useTranslation();

    const [reset, setReset] = React.useState<CredentialResetSchema>();

    const createReset = async () => {
        const credentialReset = await Api.admin.accounts.resetCredentials(props.account.uuid);
        setReset(credentialReset);
    };

    React.useEffect(() => {
        if (props.open) {
            createReset().then();
        }
    }, [props.open, props.account]);

    return (
        <Dialog open={props.open} onClose={props.onClose}>
            <DialogTitle>{t("heading.reset-credentials-for", { name: props.account.display_name })}</DialogTitle>
            <DialogBody>
                <div className={"grid grid-cols-[auto_1fr] gap-12"}>
                    <Text className={"!text-black dark:!text-white"}>{t("label.expires-at")}</Text>
                    <Text>{new Date(reset?.expires_at ?? "").toLocaleDateString("de")}</Text>
                </div>
            </DialogBody>
            <DialogActions>
                <Button plain={true} onClick={props.onClose}>
                    {tg("button.cancel")}
                </Button>
                <PrimaryButton
                    onClick={async () => {
                        reset && (await navigator.clipboard.writeText(reset.link));
                        toast.success(tg("toast.copied-to-clipboard"));
                        props.onClose();
                    }}
                >
                    {t("button.copy-link")}
                </PrimaryButton>
            </DialogActions>
        </Dialog>
    );
}
