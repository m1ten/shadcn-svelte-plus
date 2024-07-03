<script lang="ts">
    import { Button } from "@/components/ui/button";
    import { ArrowLeft, ArrowRight } from "lucide-svelte";
</script>

<div class="flex flex-col items-center justify-center h-screen space-y-4">
    <Button
        variant="secondary"
        modifier="expandIcon"
        icon={ArrowLeft}
        iconPosition="left"
        >l-test
    </Button>
    <Button
        variant="link"
        modifier="shine"
        icon={ArrowRight}
        iconPosition="right"
        >r-test
    </Button>
</div>
