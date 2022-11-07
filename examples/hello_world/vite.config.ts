import { UserConfigExport } from "vite";
import vitePluginWASM from "vite-plugin-wasm";

export default {
	plugins: [vitePluginWASM()],
} as UserConfigExport;
