<script>
  import ChatItem from "./ChatItem.svelte";

  let items = [
    {
      title: "Brandon Kauffman",
      description: "Explore the birth of groundbreaking ideas and inventions.",
    },
    {
      title: "Preston Gibbs",
      description: "Dive into the transformative power of technology.",
    },
    {
      title:
        "Andrew Medford is Chatting in the thing here and it is going well",
      description: "Discover the beauty of thoughtful and functional design.",
    },
    {
      title: "The Power of Communication",
      description:
        "Understand the impact of effective communication in our lives.",
    },
  ];
</script>

<div
  class="h-screen bg-zinc-950 w-3/12 min-w-60 border-r border-teams-blue-200"
>
  {#each items as item}
    <ChatItem description={item.description} title={item.title} />
  {/each}
</div>
