<script lang="ts">
  import { ClipboardCopy, File, FileSignature, Table } from "lucide-svelte";
  import BentoGrid from "./lib/ui/BentoGrid/BentoGrid.svelte";
  import BentoGridItem from "./lib/ui/BentoGrid/BentoGridItem.svelte";
  const items = [
    {
      title: "The Dawn of Innovation",
      description: "Explore the birth of groundbreaking ideas and inventions.",
      className: "md:col-span-2",
      icon: ClipboardCopy,
    },
    {
      title: "The Digital Revolution",
      description: "Dive into the transformative power of technology.",
      className: "md:col-span-1",
      icon: File,
    },
    {
      title: "The Art of Design",
      description: "Discover the beauty of thoughtful and functional design.",
      className: "md:col-span-1",
      icon: FileSignature,
    },
    {
      title: "The Power of Communication",
      description:
        "Understand the impact of effective communication in our lives.",
      className: "md:col-span-2",
      icon: Table,
    },
  ];
</script>

<BentoGrid className="max-w-4xl mx-auto md:auto-rows-[20rem]">
  {#each items as item, i (i)}
    <BentoGridItem
      title={item.title}
      description={item.description}
      className={item.className}
    >
      <div
        slot="header"
        class="flex h-full min-h-[6rem] w-full flex-1 rounded-xl border border-transparent bg-neutral-100 bg-dot-black/[0.2] [mask-image:radial-gradient(ellipse_at_center,white,transparent)] dark:border-white/[0.2] dark:bg-black dark:bg-dot-white/[0.2]"
      ></div>
      <svelte:component
        this={item.icon}
        slot="icon"
        class="h-4 w-4 text-neutral-500"
      />
    </BentoGridItem>
  {/each}
</BentoGrid>
<h1 class="text-3xl font-bold underline bg-red-500">Hello world!</h1>

<style lang="postcss">
  :global(html) {
    background-color: #f5f5f5;
  }
</style>
