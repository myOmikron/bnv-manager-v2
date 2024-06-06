import React from "react";
import HeadingLayout from "../../../components/heading-layout";
import {
    Field,
    FieldGroup,
    Fieldset,
    Label,
} from "../../../components/fieldset";
import { Input } from "../../../components/input";
import { Button } from "../../../components/button";
import { Dialog } from "../../../components/dialog";

/**
 * The properties for {@link CreateWebsiteDialog}
 */
export type CreateWebsiteDialogProps = {
    open: boolean;
    onClose: () => void;
    onSubmit: (name: string) => void;
};

/**
 * The dialog to create a website
 */
export default function CreateWebsiteDialog(props: CreateWebsiteDialogProps) {
    const [name, setName] = React.useState("");
    return (
        <Dialog open={props.open} onClose={props.onClose}>
            <HeadingLayout heading={"Create Website"}>
                <form
                    method={"post"}
                    onSubmit={(e) => {
                        e.preventDefault();
                        props.onSubmit(name);
                    }}
                >
                    <Fieldset>
                        <FieldGroup>
                            <Field>
                                <Label>Name</Label>
                                <Input
                                    value={name}
                                    onChange={(e) => setName(e.target.value)}
                                    required={true}
                                    autoFocus={true}
                                />
                            </Field>
                            <Button
                                type={"submit"}
                                color={"orange"}
                                className={"w-full"}
                            >
                                Create
                            </Button>
                        </FieldGroup>
                    </Fieldset>
                </form>
            </HeadingLayout>
        </Dialog>
    );
}
