import React, { useEffect, useRef, useState } from "react";
import { useParams } from "react-router";
import { FullWebsite } from "../../../api/generated";
import { Api } from "../../../api/api";
import { toast } from "react-toastify";
import HeadingLayout from "../../../components/heading-layout";
import { ChevronLeftIcon } from "@heroicons/react/24/solid";
import { Button } from "../../../components/button";
import { Text } from "../../../components/text";
import { ROUTER } from "../../../router";
import { Subheading } from "../../../components/heading";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "../../../components/table";
import { XMarkIcon } from "@heroicons/react/20/solid";
import { Input } from "../../../components/input";
import { ErrorMessage } from "../../../components/fieldset";
import { Divider } from "../../../components/divider";

/**
 * The properties for {@link WebsiteConfiguration}
 */
export type WebsiteConfigurationProps = {};

/**
 * The configuration for a website
 */
export default function WebsiteConfiguration(props: WebsiteConfigurationProps) {
    const { websiteUuid } = useParams();

    const [website, setWebsite] = useState<FullWebsite>();

    const domainRef = useRef<HTMLInputElement>(null);
    const [newDomain, setNewDomain] = React.useState("");
    const [newDomainError, setNewDomainError] = React.useState(false);

    const refresh = () => {
        websiteUuid &&
            Api.websites.get(websiteUuid).then((res) =>
                res.match(
                    (website) => setWebsite(website),
                    (err) => toast.error(err.message),
                ),
            );
    };

    const addDomain = () => {
        if (!validateDomain(newDomain)) {
            setNewDomainError(true);
            toast.error("Domain appears to be invalid");
            domainRef.current && domainRef.current.focus();
            return;
        }
        setNewDomainError(false);
        website &&
            Api.websites.addDomain(website.uuid, newDomain).then((res) =>
                res.match(
                    () => {
                        refresh(), setNewDomain("");
                    },
                    (err) => toast.error(err.message),
                ),
            );
    };

    useEffect(() => {
        refresh();
    }, []);

    return (
        website && (
            <div className={"flex flex-col justify-start gap-6"}>
                <Button
                    plain={true}
                    className={"w-fit"}
                    href={ROUTER.WEBSITES.path}
                >
                    <ChevronLeftIcon />
                    <Text>Websites</Text>
                </Button>
                <HeadingLayout heading={`Edit Website ${website.name}`}>
                    <Table className={"text-zinc-800 dark:text-zinc-200"}>
                        <TableHead>
                            <TableRow>
                                <TableHeader>Domain</TableHeader>
                                <TableHeader className={"text-right"}>
                                    <span className={"sr-only"}>Action</span>
                                </TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {website.domains.map((x) => (
                                <TableRow key={x.uuid}>
                                    <TableCell>{x.domain}</TableCell>
                                    <TableCell className={"text-right"}>
                                        <Button
                                            plain={true}
                                            onClick={() => {
                                                websiteUuid &&
                                                    Api.websites
                                                        .removeDomain(
                                                            websiteUuid,
                                                            x.uuid,
                                                        )
                                                        .then((res) =>
                                                            res.match(
                                                                () => refresh(),
                                                                (err) =>
                                                                    toast.error(
                                                                        err.message,
                                                                    ),
                                                            ),
                                                        );
                                            }}
                                        >
                                            <XMarkIcon />
                                        </Button>
                                    </TableCell>
                                </TableRow>
                            ))}
                            <TableRow>
                                <TableCell colSpan={2}>
                                    <form
                                        method={"post"}
                                        onSubmit={(ev) => {
                                            ev.preventDefault();
                                            addDomain();
                                        }}
                                        className={"flex gap-3"}
                                    >
                                        <Input
                                            placeholder={"New domain"}
                                            value={newDomain}
                                            onChange={(ev) => {
                                                setNewDomain(
                                                    ev.currentTarget.value,
                                                );
                                            }}
                                            invalid={newDomainError}
                                            ref={domainRef}
                                        />
                                        <Button type={"submit"} plain={true}>
                                            Add
                                        </Button>
                                    </form>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                    <Divider />
                    <div className={"flex justify-end gap-6"}>
                        <Button color={"orange"}>Deploy configuration</Button>
                    </div>
                </HeadingLayout>
            </div>
        )
    );
}

const re = new RegExp(
    `^(?=.{1,253}\\.?$)(?:(?!-|[^.]+_)[A-Za-z0-9-_]{1,63}(?<!-)(?:\\.|$)){2,}$`,
);

/**
 * Tests whether an input string seems to be a valid domain
 *
 * @param domain The string to test
 *
 * @returns boolean whether the test passed
 */
function validateDomain(domain: string) {
    return re.test(domain);
}
