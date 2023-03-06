import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import UnoCSS from "unocss/vite";
import { presetUno, presetWebFonts, transformerVariantGroup } from "unocss";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    UnoCSS({
      transformers: [transformerVariantGroup()],
      presets: [
        presetUno(),
        presetWebFonts({
          provider: "fontshare",
          fonts: {
            sans: { name: "Switzer", weights: [400, 500, 600, 700, 800] },
          },
        }),
      ],
    }),
  ],
});
