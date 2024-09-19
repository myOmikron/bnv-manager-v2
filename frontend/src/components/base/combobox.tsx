import * as Headless from "@headlessui/react";
import React from "react";
import { Input } from "src/components/base/input";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import { AnimatePresence, motion } from "framer-motion";
import { clsx } from "clsx";

/**
 * The properties for a ComboboxOption
 */
export type ComboboxOptionProps<TType> = {
    /** The value this option represents */
    value: TType;
    /** The children of the option */
    children: React.ReactElement | Array<React.ReactElement>;
};

/**
 * The option for a combobox
 */
export function ComboboxOption<TType>(props: ComboboxOptionProps<TType>) {
    return (
        <Headless.ComboboxOption
            {...props}
            className={clsx([
                // Basic layout
                "relative block w-full appearance-none rounded-lg py-[calc(theme(spacing[2.5])-1px)] sm:py-[calc(theme(spacing[1.5])-1px)]",
                // Set minimum height for when no value is selected
                "min-h-11 sm:min-h-9",
                // Horizontal padding
                "pl-[calc(theme(spacing[3.5])-1px)] pr-[calc(theme(spacing.7)-1px)] sm:pl-[calc(theme(spacing.3)-1px)]",
                // Typography
                "text-left text-base/6 text-zinc-950 placeholder:text-zinc-500 sm:text-sm/6 dark:text-white forced-colors:text-[CanvasText]",
                // Background color
                "bg-transparent",
                // Invalid state
                "group-data-[invalid]:border-red-500 group-data-[invalid]:group-data-[hover]:border-red-500 group-data-[invalid]:dark:border-red-600 group-data-[invalid]:data-[hover]:dark:border-red-600",
                // Disabled state
                "group-data-[disabled]:border-zinc-950/20 group-data-[disabled]:opacity-100 group-data-[disabled]:dark:border-white/15 group-data-[disabled]:dark:bg-white/[2.5%] dark:data-[hover]:group-data-[disabled]:border-white/15",
                // Selected
                "data-[selected]:bg-blue-300 dark:data-[selected]:bg-blue-600",
                // Focus
                "data-[focus]:bg-blue-200 dark:data-[focus]:bg-blue-700",
            ])}
        />
    );
}

/**
 * The props for ComboboxOptions
 */
type ComboboxOptionsProps<TType> = {
    /** The options for the combobox options */
    children:
        | React.ReactElement<ComboboxOptionProps<TType>>
        | Array<React.ReactElement<ComboboxOptionProps<TType>>>;
};

/**
 * The options for a combobox
 */
function ComboboxOptions<TType>(props: ComboboxOptionsProps<TType>) {
    return (
        <Headless.ComboboxOptions
            data-control={"list"}
            anchor={"bottom start"}
            static={true}
            as={motion.div}
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            transition={{ duration: 0.1 }}
            className={clsx(
                // Anchor positioning
                "[--anchor-gap:theme(spacing.2)] [--anchor-padding:theme(spacing.1)] data-[anchor~=start]:[--anchor-offset:-6px] data-[anchor~=end]:[--anchor-offset:6px] sm:data-[anchor~=start]:[--anchor-offset:-4px] sm:data-[anchor~=end]:[--anchor-offset:4px]",
                // Base styles
                "w-fit rounded-xl p-1",
                // Invisible border that is only visible in `forced-colors` mode for accessibility purposes
                "outline outline-1 outline-transparent focus:outline-none",
                // Handle scrolling when menu won't fit in viewport
                "overflow-y-auto",
                // Popover background
                "bg-white/75 backdrop-blur-xl dark:bg-zinc-800/75",
                // Shadows
                "shadow-lg ring-1 ring-zinc-950/10 dark:ring-inset dark:ring-white/10",
            )}
            children={props.children}
        />
    );
}

/**
 * The properties for {@link Combobox}
 */
export type ComboboxProps<TType> = {
    /** The current value of the combobox */
    value: TType;
    /** The function that is called when a value is selected */
    onChange: (value: TType) => void;
    /** The function that should be called when closing the dropdown */
    onClose: () => void;
    /** The display function */
    queryDisplay: (type: TType) => string;
    /** Invoked when typing into the input field */
    onQueryChange: (query: string) => void;

    /** The options of the combobox */
    children:
        | React.ReactElement<ComboboxOptionProps<TType>>
        | Array<React.ReactElement<ComboboxOptionProps<TType>>>;
};

/**
 * A multiselect and searchable combobox
 */
export function Combobox<TType>(props: ComboboxProps<TType>) {
    const {
        children,
        value,
        onChange,
        onClose,
        queryDisplay,
        onQueryChange,
        ...other
    } = props;

    return (
        <Headless.Combobox
            value={value}
            onChange={onChange}
            onClose={onClose}
            {...other}
        >
            {({ open }) => (
                <>
                    <div data-slot={"control"} className={"relative my-3"}>
                        <Headless.ComboboxInput
                            as={Input}
                            autoComplete={"off"}
                            aria-autocomplete={"none"}
                            displayValue={queryDisplay}
                            onChange={(ev) => onQueryChange(ev.target.value)}
                        />
                        <Headless.ComboboxButton className="group absolute inset-y-0 right-0 px-2.5">
                            <ChevronDownIcon className="size-4 fill-zinc-950/60 group-data-[hover]:fill-zinc-950 dark:fill-white/60 dark:group-data-[hover]:fill-white" />
                        </Headless.ComboboxButton>
                    </div>

                    <AnimatePresence>
                        {open &&
                            (Array.isArray(children) &&
                            children.length === 0 ? undefined : (
                                <ComboboxOptions children={children} />
                            ))}
                    </AnimatePresence>
                </>
            )}
        </Headless.Combobox>
    );
}
