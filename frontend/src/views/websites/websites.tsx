import React, { useEffect } from "react";
import HeadingLayout from "../../components/heading-layout";
import { Button } from "../../components/button";
import { PlusIcon } from "@heroicons/react/20/solid";
import CreateWebsiteDialog from "./components/create-website-dialog";
import { SimpleWebsite } from "../../api/generated";
import { Api } from "../../api/api";
import { toast } from "react-toastify";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "../../components/table";
import DeployStateBadge from "./components/deploy-state-badge";

/**
 * The properties for {@link Websites}
 */
export type WebsitesProps = {};

/**
 * The configuration for the websites
 */
export default function Websites(props: WebsitesProps) {
    const [open, setOpen] = React.useState(false);
    const [websites, setWebsites] = React.useState<Array<SimpleWebsite>>([]);

    useEffect(() => {
        Api.websites.getAll().then((res) =>
            res.match(
                (websites) => {
                    setWebsites(() => websites.websites);
                },
                (err) => toast.error(err.message),
            ),
        );
    }, [open]);

    return (
        <>
            <CreateWebsiteDialog
                open={open}
                onClose={() => setOpen(false)}
                onSubmit={(name) => {
                    Api.websites.create(name).then((res) =>
                        res.match(
                            () => setOpen(false),
                            (err) => err.message,
                        ),
                    );
                }}
            />
            <HeadingLayout
                heading={"Website configuration"}
                disableBottomBorder={true}
                headingButtons={
                    <div className={"flex gap-3"}>
                        <Button type={"button"} onClick={() => setOpen(true)}>
                            <PlusIcon />
                            <span>Create</span>
                        </Button>
                    </div>
                }
            >
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableHeader>Name</TableHeader>
                            <TableHeader>Created at</TableHeader>
                            <TableHeader>Status</TableHeader>
                            <TableHeader>Last deployed</TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {websites.map((x) => (
                            <TableRow
                                key={x.uuid}
                                href={`/websites/${x.uuid}`}
                                className={"dark:text-white"}
                            >
                                <TableCell>{x.name}</TableCell>
                                <TableCell>
                                    {x.createdAt.toLocaleDateString()}
                                </TableCell>
                                <TableCell>
                                    <DeployStateBadge state={x.deployState} />
                                </TableCell>
                                <TableCell>
                                    {x.lastDeployment?.toLocaleString()}
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </HeadingLayout>
        </>
    );
}
