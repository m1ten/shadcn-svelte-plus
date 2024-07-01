# shadcn-svelte-plus

A collection of enhanced components for shadcn-svelte.

[Prerequisites](#prerequisites) | [Installation](#installation) | [Special Thanks](#special-thanks) | [License](#license)

<h2 id="prerequisites" style="font-size: 1.4rem;">Prerequisites</h2>

- [SvelteKit](https://kit.svelte.dev/)
- [Shadcn-svelte](https://www.shadcn-svelte.com/)
  - With the following components:
    - [Button](https://www.shadcn-svelte.com/docs/components/button)
- [Tailwind CSS](https://tailwindcss.com/)
- [Lucide Icons](https://lucide.dev/guide/packages/lucide-svelte)

<h2 id="installation" style="font-size: 1.4rem;">Installation</h2>

- Copy over the components you need from the [`shared`](shared) directory to your project.
- Add the following to your `tailwind.config.ts`:

```diff
const config = {
    extend: {
        keyframes: {
+          "shine": {
+            from: { backgroundPosition: '200% 0' },
+            to: { backgroundPosition: '-200% 0' },
+           },
        },
        animation: {
+         "shine": "shine 8s ease-in-out infinite",
        },
    },
}
```

<h2 id="special-thanks" style="font-size: 1.4rem;">Special Thanks</h2>

[Shadcn](https://www.shadcn.com/) for the design and
[Shadcn-svelte](https://www.shadcn-svelte.com/) for the components.

Enhanced Button component forked from [jakobhoeg/enhanced-button](https://githuv.com/jakobhoeg/enhanced-button).

<h2 id="license" style="font-size: 1.4rem;">License</h2>

This is purely for personal use, however, licensed under [MIT](LICENSE).
