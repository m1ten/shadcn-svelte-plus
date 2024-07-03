<script lang="ts">
    import { Button as ButtonPrimitive } from "bits-ui";
    import { type Events, type Props, buttonVariants } from "./index.js";
    import { cn } from "$lib/utils.js";

    type $$Props = Props;
    type $$Events = Events;

    let className: $$Props["class"] = undefined;
    let variant: $$Props["variant"] = "default";
    let modifier: $$Props["modifier"] = "default";
    let size: $$Props["size"] = "default";
    let builders: $$Props["builders"] = [];
    let icon: $$Props["icon"] = undefined;
    let iconPosition: $$Props["iconPosition"] = "left";

    export {
        className as class,
        variant,
        modifier,
        size,
        builders,
        icon,
        iconPosition,
    };
</script>

<ButtonPrimitive.Root
    {builders}
    class={cn(buttonVariants({ variant, modifier, size, className }))}
    type="button"
    {...$$restProps}
    on:click
    on:keydown
>
    {#if iconPosition === "left"}
        <!-- Optional change group-hover:mr-2 to group-hover:pr-2 -->
        <div
            class="w-0 translate-x-[0%] pr-0 opacity-0 transition-all duration-200 group-hover:w-5 group-hover:translate-x-100 group-hover:mr-2 group-hover:opacity-100"
        >
            <svelte:component this={icon} class="h-[1.2rem] w-[1.2rem]" />
        </div>
    {/if}
    <slot />
    {#if iconPosition === "right"}
        <div
            class="w-0 translate-x-[100%] pl-0 opacity-0 transition-all duration-200 group-hover:w-5 group-hover:translate-x-0 group-hover:ml-2 group-hover:opacity-100"
        >
            <svelte:component this={icon} class="h-[1.2rem] w-[1.2rem]" />
        </div>
    {/if}
</ButtonPrimitive.Root>
