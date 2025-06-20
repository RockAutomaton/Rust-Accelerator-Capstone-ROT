# Yew + Tailwind CSS v4 Setup Guide

This guide explains how to set up **Tailwind CSS v4** with a [Yew](https://yew.rs/) (Rust) project, using Trunk for building and serving. It covers the key steps, configuration, and commands needed for a modern, working integration.

---

## 1. Install Tailwind CSS

Install Tailwind CSS v4 and its CLI via npm:

```sh
npm install -D tailwindcss@latest @tailwindcss/cli@latest
```

---

## 2. Create Your Main CSS File

Create `src/style.css` with the following content (Tailwind v4 syntax):

```css
@import "tailwindcss";
@source "./src";
```
- `@import "tailwindcss";` loads Tailwind's utilities.
- `@source "./src";` tells Tailwind to scan all files in the `src/` directory (including `.rs` Yew files) for class usage.

---

## 3. Tailwind Config (Optional)

You only need `tailwind.config.js` if you want to customize the theme or add plugins. **You do NOT need a `content` array in v4.**

Example minimal config:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {},
  },
  plugins: [],
}
```

---

## 4. Build Tailwind CSS

Add a script to your `package.json`:

```json
"build-css": "npx @tailwindcss/cli -i ./src/style.css -o ./dist/style.css --watch"
```

Or run directly:

```sh
npx @tailwindcss/cli -i ./src/style.css -o ./dist/style.css --watch
```

---

## 5. Reference the CSS in Your HTML

In your `index.html` (used by Trunk), add:

```html
<link data-trunk rel="css" href="dist/style.css" />
```

---

## 6. Serve with Trunk

Run Trunk to build and serve your Yew app:

```sh
trunk serve
```

---

## 7. Development Workflow

- **Start the Tailwind watcher:**
  ```sh
  npm run build-css
  ```
- **In another terminal, start Trunk:**
  ```sh
  trunk serve
  ```
- Edit your Rust/Yew files and Tailwind classes will be picked up automatically!

---

## Notes on Tailwind v4
- **No `content` array needed** in `tailwind.config.js` (automatic source detection).
- Use `@import` and `@source` in your CSS instead of `@tailwind` directives.
- If you want to use a JS config, add `@config "./tailwind.config.js";` to your CSS.

---

## Troubleshooting
- If styles don't appear, make sure you:
  - Use the v4 syntax in `src/style.css`.
  - Rebuild your CSS after changes.
  - Reference the correct CSS file in your HTML.
- Check the generated CSS file for your classes (e.g., `.bg-indigo-800`).

---

**Enjoy building beautiful Yew apps with Tailwind CSS v4!** 